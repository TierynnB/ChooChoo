//! used to communicate with the engine

use crate::bench;
use crate::board::*;
use crate::search::*;
use crate::{conversion, evaluate};
use std::io;

const NAME: &str = "ChooChoo";
const VERSION: &str = "0.1";
const HELP: &str = "bench - run buit in bench";
const AUTHOR: &str = "Tierynn Byrnes";
const CHOO_CHOO_TRAIN: &str = r"
____
|DD|____T_
|_ |_____|<
@-@-@-oo\ ";

pub enum CommandTypes {
    Uci,
    IsReady,
    Position,
    SetOption,
    NewGame,
    UciNewGame,
    Go,
    PrintState,
    Evaluate,
    Perft,
    MakeMove,
    MakeUnMake,
    Bench,
    GetFen,
    Hash,
    Empty,
    Invalid, //
    Quit,    //
    Search,  //
    MoveList,
    Help,
}

pub struct CommunicationManager {
    uci_enabled: bool,
    // debug_enabled: bool,
    board: Board,
    engine: SearchEngine,
}
pub struct UciCommandOptions {
    // no option supported atm
}
impl CommunicationManager {
    pub fn new() -> Self {
        CommunicationManager {
            uci_enabled: false,
            // debug_enabled: false,
            board: Board::init(),
            engine: SearchEngine::new(),
        }
    }
    pub fn quit() {
        println!("bye");
        std::process::exit(0);
    }
    pub fn get_first_command(first_command: &str) -> CommandTypes {
        match first_command {
            "uci" => CommandTypes::Uci,
            "isready" => CommandTypes::IsReady,
            "position" => CommandTypes::Position,
            "hash" => CommandTypes::Hash,
            "go" => CommandTypes::Go,
            "search" => CommandTypes::Search,
            "quit" => CommandTypes::Quit,
            "printstate" | "show" | "print" => CommandTypes::PrintState,
            "evaluate" => CommandTypes::Evaluate,
            "perft" => CommandTypes::Perft,
            "newgame" => CommandTypes::NewGame,
            "ucinewgame" => CommandTypes::UciNewGame,
            "setoption" => CommandTypes::SetOption,
            "makeunmake" => CommandTypes::MakeUnMake,
            "movelist" => CommandTypes::MoveList,
            // "splitperft" => CommandTypes::SplitPerft,
            // "perftsuite" => CommandTypes::PerftSuite,
            "makemove" => CommandTypes::MakeMove,
            "bench" => CommandTypes::Bench,
            "help" => CommandTypes::Help,
            _ => {
                println!("invalid command: {}", first_command);
                CommandTypes::Invalid
            }
        }
    }
    pub fn position(&mut self, command_text: &str) {
        // first token should be "position"
        // second token should be "fen" or "startpos"ev
        // if fen, is followed by a 6 space separated tokens

        // after that, can be "moves".
        // if so, it can be followed by a list of moves.
        let mut command_text_split = command_text.split_ascii_whitespace();
        let _first_token = command_text_split.next().expect("no token");
        let second_token = command_text_split.next().expect("no token");

        if second_token == "startpos" {
            self.board = Board::init();
        }

        if second_token == "fen" {
            let mut fen_string = String::new();
            for _fen_tokens in 1..7 {
                fen_string
                    .push_str(format!("{} ", command_text_split.next().unwrap_or("")).as_str());
            }
            println!("fen: {}", fen_string);
            self.board = conversion::convert_fen_to_board(fen_string.as_str());
        }

        let mut moves_token = second_token;
        if second_token != "moves" {
            moves_token = command_text_split.next().unwrap_or_default();
        }

        // println!("moves token: {}", moves_token);
        if moves_token == "moves" {
            for move_token in command_text_split {
                // println!("move: {}", move_token);
                self.board
                    .make_move_with_notation(move_token.to_string())
                    .expect("invalid move");
            }
        }
    }
    pub fn evaluate(&self) {
        println!(
            "evaluate with negamax for {}: {}",
            self.board.side_to_move,
            evaluate::evaluate(&self.board) // self.board.get_running_evaluation()
        );
    }

