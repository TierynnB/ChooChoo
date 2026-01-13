use crate::board::Board;
use crate::constants::*;
use crate::conversion;
use crate::evaluate;
use crate::evaluate::evaluate;
use crate::movegen;
use crate::moves::*;
use std::time::Instant;

#[derive(Clone, Copy, PartialEq)]
pub enum TTFlag {
    Exact,
    LowerBound,
    UpperBound,
}

#[derive(Clone, Copy)]
pub struct TranspositionTableEntry {
    pub position_hash: u64,
    pub depth: i8,
    pub score: i32,
    pub flag: TTFlag,
    pub best_move: Option<Move>,
}
pub struct MoveNode {
    pub move_notation: String,
    pub nodes: i128,
}
pub struct BestMoves {
    pub best_move: Move,
    pub best_score: i32,
}
pub struct SearchEngine {
    pub nodes: i128,
    pub start: Instant,
    pub move_nodes: Vec<MoveNode>,
    pub depth: i8,
    pub current_depth: i8,
    pub wtime: u128,
    pub movetime: u128,
    pub btime: u128,
    pub winc: u128,
    pub binc: u128,
    pub use_time_management: bool,
    pub searching_side: i8,
    pub move_overhead: u128,
    pub transposition_table: Vec<TranspositionTableEntry>,
    pub max_size_move_nodes: usize,
    pub abort_search: bool,
    pub time_limit: u128,
    pub movestogo: i8,
}

pub fn order_moves(moves: &mut Vec<Move>, best_move: Option<Move>) {
    for i in 0..moves.len() {
        let move_to_score = moves.get_mut(i).unwrap();
        
        // Prioritize the best move from TT
        if let Some(bm) = &best_move {
            if move_to_score == bm {
                move_to_score.sort_score = 1000000;
                continue;
            }
        }

        let value = MVV_LVA[move_to_score.to_piece as usize][move_to_score.from_piece as usize];
        move_to_score.sort_score += value as i32;
    }

    moves.sort_by(|a, b| b.sort_score.cmp(&a.sort_score));
}
impl SearchEngine {
    pub fn new() -> Self {
        SearchEngine {
            nodes: 0,
            start: Instant::now(),
            move_nodes: Vec::new(),
            depth: 3,
            current_depth: 1,
            winc: 0,
            movetime: 0,
            move_overhead: 10,
            wtime: 0,
            binc: 0,
            btime: 0,
            use_time_management: false,
            searching_side: WHITE,
            transposition_table: Vec::new(),
            max_size_move_nodes: 0,
            abort_search: false,
            time_limit: u128::MAX,
            movestogo: 30, // Default to 30 moves left
        }
    }
    pub fn set_depth(&mut self, depth: i8) {
        self.depth = depth;
    }
    fn clear_tt(&mut self) {
        self.transposition_table = Vec::new();
    }
    fn reset_search_state(&mut self) {
        self.nodes = 0;
        self.abort_search = false;
        self.start = Instant::now();
    }
    pub fn get_allowed_time(&self, side: i8) -> u128 {
        if self.use_time_management {
            if self.movetime > 0 {
                // Use movetime but ensure we leave some overhead
                let safe_movetime = if self.movetime > 2 * self.move_overhead {
                    self.movetime - 2 * self.move_overhead
                } else {
                    self.movetime / 2
                };
                return safe_movetime;
            }

            let time_left = if side == WHITE {
                self.wtime
            } else {
                self.btime
            };

            let increment = if side == WHITE { self.winc } else { self.binc };
            
            // Use movestogo if provided, otherwise default to 40 moves
            let moves_to_plan_for = if self.movestogo > 0 { self.movestogo as u128 } else { 40 };
            
            let base_time = time_left / moves_to_plan_for;
            let inc_bonus = (increment * 3) / 4;
            let overhead_cost = 2 * self.move_overhead;
            
            // Ensure we don't go negative
            if base_time + inc_bonus > overhead_cost {
                return base_time + inc_bonus - overhead_cost;
            } else {
                // Emergency: use at least 1% of remaining time
                return std::cmp::max(time_left / 100, 50);
            }
        } else {
            // When time management is disabled, return a very large value
            // This allows depth-based search to complete
            return u128::MAX;
        }
    }
    fn store_tt(
        &mut self,
        position_hash: u64,
        depth: i8,
        score: i32,
        flag: TTFlag,
        best_move: Option<Move>,
    ) {
         // Simple replacement scheme or stick depth
        if let Some(existing) = self.transposition_table.iter_mut().find(|e| e.position_hash == position_hash) {
             if existing.depth <= depth {
                  existing.depth = depth;
                  existing.score = score;
                  existing.flag = flag;
                  existing.best_move = best_move;
             }
             return;
        }

        self.transposition_table.push(TranspositionTableEntry {
            position_hash,
            depth,
            score,
            flag,
            best_move,
        });
    }

