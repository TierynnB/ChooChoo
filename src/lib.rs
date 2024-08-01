use std::io::{self};

const NAME: &str = "rust_chess";
const VERSION: &str = "0.1";
const HELP: &str = "uci commands \n\
 bench - run buit in bench";

// Pieces declarations
pub const PAWN: i8 = 1;
pub const KNIGHT: i8 = 2;
pub const BISHOP: i8 = 3;
pub const ROOK: i8 = 4;
pub const QUEEN: i8 = 5;
pub const KING: i8 = 6;

pub const WHITE: i8 = 1;
pub const BLACK: i8 = 2;
pub const EMPTY: i8 = 0;

pub struct Board {
    pub board_array: [[i8; 12]; 12],
    pub colour_array: [[i8; 12]; 12],
}

pub struct Move {
    pub from: (usize, usize),
    pub to: (usize, usize),
}
pub const BOARD_COORDINATES: [[&str; 12]; 12] = [
    ["", "", "", "", "", "", "", "", "", "", "", ""],
    ["", "", "", "", "", "", "", "", "", "", "", ""],
    [
        "", "", "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8", "", "",
    ],
    [
        "", "", "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7", "", "",
    ],
    [
        "", "", "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6", "", "",
    ],
    [
        "", "", "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "", "",
    ],
    [
        "", "", "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4", "", "",
    ],
    [
        "", "", "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "", "",
    ],
    [
        "", "", "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2", "", "",
    ],
    [
        "", "", "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "", "",
    ],
    ["", "", "", "", "", "", "", "", "", "", "", ""],
    ["", "", "", "", "", "", "", "", "", "", "", ""],
];

