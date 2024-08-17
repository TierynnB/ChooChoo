//! used to communicate with the engine
use crate::board::*;
use crate::search::*;
use crate::{conversion::*, evaluate::*, movegen::*};
use std::io;
const NAME: &str = "rust_chess";
const VERSION: &str = "0.1";
const HELP: &str = "uci commands \n\
 bench - run buit in bench";

pub struct Data {
    uci_enabled: bool,
    debug_enabled: bool,
}
pub fn run() {
    println!("{} {}", NAME, VERSION);

    let mut board = Board::init();
    let mut search_engine = SearchEngine::new();
    let stdin = io::stdin();

    // loop over the user inputs
    loop {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).expect("Failed to read line");
        let mut inputs = buffer.trim().split(' ');

        match inputs.next() {
            None => println!("should have provided a command"),
            Some(arg) => match arg {
                "help" => println!("{}", HELP),
                "hash" => println!("{}", board.hash_board_state()),
                "bench" => println!("run bench"),
                "uci" => println!("enable uci mode"),
                "debug" => println!("turn debug mode on or off"),
                "isSideInCheck" => println!("{}", board.is_side_in_check(board.side_to_move)),
                "isready" => println!("yeah lets go"),
                "reset_board" => board.reset_board(),
                "printBoard" => print_board(&board),
                "IsRepeated" => println!("{}", board.has_positions_repeated()),
                "search" => match inputs.next() {
                    None => println!("no more commands"),
                    Some(arg_2) => {
                        search_engine = SearchEngine::new();
                        let depth: i8 = arg_2.parse().expect("Invalid depth value");
                        let outcome = search_engine.search(&mut board, depth);
                        println!(
                            "nodes: {}, time:{:?}, nodes per second: {}",
                            search_engine.nodes,
                            search_engine.start.elapsed().as_micros(),
                            search_engine.nodes as f32
                                / search_engine.start.elapsed().as_secs_f32()
                        );
                        println!(
                            "best move {}, score {}",
                            outcome[0].best_move.notation_move, outcome[0].best_score
                        );
                    }
                },
                "setoption" => match inputs.next() {
                    None => println!("no more commands"),
                    Some(arg_2) => println!("found a match for option {}", arg_2),
                },
                "getPieceValue" => {
                    println!("{}", get_piece_square_value((8, 2), 1, 2));
                }
                "fen" => {
                    let mut fen_string = String::new();
                    for input in inputs {
                        fen_string.push_str(input);
                        fen_string.push(' ');
                    }

                    fen_string.pop(); // remove the trailing space
                    board = convert_fen_to_board(&fen_string);
                }

                "move" => match inputs.next() {
                    None => println!("no more commands"),
                    Some(arg_2) => {
                        let outcome = board.make_move_with_notation(arg_2.to_string());
                        match outcome {
                            Ok(m) => {
                                println!(
                                    "made the mode: from {},{}, to: {},{}, notation: {}",
                                    m.from.0, m.from.1, m.to.0, m.to.1, m.notation_move
                                );
                                println!("piece that move {}", m.from_piece);
                                println!(" to piece  {}", m.to_piece);
                                // board.un_make_move(m);
                                print_board(&board);
                            }
                            Err(e) => println!("{}", e),
                        }
                    }
                },
                "eval" => {
                    println!("{}, {}", evaluate(&board), board.running_evaluation);
                }
                "perft" => {
                    // perft
                    search_engine = SearchEngine::new();
                    let depth: i8 = inputs
                        .next()
                        .expect("Invalid depth value")
                        .parse()
                        .expect("Invalid depth value");

                    search_engine.perft(&mut board, depth, true);
                    println!("total nodes: {}", search_engine.nodes);
                    println!("root moves: {}", search_engine.move_nodes.len());
                    for root in search_engine.move_nodes.iter() {
                        println!("{} - {}", root.move_notation, root.nodes);
                    }
                    println!()
                }
                "generate" => {
                    let side_to_move = board.side_to_move;
                    let moves = generate_pseudo_legal_moves(&mut board, side_to_move);
                    for (_index, generated_move) in moves.iter().enumerate() {
                        println!("{}", generated_move.notation_move);
                    }
                    println!("total moves: {}", moves.len());
                }
                _ => {
                    println!("unknown command {}", buffer);
                }
            },
        }
        buffer.clear();
    }
}
