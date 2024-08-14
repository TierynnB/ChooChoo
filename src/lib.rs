use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::io::{self};
use std::time::Instant;

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
pub struct MoveNode {
    move_notation: String,
    nodes: i32,
}
pub struct BestMoves {
    pub best_move: Move,
    pub best_score: i32,
}
pub struct SearchEngine {
    pub nodes: i32,
    start: Instant,
    pub move_nodes: Vec<MoveNode>,
}

pub struct Data {
    uci_enabled: bool,
    debug_enabled: bool,
}
#[derive(Clone)]
pub struct PlyData {
    pub ply: i8,
    pub side_to_move: i8,
    has_king_moved: bool,
    a1_rook_not_moved: bool, // defaults to true
    a8_rook_not_moved: bool, // defaults to true
    h1_rook_not_moved: bool, // defaults to true
    h8_rook_not_moved: bool, // defaults to true
    en_passant: bool,
    en_passant_location: Option<(usize, usize)>,
}
#[derive(Clone)]
pub struct Board {
    pub board_array: [[i8; 12]; 12],
    pub colour_array: [[i8; 12]; 12],
    has_king_moved: bool,
    a1_rook_not_moved: bool, // defaults to true
    a8_rook_not_moved: bool, // defaults to true
    h1_rook_not_moved: bool, // defaults to true
    h8_rook_not_moved: bool, // defaults to true
    en_passant: bool,
    en_passant_location: Option<(usize, usize)>,
    pub ply: i8,
    pub side_to_move: i8,
    pub hash_of_previous_positions: Vec<String>,
    pub ply_record: Vec<PlyData>,
    pub running_evaluation: i32,
    pub player_colour: i8,
}

#[derive(Clone)]
pub struct Move {
    pub from: (usize, usize),
    pub from_piece: i8,
    pub from_colour: i8,
    pub to: (usize, usize),
    pub to_piece: i8,
    pub to_colour: i8,
    pub notation_move: String,
    pub promotion_to: Option<i8>,
    pub en_passant: bool,
    pub castle_from_to_square: Option<((usize, usize), (usize, usize))>,
    pub sort_score: i32,
}
impl Default for Move {
    fn default() -> Self {
        // return a default instance of Move
        return Move {
            from: (0, 0),
            from_piece: 0,
            from_colour: 0,
            to: (0, 0),
            to_piece: 0,
            to_colour: 0,

            notation_move: String::default(),
            promotion_to: None,
            en_passant: false,
            castle_from_to_square: None,
            sort_score: 0,
        };
    }
}
// MVV_VLA[victim][attacker]
// Most Valued Victim, Least Valued Attacker
pub const MVV_LVA: [[u8; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0],       // victim K, attacker K, Q, R, B, N, P, None
    [10, 11, 12, 13, 14, 15, 0], // victim P, attacker K, Q, R, B, N, P, None
    [20, 21, 22, 23, 24, 25, 0], // victim N, attacker K, Q, R, B, N, P, None
    [30, 31, 32, 33, 34, 35, 0], // victim B, attacker K, Q, R, B, N, P, None
    [40, 41, 42, 43, 44, 45, 0], // victim R, attacker K, Q, R, B, N, P, None
    [50, 51, 52, 53, 54, 55, 0], // victim Q, attacker K, Q, R, B, N, P, None
    [0, 0, 0, 0, 0, 0, 0],       // victim None, attacker K, Q, R, B, N, P, None
];
pub const MG_PAWN_TABLE: [[i32; 8]; 8] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [98, 134, 61, 95, 68, 126, 34, -11],
    [-6, 7, 26, 31, 65, 56, 25, -20],
    [-14, 13, 6, 21, 23, 12, 17, -23],
    [-27, -2, -5, 12, 17, 6, 10, -25],
    [-26, -4, -4, -10, 3, 3, 33, -12],
    [-35, -1, -20, -23, -15, 24, 38, -22],
    [0, 0, 0, 0, 0, 0, 0, 0],
];
pub const MG_KNIGHT_TABLE: [[i32; 8]; 8] = [
    [-167, -89, -34, -49, 61, -97, -15, -107],
    [-73, -41, 72, 36, 23, 62, 7, -17],
    [-47, 60, 37, 65, 84, 129, 73, 44],
    [-9, 17, 19, 53, 37, 69, 18, 22],
    [-13, 4, 16, 13, 28, 19, 21, -8],
    [-23, -9, 12, 10, 19, 17, 25, -16],
    [-29, -53, -12, -3, -1, 18, -14, -19],
    [-105, -21, -58, -33, -17, -28, -19, -23],
];
pub const MG_BISHOP_TABLE: [[i32; 8]; 8] = [
    [-29, 4, -82, -37, -25, -42, 7, -8],
    [-26, 16, -18, -13, 30, 59, 18, -47],
    [-16, 37, 43, 40, 35, 50, 37, -2],
    [-4, 5, 19, 50, 37, 37, 7, -2],
    [-6, 13, 13, 26, 34, 12, 10, 4],
    [0, 15, 15, 15, 14, 27, 18, 10],
    [4, 15, 16, 0, 7, 21, 33, 1],
    [-33, -3, -14, -21, -13, -12, -39, -21],
];