    pub fn search(&mut self, command_text: &str) {
        let mut command_text_split = command_text.split_ascii_whitespace();
        let _search_token = command_text_split.next().expect("no token");

        match command_text_split.next() {
            None => println!("no more commands"),
            Some(arg_2) => {
                self.engine = SearchEngine::new();
                let _depth: i8 = arg_2.parse::<i8>().expect("Invalid depth value");
                let outcome = self.engine.search(&mut self.board);
                println!(
                    "nodes: {}, time:{:?}, nodes per second: {}",
                    self.engine.nodes,
                    self.engine.start.elapsed().as_micros(),
                    self.engine.nodes as f32 / self.engine.start.elapsed().as_secs_f32()
                );

                // get random move from best moves with matching top score.
                println!(
                    "best move {}, score {}",
                    conversion::convert_move_to_notation(&outcome.1[0].best_move),
                    outcome.1[0].best_score
                );
            }
        }
    }
    pub fn make_move(&mut self, command_text: &str) {
        let mut command_token_split = command_text.split_ascii_whitespace();
        let _first_token = command_token_split.next().expect("no token");

        match command_token_split.next() {
            None => println!("no more commands"),
            Some(arg_2) => {
                let outcome = self.board.make_move_with_notation(arg_2.to_string());
                match outcome {
                    Ok(m) => {
                        println!(
                            "made the mode: from {},{}, to: {},{}, notation: {}",
                            m.from.0,
                            m.from.1,
                            m.to.0,
                            m.to.1,
                            conversion::convert_move_to_notation(&m)
                        );
                        println!("piece that move {}", m.from_piece);
                        println!(" to piece  {}", m.to_piece);
                        // board.un_make_move(m);
                        print_board(&self.board);
                    }
                    Err(e) => println!("{}", e),
                }
            }
        }
    }
    pub fn make_unmake_move(&mut self, command_text: &str) {
        let mut command_token_split = command_text.split_ascii_whitespace();
        let _first_token = command_token_split.next().expect("no token");

        match command_token_split.next() {
            None => println!("no more commands"),
            Some(arg_2) => {
                let move_to_do = self.board.convert_notation_to_move(arg_2.to_string());
                self.board.make_move(move_to_do.as_ref().unwrap());
                print_board(&self.board);
                self.board.un_make_move(move_to_do.as_ref().unwrap());
                print_board(&self.board);
            }
        }
    }
    pub fn perft(&mut self, command_text: &str) {
        let depth: i8 = command_text
            .split_ascii_whitespace()
            .nth(1)
            .expect("Invalid depth value")
            .parse()
            .expect("Invalid depth value");
        self.engine = SearchEngine::new();
        let nodes = self.engine.perft(&mut self.board, depth, true);
        // println!("total nodes: {}", self.engine.nodes);
        // println!("root moves: {}", self.engine.move_nodes.len());
        for root in self.engine.move_nodes.iter() {
            println!("{} - {}", root.move_notation, root.nodes);
        }
        println!("total nodes: {}", self.engine.nodes);
        println!("root moves: {}", self.engine.move_nodes.len());
        println!("perft nodes: {}", nodes);
        println!()
    }
    pub fn enable_uci(&mut self) {
        self.uci_enabled = true;
        println!("id name {}", NAME);
        println!("id author {}", AUTHOR);

        // not true yet
        println!("option name Move Overhead type spin default 10 min 0 max 2000");
        println!("option name Threads type spin default 1 min 1 max 1");
        println!("option name Hash type spin default 0 min 0 max 0");
        println!("uciok");
        // output all the options curently supported
    }
    pub fn go(&mut self, command_text: &str) {
        let mut command_text_split = command_text.split_ascii_whitespace();
        let _first_token = command_text_split.next().expect("no token");

        while let Some(token) = command_text_split.next() {
            // println!("token: {}", token);

            match token {
                "wtime" => {
                    self.engine.wtime = command_text_split.next().unwrap().parse::<u128>().unwrap();
                    self.engine.use_time_management = true;
                }
                "btime" => {
                    self.engine.btime = command_text_split.next().unwrap().parse::<u128>().unwrap();
                    self.engine.use_time_management = true;
                }
                "winc" => {
                    self.engine.winc = command_text_split.next().unwrap().parse::<u128>().unwrap();
                    self.engine.use_time_management = true;
                }
                "binc" => {
                    self.engine.wtime = command_text_split.next().unwrap().parse::<u128>().unwrap();
                    self.engine.use_time_management = true;
                }
                "movestogo" => {
                    self.engine.depth = command_text_split.next().unwrap().parse::<i8>().unwrap();
                }
                "depth" => {
                    self.engine.depth = command_text_split.next().unwrap().parse::<i8>().unwrap();
                }
                "movetime" => {
                    self.engine.movetime =
                        command_text_split.next().unwrap().parse::<u128>().unwrap();
                    self.engine.use_time_management = true;
                }

                _ => {}
            }
        }
        let moves = self.engine.search(&mut self.board);

        let time_taken_seconds = self.engine.start.elapsed().as_secs_f32();

        println!(
            "info depth {} time {} nodes {} nps {} score cp {:.2}",
            self.engine.current_depth,
            self.engine.start.elapsed().as_millis(),
            self.engine.nodes,
            self.engine.nodes as f32 / time_taken_seconds,
            moves.0.search_score,
        );
        println!(
            "bestmove {}",
            conversion::convert_array_location_to_notation(
                moves.0.from,
                moves.0.to,
                Some(match moves.0.promotion_to.unwrap_or(0) {
                    1 => 'p'.to_string(),
                    2 => 'n'.to_string(),
                    3 => 'b'.to_string(),
                    4 => 'r'.to_string(),
                    5 => 'q'.to_string(),
                    6 => 'k'.to_string(),
                    0 => ' '.to_string(),
                    _ => ' '.to_string(),
                })
            )
        );
        // return moves[0];
        // do the search with the provided settings
    }
}

