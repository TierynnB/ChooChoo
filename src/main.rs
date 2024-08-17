pub mod board;
pub mod constants;
pub mod conversion;
pub mod evaluate;
pub mod movegen;
pub mod moves;
pub mod search;
pub mod uci;
fn main() {
    uci::run();
}