pub const MG_ROOK_TABLE: [[i32; 8]; 8] = [
    [32, 42, 32, 51, 63, 9, 31, 43],
    [27, 32, 58, 62, 80, 67, 26, 44],
    [-5, 19, 26, 36, 17, 45, 61, 16],
    [-24, -11, 7, 26, 24, 35, -8, -20],
    [-36, -26, -12, -1, 9, -7, 6, -23],
    [-45, -25, -16, -17, 3, 0, -5, -33],
    [-44, -16, -20, -9, -1, 11, -6, -71],
    [-19, -13, 1, 17, 16, 7, -37, -26],
];
pub const MG_QUEEN_TABLE: [[i32; 8]; 8] = [
    [-28, 0, 29, 12, 59, 44, 43, 45],
    [-24, -39, -5, 1, -16, 57, 28, 54],
    [-13, -17, 7, 8, 29, 56, 47, 57],
    [-27, -27, -16, -16, -1, 17, -2, 1],
    [-9, -26, -9, -10, -2, -4, 3, -3],
    [-14, 2, -11, -2, -5, 2, 14, 5],
    [-35, -8, 11, 2, 8, 15, -3, 1],
    [-1, -18, -9, 10, -15, -25, -31, -50],
];
pub const MG_KING_TABLE: [[i32; 8]; 8] = [
    [-65, 23, 16, -15, -56, -34, 2, 13],
    [29, -1, -20, -7, -8, -4, -38, -29],
    [-9, 24, 2, -16, -20, 6, 22, -22],
    [-17, -20, -12, -27, -30, -25, -14, -36],
    [-49, -1, -27, -39, -46, -44, -33, -51],
    [-14, -14, -22, -46, -44, -30, -15, -27],
    [1, 7, -8, -64, -43, -16, 9, 8],
    [-15, 36, 12, -54, 8, -28, 24, 14],
];
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
pub const EVAL_WEIGHTS: [[i8; 12]; 12] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 2, 2, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 2, 2, 2, 2, 1, 1, 0, 0],
    [0, 0, 1, 1, 2, 2, 2, 2, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 2, 2, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
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
            a1_rook_not_moved: true,
            a8_rook_not_moved: true,
            h1_rook_not_moved: true,
            h8_rook_not_moved: true,
            has_king_moved: false,
            en_passant: false,
            en_passant_location: None,
            ply: 0,
            side_to_move: 1,
            hash_of_previous_positions: Vec::new(),
            ply_record: Vec::new(),
            running_evaluation: 0,
            player_colour: 1,
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
        self.a1_rook_not_moved = true;
        self.h1_rook_not_moved = true;
        self.a8_rook_not_moved = true;
        self.h8_rook_not_moved = true;
        self.en_passant = false;
        self.en_passant_location = None;
        self.has_king_moved = false;
        self.ply = 0;
        self.side_to_move = 1;
        self.hash_of_previous_positions = Vec::new();
        self.ply_record = Vec::new();
        self.running_evaluation = 0;
        self.player_colour = 1;
    }
    fn clear_hash_of_previous_positions(&mut self) {
        self.hash_of_previous_positions = Vec::new();
    }

    fn add_hash_of_current_position(&mut self) {
        self.hash_of_previous_positions
            .push(self.hash_board_state());
    }
    fn remove_last_hash(&mut self) {
        self.hash_of_previous_positions.pop();
    }
    pub fn clear_board(&mut self) {
        self.reset_board();

        // set all squares to empty
        self.board_array = [
            [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        ];
        self.colour_array = [
            [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
        ];
    }
    pub fn remove_piece_from_evaluation(
        &mut self,
        location: (usize, usize),
        piece_type: i8,
        piece_colour: i8,
    ) {
        let value = get_piece_square_value(location, piece_type, piece_colour);
        if piece_colour == self.player_colour {
            self.running_evaluation -= value;
        } else {
            self.running_evaluation += value;
        }
        // println!("running_evaluation: {}", self.running_evaluation);
    }
    pub fn add_piece_to_evaluation(
        &mut self,
        location: (usize, usize),
        piece_type: i8,
        piece_colour: i8,
    ) {
        let value = get_piece_square_value(location, piece_type, piece_colour);
        if piece_colour == self.player_colour {
            self.running_evaluation += value;
        } else {
            self.running_evaluation -= value;
        }
    }
    pub fn make_move(&mut self, move_to_do: &Move) {
        // copy data to record
        self.ply_record.push(PlyData {
            ply: self.ply,
            side_to_move: self.side_to_move,
            has_king_moved: self.has_king_moved,
            a1_rook_not_moved: self.a1_rook_not_moved,
            a8_rook_not_moved: self.a8_rook_not_moved,
            h1_rook_not_moved: self.h1_rook_not_moved,
            h8_rook_not_moved: self.h8_rook_not_moved,
            en_passant: self.en_passant,
            en_passant_location: self.en_passant_location,
        });

        // set board level en passant information
        self.en_passant = move_to_do.en_passant;
        self.en_passant_location = Some(move_to_do.to);

        self.board_array[move_to_do.to.0][move_to_do.to.1] =
            self.board_array[move_to_do.from.0][move_to_do.from.1];

        self.board_array[move_to_do.from.0][move_to_do.from.1] = 0;

        self.colour_array[move_to_do.to.0][move_to_do.to.1] =
            self.colour_array[move_to_do.from.0][move_to_do.from.1];

        self.colour_array[move_to_do.from.0][move_to_do.from.1] = 0;

        // need to know if castling
        if move_to_do.from_piece == ROOK
            && (self.a1_rook_not_moved
                || self.a8_rook_not_moved
                || self.h1_rook_not_moved
                || self.h8_rook_not_moved)
        {
            // check if rook moved from a1 or h1 or a8 or h8
            if move_to_do.from == (9, 2) {
                self.a1_rook_not_moved = false;
            } else if move_to_do.from == (9, 9) {
                self.h1_rook_not_moved = false;
            } else if move_to_do.from == (2, 2) {
                self.a8_rook_not_moved = false;
            } else if move_to_do.from == (2, 9) {
                self.h8_rook_not_moved = false;
            }
        }

        // If current move castling, move rook too
        if move_to_do.castle_from_to_square.is_some() {
            let castle_from_to_square = move_to_do.castle_from_to_square.unwrap();
            self.board_array[castle_from_to_square.1 .0][castle_from_to_square.1 .1] =
                self.board_array[castle_from_to_square.0 .0][castle_from_to_square.0 .1];

            self.board_array[castle_from_to_square.0 .0][castle_from_to_square.0 .1] = 0;

            self.colour_array[castle_from_to_square.1 .0][castle_from_to_square.1 .1] =
                self.colour_array[castle_from_to_square.0 .0][castle_from_to_square.0 .1];

            self.colour_array[castle_from_to_square.0 .0][castle_from_to_square.0 .1] = 0;
        }

        // if a capture, delete all hashes as there can never be a repeat of any previous position once a piece is removed.
        self.add_hash_of_current_position();

        // set side to move to opposite
        self.side_to_move = if self.side_to_move == WHITE {
            BLACK
        } else {
            WHITE
        };

        self.ply += 1;

        //update evaluation
        self.remove_piece_from_evaluation(
            move_to_do.from,
            move_to_do.from_piece,
            move_to_do.from_colour,
        );
        self.remove_piece_from_evaluation(move_to_do.to, move_to_do.to_piece, move_to_do.to_colour);

        // if promotion, add promotion piece
        if move_to_do.promotion_to.is_some() {
            self.add_piece_to_evaluation(
                move_to_do.to,
                move_to_do.promotion_to.unwrap(),
                move_to_do.from_colour,
            );
        } else {
            self.add_piece_to_evaluation(
                move_to_do.to,
                move_to_do.from_piece,
                move_to_do.from_colour,
            );
        }
    }

    /// make move does not validate the move, it just does it, overwritting the destination square
    pub fn make_move_with_notation(&mut self, chess_move: String) -> Result<Move, String> {
        let move_to_do_result = self.convert_notation_to_move(chess_move);
        let move_to_do = match move_to_do_result {
            Err(e) => return Err(e),
            Ok(m) => m,
        };

        // validate move
        if move_to_do.from_piece == EMPTY {
            return Err("cannot move empty square".to_string());
        }
        if move_to_do.from_colour == EMPTY {
            return Err("cannot move empty colour".to_string());
        }

        self.make_move(&move_to_do);
        return Ok(move_to_do);
    }

    /// make move does not validate the move, it just does it, overwritting the destination square
    pub fn un_make_move(&mut self, chess_move: &Move) {
        // the move should retain the original pieces in each square.
        self.board_array[chess_move.to.0][chess_move.to.1] = chess_move.to_piece;

        self.colour_array[chess_move.to.0][chess_move.to.1] = chess_move.to_colour;

        self.colour_array[chess_move.from.0][chess_move.from.1] = chess_move.from_colour;

        self.board_array[chess_move.from.0][chess_move.from.1] = chess_move.from_piece;

        self.remove_last_hash();

        self.ply -= 1;
        self.side_to_move = if self.side_to_move == WHITE {
            BLACK
        } else {
            WHITE
        };
        // remove last ply data

        let previous_ply = self.ply_record.last();

        if previous_ply.is_some() {
            let previous_ply_data = previous_ply.unwrap();

            // aply previous ply data to self.
            self.ply = previous_ply_data.ply;
            self.side_to_move = previous_ply_data.side_to_move;
            self.has_king_moved = previous_ply_data.has_king_moved;
            self.a1_rook_not_moved = previous_ply_data.a1_rook_not_moved;
            self.a8_rook_not_moved = previous_ply_data.a8_rook_not_moved;
            self.h1_rook_not_moved = previous_ply_data.h1_rook_not_moved;
            self.h8_rook_not_moved = previous_ply_data.h8_rook_not_moved;
            self.en_passant = previous_ply_data.en_passant;
            self.en_passant_location = previous_ply_data.en_passant_location;
        }
        self.ply_record.pop();
        //update evaluation
        self.add_piece_to_evaluation(chess_move.to, chess_move.from_piece, chess_move.to_colour);
        //update evaluation
        // if promotion, add promotion piece
        if chess_move.promotion_to.is_some() {
            self.remove_piece_from_evaluation(
                chess_move.to,
                chess_move.promotion_to.unwrap(),
                chess_move.from_colour,
            );
        } else {
            self.remove_piece_from_evaluation(
                chess_move.to,
                chess_move.from_piece,
                chess_move.from_colour,
            );
        }
        self.add_piece_to_evaluation(
            chess_move.from,
            chess_move.from_piece,
            chess_move.from_colour,
        );
    }

    pub fn convert_notation_to_move(&self, chess_move: String) -> Result<Move, String> {
        let mut converted_move = Move::default();

        // castling is a special case
        if chess_move == "O-O" || chess_move == "O-O-O" {
            let from_to_squares = match chess_move.as_str() {
                "O-O" => {
                    if self.side_to_move == WHITE {
                        ((9, 6), (9, 8), (9, 9), (9, 7))
                    } else {
                        ((2, 6), (2, 8), (2, 9), (2, 7))
                    }
                }
                "O-O-O" => {
                    if self.side_to_move == WHITE {
                        ((9, 6), (9, 4), (9, 2), (9, 5))
                    } else {
                        ((2, 6), (2, 4), (2, 2), (2, 5))
                    }
                }
                _ => return Err("Invalid castling move".to_string()),
            };
            // depends on side to move where piece is
            return Ok(Move {
                from: from_to_squares.0,
                from_piece: KING,
                from_colour: self.side_to_move,
                to: from_to_squares.1,
                to_piece: EMPTY,
                to_colour: EMPTY,

                notation_move: chess_move.clone(),
                promotion_to: None,
                en_passant: false,
                castle_from_to_square: Some((from_to_squares.2, from_to_squares.3)),
                sort_score: 0,
            });
        }

        // should be in format e2e3
        if chess_move.len() != 4 && chess_move.len() != 5 {
            return Err("not equal to 4 or 5 characters".to_string());
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
                // last character is for converting when pawn reaches last rank
                4 => {
                    if !char.is_alphabetic() && char.is_numeric() {
                        return Err(format!("fifth character must a piece type: {}", char));
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

        // handle last character as promotion
        if chess_move.len() == 5 {
            let promotion_to = match chess_move.chars().nth(4).unwrap() {
                'p' => 1,
                'n' => 2,
                'b' => 3,
                'r' => 4,
                'q' => 5,
                'k' => 6,
                _ => 0,
            };
            converted_move.promotion_to = Some(promotion_to);
        }

        converted_move.from_colour =
            self.colour_array[converted_move.from.0][converted_move.from.1];
        converted_move.to_colour = self.colour_array[converted_move.to.0][converted_move.to.1];

        converted_move.from_piece = self.board_array[converted_move.from.0][converted_move.from.1];
        converted_move.to_piece = self.board_array[converted_move.to.0][converted_move.to.1];

        // if piece is a pawn, check if en passant
        if converted_move.from_piece == PAWN
            && (converted_move.from.0 as i8 - converted_move.to.0 as i8).abs() == 2
        {
            converted_move.en_passant = true;
        }

        return Ok(converted_move);
    }

    pub fn is_square_empty(&self, square: &str) -> bool {
        let coordinates = convert_notation_to_location(square).unwrap_or((0, 0));
        return self.board_array[coordinates.0][coordinates.1] == EMPTY;
    }
    pub fn hash_board_state(&self) -> String {
        // take board state and generate a hash to use to compare uniqueness of position

        let mut hasher = DefaultHasher::new();
        for row in self.board_array.iter().skip(2).take(8) {
            for square in row.iter().skip(2).take(8) {
                hasher.write_i8(*square);
            }
        }
        for row in self.colour_array.iter().skip(2).take(8) {
            for square in row.iter().skip(2).take(8) {
                hasher.write_i8(*square);
            }
        }

        hasher.write_i8(self.side_to_move);

        return format!("{:x}", hasher.finish());
    }
    pub fn has_positions_repeated(&self) -> bool {
        // check if current hash appears two or more times in the history
        let current_hash = self.hash_board_state();

        //search history for this hash
        let count = self
            .hash_of_previous_positions
            .iter()
            .filter(|&x| *x == current_hash)
            .count();

        return if count >= 3 { true } else { false };
    }
    pub fn get_king_location(&self, side: i8) -> (usize, usize) {
        // find king for side
        // go through each piece on the board, by colour to only get moves for side to move.
        for (row_index, row) in self.board_array.iter().enumerate() {
            for (column_index, piece) in row.iter().enumerate() {
                let location = (row_index, column_index);
                let square_colour = &self.colour_array[row_index][column_index];

                if square_colour != &side || *piece != KING {
                    continue;
                }
                return location;
            }
        }
        panic!("King not found!");
    }
    pub fn is_side_in_check(&self, side: i8) -> bool {
        // how to check if side is in check

        // could generate all the moves for current state, for the opposing side
        // and if any attack the king square, its in check
        // this could also be used to move the king out of check
        let opposing_side = if side == WHITE { BLACK } else { WHITE };
        let king_location = self.get_king_location(side);
        let mut moves_attacking_king = generate_pseudo_legal_moves(&self.clone(), opposing_side);

        moves_attacking_king.retain(|x| x.to == king_location);

        if moves_attacking_king.len() > 0 {
            return true;
        }

        return false;
    }
}
pub fn convert_notation_to_location(chess_move: &str) -> Option<(usize, usize)> {
    let mut location = (0, 0);

    // get first two characters
    for (board_row_index, board_row) in BOARD_COORDINATES.iter().enumerate() {
        for (column_index, square_coordinate) in board_row.iter().enumerate() {
            // println!("{}", square_coordinate);
            if *square_coordinate == chess_move {
                location.0 = board_row_index;
                location.1 = column_index;
                break;
            }
        }
    }
    return Some(location);
}

pub fn convert_array_location_to_notation(
    from: (usize, usize),
    to: (usize, usize),
    promotion: Option<String>,
) -> String {
    let mut notation_move: String = Default::default();
    let start_location = BOARD_COORDINATES[from.0][from.1];
    let end_location = BOARD_COORDINATES[to.0][to.1];

    notation_move.push_str(start_location);
    notation_move.push_str(end_location);

    if promotion.is_some() {
        // println!("promoted {}", &promotion.clone().unwrap());
        notation_move.push_str(&promotion.unwrap().clone())
    }
    return notation_move;
}

/// prints the board from whites perspective in ascii to command line
pub fn print_board(board: &Board) {
    let mut row_string = String::new();

    for (row_index, row) in board.colour_array.iter().enumerate() {
        // println!("{:?}", row);
        for (column_index, _colour) in row.iter().enumerate() {
            // let location = (row_index, column_index);
            let square = board.board_array[row_index][column_index];
            if square == -1 {
                continue;
            }
            let piece_type = match square {
                1 => "P",
                2 => "N",
                3 => "B",
                4 => "R",
                5 => "Q",
                6 => "K",
                0 => " ",
                -1 => " ",
                _ => " ",
            };

            row_string.push_str("|");
            row_string.push_str(piece_type);
        }
        if !row_string.is_empty() {
            row_string.push_str("|");
        }

        println!("{}", row_string);
        row_string.clear();
    }

    // print colour board
    for (row_index, row) in board.colour_array.iter().enumerate() {
        // println!("{:?}", row);
        for (column_index, _colour) in row.iter().enumerate() {
            // let location = (row_index, column_index);
            let square = board.colour_array[row_index][column_index];
            if square == -1 {
                continue;
            }
            let colour = match square {
                1 => "W",
                2 => "B",
                0 => " ",
                _ => " ",
            };

            row_string.push_str("|");
            row_string.push_str(colour);
        }
        if !row_string.is_empty() {
            row_string.push_str("|");
        }

        println!("{}", row_string);
        row_string.clear();
    }
    //print all the attributes of the board to the command line
    println!("has the A1 Rook not moved: {}", board.a1_rook_not_moved);
    println!("has the H1 Rook not moved: {}", board.h1_rook_not_moved);
    println!("has the A8 Rook not moved: {}", board.a8_rook_not_moved);
    println!("has the H8 Rook not moved: {}", board.h8_rook_not_moved);

    println!(
        "is en passant possible: {}, location {:?}",
        board.en_passant,
        board.en_passant_location.unwrap_or((0, 0))
    );

    println!("has the king moved: {}", board.has_king_moved);
    println!("game ply: {}", board.ply);
    println!("to move: {}", board.side_to_move);
}
pub fn generate_pawn_moves(
    square: (usize, usize),
    side_to_generate_for: i8,
    board: &Board,
    // preceeding_move: Option<&Move>,
) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let mut blocked = false;
    let direction_of_pawns: i8 = match side_to_generate_for {
        1 => -1,
        2 => 1,
        _ => 0,
    };
    let enemy_color = if side_to_generate_for == 1 { 2 } else { 1 };

    // know if double jump allowed if from starting row
    let starting_row = if side_to_generate_for == 1 { 8 } else { 3 };
    let promotion_row = if side_to_generate_for == 1 { 3 } else { 8 };

    let (row, column) = square;

    // if square in front of pawn is not filled, can move there
    let index_of_square_in_front = if direction_of_pawns.is_negative() {
        row - 1
    } else {
        row + 1
    };
    let square_in_front = board.board_array[index_of_square_in_front][column];
    // let square_in_front_colour = board.colour_array[index_of_square_in_front][column];
    // if square not empty, return.
    if square_in_front != 0 {
        blocked = true;
    }

    if row == promotion_row && !blocked {
        for piece in [KNIGHT, BISHOP, ROOK, QUEEN] {
            moves.push(Move {
                from: square,
                from_piece: PAWN,
                to: (index_of_square_in_front, column),
                to_piece: square_in_front,
                from_colour: side_to_generate_for,
                to_colour: EMPTY,
                notation_move: convert_array_location_to_notation(
                    square,
                    (index_of_square_in_front, column),
                    Some(match piece {
                        1 => 'p'.to_string(),
                        2 => 'n'.to_string(),
                        3 => 'b'.to_string(),
                        4 => 'r'.to_string(),
                        5 => 'q'.to_string(),
                        6 => 'k'.to_string(),
                        0 => ' '.to_string(),
                        -1 => ' '.to_string(),
                        _ => ' '.to_string(),
                    }),
                ),
                en_passant: false,

                promotion_to: Some(piece),
                castle_from_to_square: None,
                sort_score: 0,
            });
        }
    } else if !blocked {
        moves.push(Move {
            from: square,
            from_piece: PAWN,
            to: (index_of_square_in_front, column),
            to_piece: square_in_front,
            from_colour: side_to_generate_for,
            to_colour: EMPTY,
            notation_move: convert_array_location_to_notation(
                square,
                (index_of_square_in_front, column),
                None,
            ),
            en_passant: false,

            promotion_to: None,
            castle_from_to_square: None,
            sort_score: 0,
        });
    }

    // if there is a square diagonnally forward from the pawn possessed by enemy
    let mut square_attack_colour = board.colour_array[index_of_square_in_front][column + 1];
    let mut square_attack_piece = board.board_array[index_of_square_in_front][column + 1];
    if square_attack_colour != side_to_generate_for
        && square_attack_colour != -1
        && square_attack_colour != 0
    {
        moves.push(Move {
            from: square,
            from_piece: PAWN,
            to: (index_of_square_in_front, column + 1),
            to_piece: square_attack_piece,
            from_colour: side_to_generate_for,
            to_colour: enemy_color,
            notation_move: convert_array_location_to_notation(
                square,
                (index_of_square_in_front, column + 1),
                None,
            ),
            en_passant: false,

            promotion_to: None,
            castle_from_to_square: None,
            sort_score: 0,
        });
    }
    // attack other diagonal
    square_attack_colour = board.colour_array[index_of_square_in_front][column - 1];
    square_attack_piece = board.board_array[index_of_square_in_front][column - 1];
    if square_attack_colour != side_to_generate_for
        && square_attack_colour != -1
        && square_attack_colour != 0
    {
        moves.push(Move {
            from: square,
            from_piece: PAWN,
            to: (index_of_square_in_front, column - 1),
            to_piece: square_attack_piece,
            from_colour: side_to_generate_for,
            to_colour: enemy_color,
            notation_move: convert_array_location_to_notation(
                square,
                (index_of_square_in_front, column - 1),
                None,
            ),
            en_passant: false,

            promotion_to: None,
            castle_from_to_square: None,
            sort_score: 0,
        });
    }

    if row == starting_row {
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
                from_piece: PAWN,
                to: (index_of_square_in_front, column),
                to_piece: square_in_front,
                from_colour: side_to_generate_for,
                to_colour: EMPTY,
                notation_move: convert_array_location_to_notation(
                    square,
                    (index_of_square_in_front, column),
                    None,
                ),

                promotion_to: None,
                en_passant: true,
                castle_from_to_square: None,
                sort_score: 0,
            });
        }
    }

    // if previous move was en pessant, and this pawn is on same row but off by one column, add en passant
    if let Some(move_info) = board.en_passant_location {
        if board.en_passant && move_info.0 == row && move_info.1.abs_diff(column) == 1 {
            // add en passant move v
            moves.push(Move {
                from: square,
                from_piece: PAWN,
                to: (index_of_square_in_front, move_info.1),
                to_piece: EMPTY,
                from_colour: side_to_generate_for,
                to_colour: EMPTY,
                notation_move: convert_array_location_to_notation(
                    square,
                    (index_of_square_in_front, move_info.1),
                    None,
                ),

                promotion_to: None,
                en_passant: false,
                castle_from_to_square: None,
                sort_score: 0,
            });
        }
    }

    return moves;
}
pub fn generate_knight_moves(
    square: (usize, usize),
    side_to_generate_for: i8,
    board: &Board,
) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    // let enemy_color = if side_to_generate_for == 1 { 2 } else { 1 };
    let (row, column) = square;
    let knight_move_steps: [(isize, isize); 8] = [
        (-2, -1),
        (-2, 1),
        (-1, -2),
        (-1, 2),
        (2, -1),
        (2, 1),
        (1, -2),
        (1, 2),
    ];
    // knight can move in any direction, two squares in one direciton, then 1 square in the other.

    // if populated by same colour piece, no move
    for (_index, move_steps) in knight_move_steps.iter().enumerate() {
        let square_move = board.colour_array[(row as isize + move_steps.0) as usize]
            [(column as isize + move_steps.1) as usize];

        if square_move == -1 || square_move == side_to_generate_for {
            continue;
        }
        //get to piece type
        let to_piece_type = board.board_array[(row as isize + move_steps.0) as usize]
            [(column as isize + move_steps.1) as usize];
        let to_square_colour = board.colour_array[(row as isize + move_steps.0) as usize]
            [(column as isize + move_steps.1) as usize];

        moves.push(Move {
            from: square,
            from_piece: KNIGHT,
            to: (
                (row as isize + move_steps.0) as usize,
                (column as isize + move_steps.1) as usize,
            ),
            to_piece: to_piece_type,
            from_colour: side_to_generate_for,
            to_colour: to_square_colour,
            notation_move: convert_array_location_to_notation(
                square,
                (
                    (row as isize + move_steps.0) as usize,
                    (column as isize + move_steps.1) as usize,
                ),
                None,
            ),
            en_passant: false,

            promotion_to: None,
            castle_from_to_square: None,
            sort_score: 0,
        });
    }
    return moves;
}

