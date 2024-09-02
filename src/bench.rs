use crate::constants;
use crate::conversion;
use crate::search;
// normal bench of fen search

pub fn bench() {
    let mut engine = search::SearchEngine::new();
    let mut nodes = 0;
    let mut time_taken_micros = 0;
    let mut time_taken_seconds: f32 = 0.00;
    for bench_fen in constants::BENCH_FENS {
        println!("bench fen: {}", bench_fen);
        let mut board = conversion::convert_fen_to_board(bench_fen);

        engine.search(&mut board);

        nodes += engine.nodes;
        time_taken_micros += engine.start.elapsed().as_micros();
        time_taken_seconds += engine.start.elapsed().as_secs_f32();
    }

    println!(
        "nodes: {}, time:{:?}, nodes per second: {}",
        nodes,
        time_taken_micros,
        nodes as f32 / time_taken_seconds
    );
}