    fn probe_tt(&self, position_hash: u64) -> Option<&TranspositionTableEntry> {
        self.transposition_table.iter().find(|e| e.position_hash == position_hash)
    }

    pub fn quiescence_search(
        &mut self,
        board: &mut Board,
        mut alpha: i32,
        beta: i32,
    ) -> i32 {
        self.nodes += 1;
        
        let stand_pat = evaluate(board);
        
        if stand_pat >= beta {
            return beta;
        }
        
        // Delta pruning: if stand_pat + BIG VALUE < alpha, we can probably quit,
        // unless we are in endgame or similar. 
        // For safety, let's use a large delta (Queen val is ~900).
        const DELTA: i32 = 975; 
        if stand_pat < alpha - DELTA {
             // We can only prune if we don't have powerful captures (promotions).
             // But for safety in simple engine, maybe skip delta pruning or be very conservative.
             // return alpha; 
        }

        if alpha < stand_pat {
            alpha = stand_pat;
        }

        let mut moves_for_current_depth =
            movegen::generate_pseudo_legal_moves(board, board.side_to_move, false, true);

        order_moves(&mut moves_for_current_depth, None);

        for generated_move in moves_for_current_depth.iter() {
            if self.use_time_management {
                if (self.nodes % 2048) == 0 && self.start.elapsed().as_millis() > self.time_limit {
                     self.abort_search = true;
                }
                if self.abort_search {
                     break;
                }
            }

            // Delta Pruning check: if move captures, stand_pat + captured_piece + margin < alpha?
            // This is safer delta pruning.
             let captured_val = evaluate::get_piece_value(generated_move.to_piece);
             if stand_pat + captured_val + 200 < alpha && generated_move.promotion_to.is_none() {
                 continue; 
             }

            board.make_move(generated_move);
            // QSearch has no depth limit, but naturally terminates as captures run out.
            let score = -self.quiescence_search(board, -beta, -alpha);
            board.un_make_move(generated_move);

            if score >= beta {
                return beta;
            }

            if score > alpha {
                alpha = score;
            }
        }

        return alpha;
    }