pub fn generate_bishop_moves(
    square: (usize, usize),
    side_to_generate_for: i8,
    board: &Board,
) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    // let _enemy_color = if side_to_generate_for == 1 { 2 } else { 1 };
    // from a bishops square, look along the 4 diagonals to see if it can move further
    let (row, column) = square;
    let bishop_move_directions: [(isize, isize); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    for direction in bishop_move_directions {
        for multiplier in 1..8 {
            let square_move = board.colour_array
                [(row as isize + direction.0 * multiplier) as usize]
                [(column as isize + direction.1 * multiplier) as usize];

            if square_move == -1 || square_move == side_to_generate_for {
                break;
            }
            //get to piece type
            let to_piece_type = board.board_array
                [(row as isize + direction.0 * multiplier) as usize]
                [(column as isize + direction.1 * multiplier) as usize];
            let to_square_colour = board.colour_array
                [(row as isize + direction.0 * multiplier) as usize]
                [(column as isize + direction.1 * multiplier) as usize];
            moves.push(Move {
                from: square,
                from_piece: BISHOP,
                to: (
                    (row as isize + direction.0 * multiplier) as usize,
                    (column as isize + direction.1 * multiplier) as usize,
                ),
                to_piece: to_piece_type,
                from_colour: side_to_generate_for,
                to_colour: to_square_colour,
                notation_move: convert_array_location_to_notation(
                    square,
                    (
                        (row as isize + direction.0 * multiplier) as usize,
                        (column as isize + direction.1 * multiplier) as usize,
                    ),
                    None,
                ),
                en_passant: false,

                promotion_to: None,
                castle_from_to_square: None,
                sort_score: 0,
            });

            // if captured a piece, stop multiplying and look in new direction
            if square_move != side_to_generate_for && square_move != EMPTY {
                break;
            }
        }
    }

    return moves;
}

