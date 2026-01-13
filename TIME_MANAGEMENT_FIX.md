# Time Management Fix Summary

## Issues Found

### 1. **Hardcoded 10-Second Timeout (Critical)**
**Location:** `src/search.rs:115`

**Problem:** When `use_time_management` was `false`, the `get_allowed_time()` function returned a hardcoded `10000` milliseconds (10 seconds). This meant that even depth-based searches would always timeout at 10 seconds instead of completing the requested depth.

**Fix:** Changed to return `u128::MAX` when time management is disabled, allowing depth-based searches to complete without artificial time limits.

### 2. **Incorrect Variable Assignment (Critical)**
**Location:** `src/uci.rs:262`

**Problem:** When parsing the `binc` (black increment) parameter, the code was incorrectly setting `self.engine.wtime` instead of `self.engine.binc`. This would cause black's time management to be completely broken.

**Fix:** Changed to correctly assign to `self.engine.binc`.

### 3. **Aggressive Time Allocation**
**Location:** `src/search.rs:113`

**Problem:** The time allocation formula `(time_left / 30 + increment - 2 * move_overhead)` was too aggressive:
- Dividing by 30 means using 1/30th of remaining time per move
- Could result in negative values if `move_overhead` is large
- Didn't account for the fact that games can last many moves

**Fix:** Implemented a more conservative formula:
```rust
let base_time = time_left / 20;  // Use 1/20th instead of 1/30th
let inc_bonus = (increment * 3) / 4;  // Use 75% of increment
let overhead_cost = 2 * self.move_overhead;

if base_time + inc_bonus > overhead_cost {
    return base_time + inc_bonus - overhead_cost;
} else {
    // Emergency: use at least 1% of remaining time
    return std::cmp::max(time_left / 100, 50);
}
```

### 4. **Movetime Safety**
**Location:** `src/search.rs:102`

**Problem:** When using `movetime`, the code didn't check if `movetime` was large enough to subtract the overhead.

**Fix:** Added safety check:
```rust
let safe_movetime = if self.movetime > 2 * self.move_overhead {
    self.movetime - 2 * self.move_overhead
} else {
    self.movetime / 2
};
```

## Testing Recommendations

1. **Test depth-based search:**
   ```
   position startpos
   go depth 6
   ```
   Should now complete the full depth 6 search instead of timing out at 10 seconds.

2. **Test time-based search:**
   ```
   position startpos
   go wtime 60000 btime 60000 winc 1000 binc 1000
   ```
   Should now use approximately 3-4 seconds per move (60000/20 + 750).

3. **Test movetime:**
   ```
   position startpos
   go movetime 5000
   ```
   Should use close to 5 seconds (minus overhead).

## Time Allocation Strategy

The new formula aims for approximately 20 moves with the remaining time:
- Base time: `time_left / 20` (5% of remaining time)
- Increment bonus: `75% of increment` (saving 25% for emergencies)
- Overhead: Subtracts `2 * move_overhead` to account for communication delays
- Emergency fallback: Uses at least 1% of remaining time or 50ms minimum

This should prevent timeouts while still playing at a reasonable pace.