    pub fn alpha_beta(&mut self, board: &mut Board, depth: i8, mut alpha: i32, beta: i32) -> i32 {
        if self.use_time_management {
             if (self.nodes % 2048) == 0 && self.start.elapsed().as_millis() > self.time_limit {
                 self.abort_search = true;
             }
             if self.abort_search {
                 return 0;
             }
        }
        
        let alpha_orig = alpha;
        let position_hash = conversion::hash_board_state_for_tt(board);
        let mut best_move: Option<Move> = None;

        // TT Probe
        if let Some(entry) = self.probe_tt(position_hash) {
            if entry.depth >= depth {
                match entry.flag {
                    TTFlag::Exact => return entry.score,
                    TTFlag::LowerBound => alpha = std::cmp::max(alpha, entry.score),
                    TTFlag::UpperBound => {
                         // beta = std::cmp::min(beta, entry.score); // standard upper bound logic
                         // For now, let's trust it only if it causes a cutoff
                         if entry.score <= alpha { return entry.score; } // this is fail-low?
                    } 
                }
                // If bounds crossed
                if alpha >= beta {
                    return entry.score;
                }
            }
            best_move = entry.best_move.clone();
        }

        if depth <= 0 {
            return self.quiescence_search(board, alpha, beta);
        };

        self.nodes += 1;

        let mut moves_for_current_depth =
            movegen::generate_pseudo_legal_moves(board, board.side_to_move, false, false);
        
        // Pass TT move to order_moves
        order_moves(&mut moves_for_current_depth, best_move.clone());

        let mut best_value = i32::MIN + 1; // +1 to avoid overflow when negating?
        let mut best_move_found: Option<Move> = None;
        
        let mut legal_moves = 0;

        for generated_move in moves_for_current_depth.iter() {
            // make_move returns void, doesn't check legality fully (e.g. self check)
            // so we assume pseudo-legal, but need to check after make_move if king is safe?
            // Existing logic checked `illegal_move` flag or unmade. 
            // The existing `search` loop handled this. `alpha_beta` assumed valid?
            // We should check validity here to be safe, OR `generate_pseudo_legal_moves` is trusted if we filter self-check.
            
            board.make_move(generated_move);
            
            // Check legality (self-check)
            // Note: `is_in_check` checks if `side_to_check` is under attack. 
            // After `make_move`, `board.side_to_move` is flipped.
            // We need to check if the side that JUST moved is in check.
            let side_just_moved = if board.side_to_move == WHITE { BLACK } else { WHITE };
            if evaluate::is_in_check(board, side_just_moved, None) {
                 board.un_make_move(generated_move);
                 continue;
            }
            legal_moves += 1;

            let eval = -self.alpha_beta(board, depth - 1, -beta, -alpha);
            board.un_make_move(generated_move);
            
            if self.abort_search {
                 return 0;
            }

            if eval > best_value {
                best_value = eval;
                best_move_found = Some(generated_move.clone());
            }
            
            alpha = std::cmp::max(alpha, eval);
            if alpha >= beta {
                best_value = beta; // Fail hard
                break; 
            }
        }
        
        if legal_moves == 0 {
             // Checkmate or Stalemate
             if evaluate::is_in_check(board, board.side_to_move, None) {
                 return -20000 + (self.current_depth as i32 - depth as i32); // Checkmate score adjusted for distance
             } else {
                 return 0; // Stalemate
             }
        }

        // TT Store
        let flag = if best_value <= alpha_orig {
             TTFlag::UpperBound
        } else if best_value >= beta {
             TTFlag::LowerBound
        } else {
             TTFlag::Exact
        };
        
        self.store_tt(position_hash, depth, best_value, flag, best_move_found);

        return best_value;
    }

    pub fn search(&mut self, board: &mut Board) -> (Move, Vec<BestMoves>) {
        let mut searching = true;
        let mut best_move = Move::default();
        let mut best_score = i32::MIN;
        let mut best_moves = Vec::new();

        self.clear_tt();
        self.searching_side = board.side_to_move;
        self.reset_search_state();
        self.time_limit = self.get_allowed_time(self.searching_side);
        
        let mut current_search_depth = 1;

        while searching {
             // Check time BEFORE starting a new depth iteration
             if self.use_time_management {
                 let elapsed = self.start.elapsed().as_millis();
                 // If we've used more than 40% of our time, don't start a new depth
                 if elapsed > (self.time_limit * 2) / 5 && current_search_depth > 1 {
                     searching = false;
                     break;
                 }
             }
             
             let alpha = i32::MIN + 1;
             let beta = i32::MAX;
             
             let score = self.alpha_beta(board, current_search_depth, alpha, beta);
             
             if self.abort_search {
                 break;
             }

             // Only update if search completed
             self.current_depth = current_search_depth;
             
             // Extract best move from TT
             let position_hash = conversion::hash_board_state_for_tt(board);
             if let Some(entry) = self.probe_tt(position_hash) {
                  if let Some(bm) = &entry.best_move {
                      best_move = bm.clone();
                      best_score = score;
                  }
             }

             // Decide whether to continue to next depth
             if current_search_depth >= self.depth {
                 searching = false;
             } else {
                 current_search_depth += 1;
                 
                 if current_search_depth > 50 { 
                     searching = false; 
                 }
                 
                 if self.use_time_management {
                     let elapsed = self.start.elapsed().as_millis();
                     if elapsed > (self.time_limit * 2) / 5 {
                         searching = false;
                     }
                 }
             }
        }
        
        // Populate best_moves vector for UI/UCI compatibility if needed by the caller
        best_moves.push(BestMoves {
            best_move: best_move.clone(),
            best_score: best_score,
        });

        return (best_move, best_moves);
    }