pub fn generate_rook_moves(
    square: (usize, usize),
    side_to_generate_for: i8,
    board: &Board,
) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    // let _enemy_color = if side_to_generate_for == 1 { 2 } else { 1 };
    // from a rooks square, look along the 4 directions to see if it can move further
    let (row, column) = square;
    let rook_move_directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for direction in rook_move_directions {
        for multiplier in 1..8 {
            let square_move = board.colour_array
                [(row as isize + direction.0 * multiplier) as usize]
                [(column as isize + direction.1 * multiplier) as usize];

            if square_move == -1 || square_move == side_to_generate_for {
                break;
            }
            //get to piece type
            let to_piece_type = board.board_array
                [(row as isize + direction.0 * multiplier) as usize]
                [(column as isize + direction.1 * multiplier) as usize];
            let to_square_colour = board.colour_array
                [(row as isize + direction.0 * multiplier) as usize]
                [(column as isize + direction.1 * multiplier) as usize];

            moves.push(Move {
                from: square,
                from_piece: ROOK,
                to: (
                    (row as isize + direction.0 * multiplier) as usize,
                    (column as isize + direction.1 * multiplier) as usize,
                ),
                to_piece: to_piece_type,
                from_colour: side_to_generate_for,
                to_colour: to_square_colour,
                notation_move: convert_array_location_to_notation(
                    square,
                    (
                        (row as isize + direction.0 * multiplier) as usize,
                        (column as isize + direction.1 * multiplier) as usize,
                    ),
                    None,
                ),
                en_passant: false,

                promotion_to: None,
                castle_from_to_square: None,
                sort_score: 0,
            });

            // if captured a piece, stop multiplying and look in new direction
            if square_move != side_to_generate_for && square_move != EMPTY {
                break;
            }
        }
    }

    // how to include castling??

    return moves;
}

