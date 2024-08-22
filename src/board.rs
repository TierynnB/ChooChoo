use crate::moves::Move;
use crate::{constants::*, conversion::*, movegen::*};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
#[derive(Clone)]
pub struct PlyData {
    pub ply: i32,
    pub side_to_move: i8,
    has_king_moved: bool,
    a1_rook_not_moved: bool, // defaults to true
    a8_rook_not_moved: bool, // defaults to true
    h1_rook_not_moved: bool, // defaults to true
    h8_rook_not_moved: bool, // defaults to true
    en_passant: bool,
    en_passant_location: Option<(usize, usize)>,
    running_eval: i32,
}
#[derive(Clone)]
pub struct Board {
    pub board_array: [[i8; 12]; 12],
    pub colour_array: [[i8; 12]; 12],
    pub white_attacks: [[bool; 8]; 8],
    pub black_attacks: [[bool; 8]; 8],
    pub has_king_moved: bool,
    pub a1_rook_not_moved: bool, // defaults to true
    pub a8_rook_not_moved: bool, // defaults to true
    pub h1_rook_not_moved: bool, // defaults to true
    pub h8_rook_not_moved: bool, // defaults to true
    pub en_passant: bool,
    pub en_passant_location: Option<(usize, usize)>,
    pub ply: i32,
    pub side_to_move: i8,
    pub hash_of_previous_positions: Vec<String>,
    pub ply_record: Vec<PlyData>,
    pub running_evaluation: i32,
    pub player_colour: i8,
}

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

        let white_attacks = [[false; 8]; 8];

        let black_attacks = [[false; 8]; 8];

        return Board {
            board_array,
            colour_array,
            white_attacks,
            black_attacks,
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
    pub fn get_attacking_squares(&self, colour: i8) -> [[bool; 8]; 8] {
        if colour == WHITE {
            return self.white_attacks;
        } else {
            return self.black_attacks;
        }
    }

    pub fn get_piece(&self, location: (usize, usize)) -> i8 {
        return self.board_array[location.0][location.1];
    }
    pub fn get_piece_colour(&self, location: (usize, usize)) -> i8 {
        return self.colour_array[location.0][location.1];
    }
    pub fn set_attacking_squares_for_piece(
        &mut self,
        colour: i8,
        location: (usize, usize),
        piece_type: i8,
    ) {
        //
        let mut attacking_squares = self.get_attacking_squares(colour);
    }
    pub fn get_fen(&self) -> String {
        return "".to_string();
    }
    pub fn remove_attacking_squares_for_piece(
        &mut self,
        colour: i8,
        location: (usize, usize),
        piece_type: i8,
    ) {
        //
        let mut attacking_squares = self.get_attacking_squares(colour);
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
        let mut value = get_piece_square_value(location, piece_type, piece_colour);

        value += match piece_type {
            PAWN => 82,
            KNIGHT => 337,
            BISHOP => 365,
            ROOK => 525,
            QUEEN => 1025,
            _ => 0,
        };
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
        let mut value = get_piece_square_value(location, piece_type, piece_colour);
        value += match piece_type {
            PAWN => 82,
            KNIGHT => 337,
            BISHOP => 365,
            ROOK => 525,
            QUEEN => 1025,
            _ => 0,
        };

        if piece_colour == self.player_colour {
            self.running_evaluation += value;
        } else {
            self.running_evaluation -= value;
        }
    }
    pub fn make_move(&mut self, move_to_do: &Move) {
        // println!("hash length: {}", self.hash_of_previous_positions.len());
        // println!("ply_record length: {}", self.ply_record.len());

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
            running_eval: self.running_evaluation,
        });

        // if enpassant was set at board level, and a pawn just moved to an empty square, behind the en passant locaiton
        // then remove the pawn at the en passant location.
        if self.en_passant
            && move_to_do.from_piece == PAWN
            && move_to_do.to_piece == EMPTY
            && move_to_do.to.1 == self.en_passant_location.unwrap_or((0, 0)).1
            && self
                .en_passant_location
                .unwrap()
                .1
                .abs_diff(move_to_do.from.1)
                == 1
            && self.en_passant_location.unwrap().0 == move_to_do.from.0
        {
            // the player is doing en passant.
            // so remove the pawn at the en passant location
            self.board_array[self.en_passant_location.unwrap().0]
                [self.en_passant_location.unwrap().1] = EMPTY;

            self.colour_array[self.en_passant_location.unwrap().0]
                [self.en_passant_location.unwrap().1] = EMPTY;
        }

        // set board level en passant information
        self.en_passant = move_to_do.en_passant;
        self.en_passant_location = Some(move_to_do.to);

        // hanbdle promotion here.

        self.board_array[move_to_do.to.0][move_to_do.to.1] = move_to_do
            .promotion_to
            .unwrap_or(self.board_array[move_to_do.from.0][move_to_do.from.1]);

        self.board_array[move_to_do.from.0][move_to_do.from.1] = 0;

        self.colour_array[move_to_do.to.0][move_to_do.to.1] =
            self.colour_array[move_to_do.from.0][move_to_do.from.1];

        self.colour_array[move_to_do.from.0][move_to_do.from.1] = 0;
        //print rook info
        // println!("{}", move_to_do.from_piece);
        // println!("{}", move_to_do.from.0);
        // println!("{}", move_to_do.from.1);

        // need to know if castling
        if move_to_do.from_piece == KING
            && (self.a1_rook_not_moved
                || self.a8_rook_not_moved
                || self.h1_rook_not_moved
                || self.h8_rook_not_moved)
        {
            // check if king moved from e1 or h1 or a8 or h8
            if move_to_do.from == (9, 6) && move_to_do.to == (9, 4) {
                self.a1_rook_not_moved = false;
            } else if move_to_do.from == (9, 6) && move_to_do.to == (9, 8) {
                self.h1_rook_not_moved = false;
            } else if move_to_do.from == (2, 6) && move_to_do.to == (2, 4) {
                self.a8_rook_not_moved = false;
            } else if move_to_do.from == (2, 6) && move_to_do.to == (2, 8) {
                self.h8_rook_not_moved = false;
            }
        }

        // even if rook moves by itself, set rook not moved to false
        if move_to_do.from_piece == ROOK
            && (self.a1_rook_not_moved
                || self.a8_rook_not_moved
                || self.h1_rook_not_moved
                || self.h8_rook_not_moved)
        {
            // check if king moved from e1 or h1 or a8 or h8
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

        // need to know if a rook has been captured
        // need to know if castling
        if move_to_do.to_piece == ROOK
            && (self.a1_rook_not_moved
                || self.a8_rook_not_moved
                || self.h1_rook_not_moved
                || self.h8_rook_not_moved)
        {
            // check if rook moved from a1 or h1 or a8 or h8
            if move_to_do.to == (9, 2) {
                self.a1_rook_not_moved = false;
            } else if move_to_do.to == (9, 9) {
                self.h1_rook_not_moved = false;
            } else if move_to_do.to == (2, 2) {
                self.a8_rook_not_moved = false;
            } else if move_to_do.to == (2, 9) {
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
            self.running_evaluation = previous_ply_data.running_eval;
            self.player_colour = previous_ply_data.side_to_move;
            // self.hash_of_previous_positions = previous_ply_data.;
            // self.ply_record = previous_ply_data.ply_record;
        }
        self.ply_record.pop();
        //update evaluation
        // self.add_piece_to_evaluation(chess_move.to, chess_move.from_piece, chess_move.to_colour);
        // //update evaluation

        // // if promotion, add promotion piece
        // if chess_move.promotion_to.is_some() {
        //     self.remove_piece_from_evaluation(
        //         chess_move.to,
        //         chess_move.promotion_to.unwrap(),
        //         chess_move.from_colour,
        //     );
        // } else {
        //     self.remove_piece_from_evaluation(
        //         chess_move.to,
        //         chess_move.from_piece,
        //         chess_move.from_colour,
        //     );
        // }
        // self.add_piece_to_evaluation(
        //     chess_move.from,
        //     chess_move.from_piece,
        //     chess_move.from_colour,
        // );
    }

    pub fn convert_notation_to_move(&self, mut chess_move: String) -> Result<Move, String> {
        let mut converted_move = Move::default();

        // convert "e1g1" "e1c1" "e8g8" "e8c8" into O-O or O-O-O
        if !self.has_king_moved
            && ((chess_move == "e1g1" && self.h1_rook_not_moved)
                || (chess_move == "e1c1" && self.a1_rook_not_moved)
                || (chess_move == "e8g8" && self.h8_rook_not_moved)
                || (chess_move == "e8c8" && self.a8_rook_not_moved))
        {
            chess_move = match chess_move.as_str() {
                "e1g1" => "O-O".to_string(),
                "e1c1" => "O-O-O".to_string(),
                "e8g8" => "O-O".to_string(),
                "e8c8" => "O-O-O".to_string(),
                _ => return Err("Invalid castling move".to_string()),
            };
        }

        // castling is a special case
        if !self.has_king_moved && (chess_move == "O-O" || chess_move == "O-O-O") {
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
        if chess_move.len() < 4 || chess_move.len() > 5 {
            // println!("{}", chess_move.len());
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
        let mut moves_attacking_king = generate_pseudo_legal_moves(&self.clone(), opposing_side, 1);

        moves_attacking_king.retain(|x| x.to == king_location);

        if moves_attacking_king.len() > 0 {
            return true;
        }

        return false;
    }
}
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