pub fn run() {
    println!("{} {}", NAME, VERSION);
    println!("{}", CHOO_CHOO_TRAIN);

    let stdin = io::stdin();

    let mut manager = CommunicationManager::new();

    loop {
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).expect("Failed to read line");

        let mut inputs = buffer.split_ascii_whitespace();

        let Some(first_command) = inputs.next() else {
            println!("should have provided a command");
            println!("{}", CHOO_CHOO_TRAIN);
            continue;
        };

        let command = CommunicationManager::get_first_command(first_command);

        match command {
            CommandTypes::Uci => manager.enable_uci(),
            CommandTypes::Quit => CommunicationManager::quit(),
            CommandTypes::Position => manager.position(&buffer),
            CommandTypes::Search => manager.search(&buffer),
            CommandTypes::MakeMove => manager.make_move(&buffer),
            CommandTypes::MakeUnMake => manager.make_unmake_move(&buffer),
            CommandTypes::Perft => manager.perft(&buffer),
            CommandTypes::Evaluate => manager.evaluate(),
            CommandTypes::NewGame => manager.board.reset_board(),
            CommandTypes::PrintState => print_board(&manager.board),
            CommandTypes::UciNewGame => {} // do nothing
            CommandTypes::MoveList => {
                for move_item in &manager.board.move_list {
                    println!("move from:{:?}, to: {:?}", move_item.from, move_item.to);
                }
            }
            CommandTypes::Invalid => {
                println!("invalid or unsupported command");
                println!("{}", &buffer);
            }
            CommandTypes::SetOption => {}
            CommandTypes::Bench => bench::bench(), //manager.bench(),
            CommandTypes::IsReady => println!("readyok"),
            CommandTypes::Go => manager.go(&buffer),
            CommandTypes::GetFen => println!("{}", manager.board.get_fen()),
            CommandTypes::Hash => {
                println!(
                    "hash: {}, has repeated: {}",
                    conversion::hash_board_state(&manager.board),
                    manager.board.has_positions_repeated()
                );

                for hash in manager.board.hash_of_previous_positions.iter() {
                    println!("hash: {}", hash);
                }
            }

            CommandTypes::Help => {
                println!("{}", HELP);
                println!("{}", CHOO_CHOO_TRAIN)
            }
            _ => panic!("Choo Choo Trouble {}", CHOO_CHOO_TRAIN),
        }
        buffer.clear();
    }
}