pub fn generate_queen_moves(
    square: (usize, usize),
    side_to_generate_for: i8,
    board: &Board,
) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    // let _enemy_color = if side_to_generate_for == 1 { 2 } else { 1 };
    // from a rooks square, look along the 4 directions to see if it can move further
    let (row, column) = square;
    let move_directions: [(isize, isize); 8] = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];
    for direction in move_directions {
        for multiplier in 1..8 {
            let square_move = board.colour_array
                [(row as isize + direction.0 * multiplier) as usize]
                [(column as isize + direction.1 * multiplier) as usize];

            if square_move == -1 || square_move == side_to_generate_for {
                break;
            }
            //get to piece type
            let to_piece_type = board.board_array
                [(row as isize + direction.0 * multiplier) as usize]
                [(column as isize + direction.1 * multiplier) as usize];
            let to_square_colour = board.colour_array
                [(row as isize + direction.0 * multiplier) as usize]
                [(column as isize + direction.1 * multiplier) as usize];
            moves.push(Move {
                from: square,
                from_piece: QUEEN,
                to: (
                    (row as isize + direction.0 * multiplier) as usize,
                    (column as isize + direction.1 * multiplier) as usize,
                ),
                to_piece: to_piece_type,
                from_colour: side_to_generate_for,
                to_colour: to_square_colour,
                notation_move: convert_array_location_to_notation(
                    square,
                    (
                        (row as isize + direction.0 * multiplier) as usize,
                        (column as isize + direction.1 * multiplier) as usize,
                    ),
                    None,
                ),
                en_passant: false,

                promotion_to: None,
                castle_from_to_square: None,
                sort_score: 0,
            });

            // if captured a piece, stop multiplying and look in new direction
            if square_move != side_to_generate_for && square_move != EMPTY {
                break;
            }
        }
    }

    return moves;
}