impl Board {
    pub fn init() -> Board {
        // initialise the board with a new game

        let board_array = [
            [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, 4, 2, 3, 5, 6, 3, 2, 4, -1, -1],
            [-1, -1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1],
            [-1, -1, 4, 2, 3, 5, 6, 3, 2, 4, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        ];
        let colour_array = [
            [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, 2, 2, 2, 2, 2, 2, 2, 2, -1, -1],
            [-1, -1, 2, 2, 2, 2, 2, 2, 2, 2, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1],
            [-1, -1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        ];

        return Board {
            board_array,
            colour_array,
        };
    }
    pub fn reset_board(&mut self) {
        self.board_array = [
            [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, 4, 2, 3, 5, 6, 3, 2, 4, -1, -1],
            [-1, -1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1],
            [-1, -1, 4, 2, 3, 5, 6, 3, 2, 4, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        ];
        self.colour_array = [
            [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, 2, 2, 2, 2, 2, 2, 2, 2, -1, -1],
            [-1, -1, 2, 2, 2, 2, 2, 2, 2, 2, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1],
            [-1, -1, 1, 1, 1, 1, 1, 1, 1, 1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        ];
    }

    pub fn make_move(&mut self, chess_move: String) -> Result<Move, String> {
        let move_to_do_result = convert_moveinput_to_array_location(chess_move);
        let move_to_do = match move_to_do_result {
            Err(e) => return Err(e),
            Ok(m) => m,
        };

        self.board_array[move_to_do.to.0][move_to_do.to.1] =
            self.board_array[move_to_do.from.0][move_to_do.from.1];

        self.board_array[move_to_do.from.0][move_to_do.from.1] = 0;

        self.colour_array[move_to_do.to.0][move_to_do.to.1] =
            self.colour_array[move_to_do.from.0][move_to_do.from.1];

        self.colour_array[move_to_do.from.0][move_to_do.from.1] = 0;

        return Ok(move_to_do);
    }
}

pub fn convert_moveinput_to_array_location(chess_move: String) -> Result<Move, String> {
    let mut converted_move = Move {
        from: (0, 0),
        to: (0, 0),
    };
    // should be in format e2e3
    if chess_move.len() != 4 {
        return Err("not equal to 4 characters".to_string());
    }
    for (index, char) in chess_move.chars().enumerate() {
        match index {
            0 => {
                if !char.is_alphabetic() && char.is_numeric() {
                    return Err(format!(
                        "first character must be the column letter: {}",
                        char
                    ));
                }
            }
            1 => {
                if char.is_alphabetic() && !char.is_numeric() {
                    return Err(format!(
                        "first character must be the numerical row: {}",
                        char
                    ));
                }
            }
            2 => {
                if !char.is_alphabetic() && char.is_numeric() {
                    return Err(format!(
                        "first character must be the column letter: {}",
                        char
                    ));
                }
            }
            3 => {
                if char.is_alphabetic() && !char.is_numeric() {
                    return Err(format!(
                        "first character must be the numerical row: {}",
                        char
                    ));
                }
            }
            _ => return Err("argument too long".to_string()),
        }
    }
    // get first two characters
    for (board_row_index, board_row) in BOARD_COORDINATES.iter().enumerate() {
        for (column_index, square_coordinate) in board_row.iter().enumerate() {
            // println!("{}", square_coordinate);
            if *square_coordinate == chess_move.get(0..2).unwrap() {
                converted_move.from.0 = board_row_index;
                converted_move.from.1 = column_index;
                break;
            }
        }
    }

    for (board_row_index, board_row) in BOARD_COORDINATES.iter().enumerate() {
        for (column_index, square_coordinate) in board_row.iter().enumerate() {
            // println!("{}", square_coordinate);
            if *square_coordinate == chess_move.get(2..4).unwrap() {
                converted_move.to.0 = board_row_index;
                converted_move.to.1 = column_index;
                break;
            }
        }
    }

    return Ok(converted_move);
}
/// prints the board from whites perspective in ascii to command line
pub fn print_board(board: Board) {

    //print board
}
pub fn generate_pawn_moves(
    square: (usize, usize),
    board: &Board,
    depth: i8,
    side_to_move: &i8,
) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let mut blocked = false;

    let direction_of_pawns: i8 = match side_to_move {
        1 => -1,
        2 => 1,
        _ => 0,
    };
    // know if double jump allowed if from starting row
    let starting_row = if *side_to_move == 1 { 9 } else { 4 };
    let (row, column) = square;

    // if square in front of pawn is not filled, can move there
    let index_of_square_in_front = if direction_of_pawns.is_negative() {
        row - 1
    } else {
        row + 1
    };
    let square_in_front = board.board_array[index_of_square_in_front][column];

    // if square not empty, return.
    if square_in_front != 0 {
        blocked = true;
    }

    moves.push(Move {
        from: square,
        to: (index_of_square_in_front, column),
    });

    // if there is a square diagonnally forward from the pawn possessed by enemy
    let mut square_attack = board.colour_array[index_of_square_in_front][column + 1];

    if square_attack != *side_to_move && square_attack != -1 && square_attack != 0 {
        moves.push(Move {
            from: square,
            to: (index_of_square_in_front, column + 1),
        });
    }
    square_attack = board.board_array[index_of_square_in_front][column - 1];
    if square_attack != *side_to_move && square_attack != -1 && square_attack != 0 {
        moves.push(Move {
            from: square,
            to: (index_of_square_in_front, column + 1),
        });
    }
    // if pawn on its starting square, can move two
    let index_of_square_in_front = if direction_of_pawns.is_negative() {
        row - 2
    } else {
        row + 2
    };
    let square_in_front = board.board_array[index_of_square_in_front][column];

    // if square not empty, return.
    if square_in_front == 0 && blocked == false {
        moves.push(Move {
            from: square,
            to: (index_of_square_in_front, column),
        });
    }

    // enpassant? need to know previous move and if on the right square.

    return moves;
}

pub fn evaluate() {}

pub fn generate_pseudo_legal_moves(board: &Board, depth: i8, side_to_move: &i8) {
    let mut moves: Vec<Move> = vec![];

    // go through each piece on the board, by colour to only get moves for side to move.
    for (row_index, row) in board.colour_array.iter().enumerate() {
        for (column_index, colour) in row.iter().enumerate().filter(|(_a, b)| *b == side_to_move) {
            let location = (row_index, column_index);
            let square = board.board_array[row_index][column_index];

            println!("{}, {}, {}, {}", row_index, column_index, colour, square);

            let mut generated_moves = match square {
                1 => generate_pawn_moves(location, board, depth, side_to_move),
                // 2 => KNIGHT,
                // 3 => BISHOP,
                // 4 => ROOK,
                // 5 => QUEEN,
                // 6 => PAWN,
                // 0 => 0,
                // -1 => -1,
                _ => vec![],
            };

            for (_index, generated_move) in generated_moves.iter().enumerate() {
                println!(
                    "from:{} {} to: {} {}",
                    generated_move.from.0,
                    generated_move.from.1,
                    generated_move.to.0,
                    generated_move.to.1
                );
            }
            moves.append(&mut generated_moves);
        }

        //
    }
    // come up with every possible move

    // append to

    // return moves;
}

// pub fn generate_moves(board: Board, side_to_move: i8) -> ();
pub fn run() {
    println!("{} {}", NAME, VERSION);

    let mut board = Board::init();

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
                "bench" => println!("run bench"),
                "uci" => println!("enable uci mode"),
                "debug" => println!("turn debug mode on or off"),
                "isready" => println!("yeah lets go"),
                "reset_board" => board.reset_board(),
                "printBoard" => println!("{:?}", board.board_array),
                "setoption" => match inputs.next() {
                    None => println!("no more commands"),
                    Some(arg_2) => println!("found a match for option {}", arg_2),
                },
                "move" => match inputs.next() {
                    None => println!("no more commands"),
                    Some(arg_2) => {
                        let outcome = board.make_move(arg_2.to_string());
                        match outcome {
                            Ok(m) => println!(
                                "made the mode: from {},{}, to: {},{}",
                                m.from.0, m.from.1, m.to.0, m.to.1
                            ),
                            Err(e) => println!("{}", e),
                        }
                    }
                },
                "generate" => generate_pseudo_legal_moves(&board, 1, &WHITE),
                _ => {
                    println!("unknown command {}", buffer);
                }
            },
        }
        buffer.clear();
    }
}