    pub fn perft(&mut self, board: &mut Board, depth: i8, first_call: bool) -> i128 {
        let mut nodes_per_root_move: i128;
        let mut nodes: i128 = 0;
        if depth == 0 {
            return 1;
        }

        let current_side = board.side_to_move;

        let currently_in_check = evaluate::is_in_check(board, current_side, None);

        let mut moves_for_current_depth = movegen::generate_pseudo_legal_moves(
            board,
            board.side_to_move,
            currently_in_check,
            false,
        );

        for generated_move in moves_for_current_depth.iter_mut() {
            board.make_move(generated_move);

            if board.has_positions_repeated() {
                generated_move.illegal_move = true;
                board.un_make_move(generated_move);
                continue;
            }

            if evaluate::is_in_check(
                board,
                current_side,
                generated_move.castling_intermediary_square,
            ) {
                board.un_make_move(generated_move);
                continue;
            }

            nodes_per_root_move = self.perft(board, depth - 1, false);
            nodes += nodes_per_root_move;
            board.un_make_move(generated_move);

            if first_call {
                // update root node here with number
                self.move_nodes.push(MoveNode {
                    move_notation: conversion::convert_move_to_notation(generated_move),
                    nodes: nodes_per_root_move,
                });
                self.nodes += nodes;
            }
        }

        return nodes;
    }
}

#[cfg(test)]
mod tests {
    use crate::conversion;
    use crate::search::Board;
    use crate::search::SearchEngine;
    #[test]
    fn perft_1_startpos() {
        let mut engine = SearchEngine::new();
        let mut board = Board::init();

        let nodes = engine.perft(&mut board, 1, true);
        assert_eq!(nodes, 20);
    }
    #[test]
    fn perft_2_startpos() {
        let mut engine = SearchEngine::new();
        let mut board = Board::init();

        let nodes = engine.perft(&mut board, 2, true);
        assert_eq!(nodes, 400);
    }

    #[test]
    fn perft_3_startpos() {
        let mut engine = SearchEngine::new();
        let mut board = Board::init();

        let nodes = engine.perft(&mut board, 3, true);
        assert_eq!(nodes, 8902);
    }
    #[test]
    fn perft_4_startpos() {
        let mut engine = SearchEngine::new();
        let mut board = Board::init();

        let nodes = engine.perft(&mut board, 4, true);
        assert_eq!(nodes, 197281);
    }

    #[test]
    fn perft_1_kiwipete() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ",
        );

        let nodes = engine.perft(&mut board, 1, true);
        assert_eq!(nodes, 48);
    }
    #[test]
    fn perft_2_kiwipete() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ",
        );

        let nodes = engine.perft(&mut board, 2, true);
        assert_eq!(nodes, 2039);
    }
    #[test]
    fn perft_3_kiwipete() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ",
        );

        let nodes = engine.perft(&mut board, 3, true);
        assert_eq!(nodes, 97862);
    }

    #[test]
    fn perft_1_position_3() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ");

        let nodes = engine.perft(&mut board, 1, true);
        assert_eq!(nodes, 14);
    }
    #[test]
    fn perft_2_position_3() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ");

        let nodes = engine.perft(&mut board, 2, true);
        assert_eq!(nodes, 191);
    }
    #[test]
    fn perft_3_position_3() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ");

        let nodes = engine.perft(&mut board, 3, true);
        assert_eq!(nodes, 2812);
    }
    #[test]
    fn perft_4_position_3() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ");

        let nodes = engine.perft(&mut board, 4, true);
        assert_eq!(nodes, 43238);
    }
    #[test]
    fn perft_5_position_3() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ");

        let nodes = engine.perft(&mut board, 5, true);
        assert_eq!(nodes, 674624);
    }

    #[test]
    fn perft_1_position_4() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        );

        let nodes = engine.perft(&mut board, 1, true);
        assert_eq!(nodes, 6);
    }
    #[test]
    fn perft_2_position_4() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        );

        let nodes = engine.perft(&mut board, 2, true);
        assert_eq!(nodes, 264);
    }
    #[test]
    fn perft_3_position_4() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        );

        let nodes = engine.perft(&mut board, 3, true);
        assert_eq!(nodes, 9467);
    }
    #[test]
    fn perft_4_position_4() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        );

        let nodes = engine.perft(&mut board, 4, true);
        assert_eq!(nodes, 422333);
    }

    #[test]
    fn perft_1_position_5() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board(
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        );

        let nodes = engine.perft(&mut board, 1, true);
        assert_eq!(nodes, 44);
    }
    #[test]
    fn perft_2_position_5() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board(
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        );

        let nodes = engine.perft(&mut board, 2, true);
        assert_eq!(nodes, 1486);
    }
    #[test]
    fn perft_3_position_5() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board(
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        );

        let nodes = engine.perft(&mut board, 3, true);
        assert_eq!(nodes, 62379);
    }
}