/// generate pseudo legal king moves,
/// this includes castling
/// this will check king is not being moved into check
pub fn generate_king_moves(
    square: (usize, usize),
    side_to_generate_for: i8,
    board: &Board,
) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    // let _enemy_color = if side_to_generate_for == 1 { 2 } else { 1 };
    let (row, column) = square;
    let king_move_directions: [(isize, isize); 8] = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    for direction in king_move_directions {
        let square_move = board.colour_array[(row as isize + direction.0) as usize]
            [(column as isize + direction.1) as usize];

        if square_move == -1 || square_move == side_to_generate_for {
            continue;
        }

        //get to piece type
        let to_piece_type = board.board_array[(row as isize + direction.0) as usize]
            [(column as isize + direction.1) as usize];
        let to_square_colour = board.colour_array[(row as isize + direction.0) as usize]
            [(column as isize + direction.1) as usize];
        moves.push(Move {
            from: square,
            from_piece: KING,
            to: (
                (row as isize + direction.0) as usize,
                (column as isize + direction.1) as usize,
            ),
            to_piece: to_piece_type,
            from_colour: side_to_generate_for,
            to_colour: to_square_colour,
            notation_move: convert_array_location_to_notation(
                square,
                (
                    (row as isize + direction.0) as usize,
                    (column as isize + direction.1) as usize,
                ),
                None,
            ),
            en_passant: false,

            promotion_to: None,
            castle_from_to_square: None,
            sort_score: 0,
        });

        // if captured a piece, stop multiplying and look in new direction
        if square_move != side_to_generate_for && square_move != EMPTY {
            continue;
        }
    }

    // castling
    if !board.has_king_moved {
        if side_to_generate_for == WHITE {
            if board.a1_rook_not_moved
                && board.is_square_empty("b1")
                && board.is_square_empty("c1")
                && board.is_square_empty("d1")
            {
                moves.push(Move {
                    from: square,
                    from_piece: KING,
                    to: (row, (column as isize - 2) as usize),
                    to_piece: EMPTY,
                    from_colour: side_to_generate_for,
                    to_colour: EMPTY,
                    notation_move: convert_array_location_to_notation(
                        square,
                        (row, (column as isize - 2) as usize),
                        None,
                    ),
                    en_passant: false,

                    promotion_to: None,
                    castle_from_to_square: None,
                    sort_score: 0,
                });
            }
            if board.h1_rook_not_moved && board.is_square_empty("f1") && board.is_square_empty("g1")
            {
                moves.push(Move {
                    from: square,
                    from_piece: KING,
                    to: (row, (column as isize + 2) as usize),
                    to_piece: EMPTY,
                    from_colour: side_to_generate_for,
                    to_colour: EMPTY,
                    notation_move: convert_array_location_to_notation(
                        square,
                        (row, (column as isize + 2) as usize),
                        None,
                    ),
                    en_passant: false,

                    promotion_to: None,
                    castle_from_to_square: None,
                    sort_score: 0,
                });
            }
        }

        if side_to_generate_for == BLACK {
            if board.a8_rook_not_moved
                && board.is_square_empty("b8")
                && board.is_square_empty("c8")
                && board.is_square_empty("d8")
            {
                moves.push(Move {
                    from: square,
                    from_piece: KING,
                    to: (row, (column as isize - 2) as usize),
                    to_piece: EMPTY,
                    from_colour: side_to_generate_for,
                    to_colour: EMPTY,
                    notation_move: convert_array_location_to_notation(
                        square,
                        (row, (column as isize - 2) as usize),
                        None,
                    ),
                    en_passant: false,

                    promotion_to: None,
                    castle_from_to_square: None,
                    sort_score: 0,
                });
            }
            if board.h8_rook_not_moved && board.is_square_empty("f8") && board.is_square_empty("g8")
            {
                moves.push(Move {
                    from: square,
                    from_piece: KING,
                    to: (row, (column as isize + 2) as usize),
                    to_piece: EMPTY,
                    from_colour: side_to_generate_for,
                    to_colour: EMPTY,
                    notation_move: convert_array_location_to_notation(
                        square,
                        (row, (column as isize + 2) as usize),
                        None,
                    ),
                    en_passant: false,

                    promotion_to: None,
                    castle_from_to_square: None,
                    sort_score: 0,
                });
            }
        }
    }

    return moves;
}

pub fn convert_alphabetic_to_piece(character: char) -> i8 {
    match character.to_ascii_uppercase() {
        'K' => KING,
        'Q' => QUEEN,
        'R' => ROOK,
        'B' => BISHOP,
        'N' => KNIGHT,
        'P' => PAWN,
        _ => -1,
    }
}
pub fn convert_fen_to_board(fen: &str) -> Board {
    // split by whitespace

    let mut board = Board::init();

    board.clear_board();

    board.h8_rook_not_moved = true;
    board.a8_rook_not_moved = true;
    board.h1_rook_not_moved = true;
    board.a1_rook_not_moved = true;

    // board is 12 x 12, but fen is 8x8. Need to convert
    // board starts at 2,2 to 2,10
    // and 10,2 and 10,10
    for (index, section) in fen.split_whitespace().enumerate() {
        match index {
            0 => {
                let mut current_row = 2;
                for (_char_index, characters) in section.split('/').enumerate() {
                    let mut current_column = 2;
                    for (_char_index, character) in characters.chars().enumerate() {
                        if current_column > 10 {
                            break;
                        }

                        if character.is_alphabetic() {
                            board.board_array[current_row][current_column] =
                                convert_alphabetic_to_piece(character);

                            if character.is_uppercase() {
                                board.colour_array[current_row][current_column] = 1;
                            } else {
                                board.colour_array[current_row][current_column] = 2;
                            }

                            current_column += 1;
                        }

                        if character.is_numeric() {
                            current_column += character.to_digit(10).unwrap() as usize;
                        }
                    }

                    current_row += 1;
                    if current_row > 10 {
                        break;
                    }
                }
            }

            1 => {
                // side to move
                match section {
                    "w" => board.side_to_move = 1,
                    "b" => board.side_to_move = 2,
                    "-" => {}
                    _ => todo!(), // probably panic
                }
            }
            2 => {
                for character in section.chars() {
                    match character {
                        'k' => board.h8_rook_not_moved = true,
                        'q' => board.a8_rook_not_moved = true,
                        'K' => board.h1_rook_not_moved = true,
                        'Q' => board.a1_rook_not_moved = true,
                        '-' => {}
                        _ => todo!(),
                    }
                }
            }
            3 => {
                let en_passant_column = section.chars().nth(0).unwrap();
                if en_passant_column == '-' {
                    continue;
                }
                board.en_passant = true;
                let en_passant_row = section.chars().nth(1).unwrap();
                match en_passant_row {
                    '3' => {
                        for (board_row_index, board_row) in BOARD_COORDINATES.iter().enumerate() {
                            for (column_index, square_coordinate) in board_row.iter().enumerate() {
                                if *square_coordinate
                                    == format!(
                                        "{}{}",
                                        en_passant_column,
                                        (en_passant_row.to_digit(10).unwrap() + 1)
                                    )
                                {
                                    board.en_passant_location =
                                        Some((board_row_index, column_index));

                                    break;
                                }
                            }
                        }
                    }
                    '6' => {
                        for (board_row_index, board_row) in BOARD_COORDINATES.iter().enumerate() {
                            for (column_index, square_coordinate) in board_row.iter().enumerate() {
                                if *square_coordinate
                                    == format!(
                                        "{}{}",
                                        (en_passant_row.to_digit(10).unwrap() - 1),
                                        en_passant_column
                                    )
                                {
                                    board.en_passant_location =
                                        Some((board_row_index, column_index));

                                    break;
                                }
                            }
                            break;
                        }
                    }
                    _ => todo!(),
                }
            } // en passant
            4 => {} // halfmove, i havent done this
            5 => board.ply = section.parse::<i8>().unwrap(),
            _ => {}
        }
    }

    // update running eval
    board.running_evaluation = evaluate(&board);

    return board;
}
pub fn get_piece_square_value(location: (usize, usize), piece_type: i8, colour: i8) -> i32 {
    if colour == WHITE {
        return match piece_type {
            PAWN => MG_PAWN_TABLE[location.0 - 2][location.1 - 2],
            KNIGHT => MG_KNIGHT_TABLE[location.0 - 2][location.1 - 2],
            BISHOP => MG_BISHOP_TABLE[location.0 - 2][location.1 - 2],
            ROOK => MG_ROOK_TABLE[location.0 - 2][location.1 - 2],
            QUEEN => MG_QUEEN_TABLE[location.0 - 2][location.1 - 2],
            KING => MG_KING_TABLE[location.0 - 2][location.1 - 2],
            _ => 0,
        };
    } else {
        if (8 - (location.0 as i8 - 2)) < 0 || (8 - (location.1 as i8 - 2)) < 0 {
            println!("{} {} {} {} ", location.0, location.1, piece_type, colour,);
        }
        return match piece_type {
            PAWN => MG_PAWN_TABLE[7 - (location.0 - 2)][7 - (location.1 - 2)],
            KNIGHT => MG_KNIGHT_TABLE[7 - (location.0 - 2)][7 - (location.1 - 2)],
            BISHOP => MG_BISHOP_TABLE[7 - (location.0 - 2)][7 - (location.1 - 2)],
            ROOK => MG_ROOK_TABLE[7 - (location.0 - 2)][7 - (location.1 - 2)],
            QUEEN => MG_QUEEN_TABLE[7 - (location.0 - 2)][7 - (location.1 - 2)],
            KING => MG_KING_TABLE[7 - (location.0 - 2)][7 - (location.1 - 2)],
            _ => 0,
        };
    }
}
pub fn evaluate(board: &Board) -> i32 {
    let mut score: i32 = 0;
    // go through each piece on the board, by colour to only get moves for side to move.
    for (row_index, row) in board.colour_array.iter().enumerate() {
        for (column_index, colour) in row.iter().enumerate() {
            let location = (row_index, column_index);
            let square = board.board_array[row_index][column_index];

            if square == -1 || square == EMPTY {
                continue;
            }
            // multiple score by eval_weights
            let mut score_for_piece_type = get_piece_square_value(location, square, *colour);

            println!(
                "score_for_piece_type: {}, square: {}, colour: {}",
                score_for_piece_type, square, colour
            );
            // if for other side, make negative.
            if colour != &board.side_to_move {
                score_for_piece_type *= -1;
            }
            // println!("{}, {}", square, colour);
            // println!("score_for_piece_type: {}", score_for_piece_type);
            score += score_for_piece_type as i32;
        }
    }
    return score;
    // count and addup pieces.
}
pub fn order_moves(moves: &mut Vec<Move>) {
    for i in 0..moves.len() {
        let move_to_score = moves.get_mut(i).unwrap();
        let value = MVV_LVA[move_to_score.to_piece as usize][move_to_score.from_piece as usize];
        move_to_score.sort_score += value as i32;
    }

    moves.sort_by(|a, b| a.sort_score.cmp(&b.sort_score));
    // sort moves
    // captures first
}
impl SearchEngine {
    pub fn new() -> Self {
        SearchEngine {
            nodes: 0,
            start: Instant::now(),
            move_nodes: Vec::new(),
        }
    }

    pub fn minimax(
        &mut self,
        board: &mut Board,
        depth: i8,
        maximizing_player: bool,
        mut alpha: i32,
        mut beta: i32,
    ) -> i32 {
        // the move needs to record its own evaluation
        if depth == 0 {
            let mut running_evaluation = board.running_evaluation;
            if board.side_to_move != WHITE {
                running_evaluation *= -1;
            }
            return running_evaluation;
        };

        // generate moves for current depth of board
        let mut moves_for_current_depth = generate_pseudo_legal_moves(board, board.side_to_move);
        order_moves(&mut moves_for_current_depth);
        if maximizing_player {
            let mut max_eval = -1000;
            for generated_move in moves_for_current_depth.iter() {
                board.make_move(generated_move);
                self.nodes += 1;

                let eval = self.minimax(board, depth - 1, false, alpha, beta);
                board.un_make_move(generated_move);
                max_eval = std::cmp::max(max_eval, eval);
                alpha = std::cmp::max(alpha, eval);
                if beta <= alpha {
                    break;
                }
            }
            return max_eval;

        // and best outcome for minimising player (enemy)
        } else {
            let mut min_eval = 1000;
            for generated_move in moves_for_current_depth.iter() {
                board.make_move(generated_move);

                let eval = self.minimax(board, depth - 1, true, alpha, beta);
                board.un_make_move(generated_move);
                min_eval = std::cmp::max(min_eval, eval);
                beta = std::cmp::min(beta, eval);
                if beta <= alpha {
                    break;
                }
            }
            return min_eval;
        }
    }

    pub fn search(&mut self, board: &mut Board, depth: i8) -> Vec<BestMoves> {
        let mut best_move = Move::default();
        let mut best_score = -1000;
        let mut best_moves = Vec::new();

        self.nodes = 0;
        self.start = Instant::now();

        // generate moves for current depth of board
        let mut moves_for_current_depth = generate_pseudo_legal_moves(board, board.side_to_move);
        order_moves(&mut moves_for_current_depth);
        for generated_move in moves_for_current_depth.iter() {
            board.make_move(generated_move);

            // print_board(board);
            let score = self.minimax(board, depth, true, i32::MIN, i32::MAX);

            if score > best_score {
                best_score = score;
                best_move = generated_move.clone();
                best_moves.push(BestMoves {
                    best_move,
                    best_score,
                });
            }

            board.un_make_move(generated_move);
            // print_board(board);
        }
        best_moves.sort_by(|a, b| b.best_score.cmp(&a.best_score));
        return best_moves;
    }

    pub fn perft(&mut self, board: &mut Board, depth: i8, first_call: bool) {
        if depth == 0 {
            return;
        }
        // for a given fen, generate all moves down to given depth
        // and print the number of nodes generated
        let moves_for_current_depth = generate_pseudo_legal_moves(board, board.side_to_move);

        // append root node and count
        for generated_move in moves_for_current_depth.iter() {
            board.make_move(generated_move);
            if !first_call {
                self.nodes += 1;
            } else {
                self.nodes = 0;
            }

            self.perft(board, depth - 1, false);

            board.un_make_move(generated_move);

            if first_call {
                // update root node here with number
                self.move_nodes.push(MoveNode {
                    move_notation: generated_move.notation_move.clone(),
                    nodes: self.nodes,
                });
            }
        }
        if first_call {
            self.nodes = 0;
            for move_node in self.move_nodes.iter() {
                self.nodes += move_node.nodes;
            }
        }
    }

    pub fn bench() {
        // run a series of fens,
        // output total nodes searched
        // are these legal moves?
    }
}

pub fn generate_pseudo_legal_moves(board: &Board, side_to_generate_for: i8) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    // go through each piece on the board, by colour to only get moves for side to move.
    for (row_index, row) in board.colour_array.iter().enumerate() {
        for (column_index, _colour) in row
            .iter()
            .enumerate()
            .filter(|(_a, b)| *b == &side_to_generate_for)
        {
            let location = (row_index, column_index);
            let square = board.board_array[row_index][column_index];

            let mut generated_moves = match square {
                1 => generate_pawn_moves(location, side_to_generate_for, board),
                2 => generate_knight_moves(location, side_to_generate_for, board),
                3 => generate_bishop_moves(location, side_to_generate_for, board),
                4 => generate_rook_moves(location, side_to_generate_for, board),
                5 => generate_queen_moves(location, side_to_generate_for, board),
                6 => generate_king_moves(location, side_to_generate_for, board),
                _ => vec![],
            };
            moves.append(&mut generated_moves);
        }
    }

    return moves;
}

// pub fn generate_moves(board: Board, side_to_move: i8) -> ();
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
