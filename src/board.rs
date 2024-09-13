use crate::moves::Move;
use crate::{constants::*, conversion::*, evaluate};
use std::collections;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
#[derive(Clone)]
pub struct PlyData {
    pub ply: i32,
    pub side_to_move: i8,
    pub can_castle_a1: bool,
    pub can_castle_a8: bool,
    pub can_castle_h1: bool,
    pub can_castle_h8: bool,
    en_passant_location: Option<(usize, usize)>,
}

#[derive(Clone)]
pub struct Board {
    pub board_array: [[i8; 8]; 8],
    pub colour_array: [[i8; 8]; 8],
    pub white_attacks: [[bool; 8]; 8],
    pub black_attacks: [[bool; 8]; 8],

    pub can_castle_a1: bool,
    pub can_castle_a8: bool,
    pub can_castle_h1: bool,
    pub can_castle_h8: bool,

    pub en_passant_location: Option<(usize, usize)>,
    pub ply: i32,
    pub side_to_move: i8,
    pub hash_of_previous_positions: Vec<String>,
    pub ply_record: Vec<PlyData>,
    pub player_colour: i8,
    pub move_list: Vec<Move>,
}

impl Board {
    pub fn init() -> Board {
        // initialise the board with a new game
        let board_array = [
            [4, 2, 3, 5, 6, 3, 2, 4],
            [1, 1, 1, 1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1, 1, 1, 1],
            [4, 2, 3, 5, 6, 3, 2, 4],
        ];

        let colour_array = [
            [-1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1],
        ];

        let white_attacks = [[false; 8]; 8];

        let black_attacks = [[false; 8]; 8];

        return Board {
            board_array,
            colour_array,
            white_attacks,
            black_attacks,
            can_castle_a1: true,
            can_castle_a8: true,
            can_castle_h1: true,
            can_castle_h8: true,
            en_passant_location: None,
            ply: 0,
            side_to_move: 1,
            hash_of_previous_positions: Vec::new(),
            ply_record: Vec::new(),
            player_colour: 1,
            move_list: Vec::new(),
        };
    }

    pub fn get_piece(&self, location: (usize, usize)) -> i8 {
        return self.board_array[location.0][location.1];
    }
    pub fn get_piece_colour(&self, location: (usize, usize)) -> i8 {
        return self.colour_array[location.0 as usize][location.1 as usize];
    }
    pub fn set_piece_and_colour(&mut self, location: (usize, usize), piece: i8, colour: i8) {
        self.board_array[location.0][location.1] = piece;
        self.colour_array[location.0][location.1] = colour;
    }

    pub fn get_fen(&self) -> String {
        return "".to_string();
    }
    pub fn reset_board(&mut self) {
        self.board_array = [
            [4, 2, 3, 5, 6, 3, 2, 4],
            [1, 1, 1, 1, 1, 1, 1, 1],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1, 1, 1, 1],
            [4, 2, 3, 5, 6, 3, 2, 4],
        ];
        self.colour_array = [
            [-1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1, 1, 1, 1],
        ];

        // self.en_passant = false;
        self.en_passant_location = None;

        self.can_castle_a1 = false;
        self.can_castle_h1 = false;

        self.can_castle_a8 = false;
        self.can_castle_h8 = false;

        self.ply = 0;
        self.side_to_move = 1;
        self.hash_of_previous_positions = Vec::new();
        self.ply_record = Vec::new();
        self.move_list = Vec::new();
        self.player_colour = 1;
    }
    fn _clear_hash_of_previous_positions(&mut self) {
        self.hash_of_previous_positions = Vec::new();
    }

    fn add_hash_of_current_position(&mut self) {
        self.hash_of_previous_positions
            .push(self.hash_board_state());
    }

    pub fn clear_board(&mut self) {
        self.reset_board();

        // set all squares to empty
        self.board_array = [
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
        ];
        self.colour_array = [
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
        ];
    }

    pub fn make_move(&mut self, move_to_do: &Move) {
        self.move_list.push(move_to_do.clone());

        self.ply_record.push(PlyData {
            ply: self.ply,
            side_to_move: self.side_to_move,

            en_passant_location: self.en_passant_location,
            can_castle_a1: self.can_castle_a1,
            can_castle_a8: self.can_castle_a8,
            can_castle_h1: self.can_castle_h1,
            can_castle_h8: self.can_castle_h8,
        });

        // if enpassant was set at board level, and a pawn just moved to an empty square, behind the en passant locaiton
        // then remove the pawn at the en passant location.
        if self.en_passant_location.is_some() {
            if move_to_do.to.1 == self.en_passant_location.unwrap_or((0, 0)).1
                && move_to_do.from_piece == PAWN
                && move_to_do.to_piece == EMPTY
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
                self.set_piece_and_colour(self.en_passant_location.unwrap(), EMPTY, EMPTY)
            }
        }

        // set board level en passant information
        if move_to_do.en_passant {
            self.en_passant_location = Some(move_to_do.to);
        } else {
            self.en_passant_location = None;
        }

        // hanbdle promotion here.
        self.set_piece_and_colour(
            move_to_do.to,
            move_to_do.promotion_to.unwrap_or(move_to_do.from_piece),
            move_to_do.from_colour,
        );

        self.set_piece_and_colour(move_to_do.from, EMPTY, EMPTY);

        if move_to_do.from_piece == KING {
            if move_to_do.from_colour == WHITE {
                self.can_castle_a1 = false;
                self.can_castle_h1 = false;
            } else if move_to_do.from_colour == BLACK {
                self.can_castle_a8 = false;
                self.can_castle_h8 = false;
            }
        }

        // even if rook moves by itself, set rook not moved to false
        if move_to_do.from_piece == ROOK {
            if move_to_do.from_colour == WHITE {
                if move_to_do.from == (7, 0) {
                    self.can_castle_a1 = false;
                } else if move_to_do.from == (7, 7) {
                    self.can_castle_h1 = false;
                }
            } else if move_to_do.from_colour == BLACK {
                if move_to_do.from == (0, 0) {
                    self.can_castle_a8 = false;
                } else if move_to_do.from == (0, 7) {
                    self.can_castle_h8 = false;
                }
            }
        }

        // if rook is captured by another piece, cant castle anymore
        if move_to_do.to_piece == ROOK {
            if move_to_do.to_colour == WHITE {
                if move_to_do.to == (7, 0) {
                    self.can_castle_a1 = false;
                } else if move_to_do.to == (7, 7) {
                    self.can_castle_h1 = false;
                }
            } else if move_to_do.to_colour == BLACK {
                if move_to_do.to == (0, 0) {
                    self.can_castle_a8 = false;
                } else if move_to_do.to == (0, 7) {
                    self.can_castle_h8 = false;
                }
            }
        }

        // If current move castling, move rook too. king alreadyt moved
        if move_to_do.castle_from_to_square.is_some() {
            let castle_from_to_square = move_to_do.castle_from_to_square.unwrap();

            // this castleFromToSquare needs to be set to the rook movements, right now its the kings.
            // set destination square
            self.set_piece_and_colour(castle_from_to_square.1, ROOK, move_to_do.from_colour);

            self.set_piece_and_colour(castle_from_to_square.0, EMPTY, EMPTY);
        }

        // set side to move to opposite
        self.side_to_move = if self.side_to_move == WHITE {
            BLACK
        } else {
            WHITE
        };

        self.add_hash_of_current_position();

        self.ply += 1;
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
        self.move_list.pop();
        // the move should retain the original pieces in each square.
        self.set_piece_and_colour(chess_move.to, chess_move.to_piece, chess_move.to_colour);

        self.set_piece_and_colour(
            chess_move.from,
            chess_move.from_piece,
            chess_move.from_colour,
        );

        // set rooks back
        if chess_move.castle_from_to_square.is_some() {
            let castle_from_to_square = chess_move.castle_from_to_square.unwrap();

            // this castleFromToSquare needs to be set to the rook movements, right now its the kings.

            self.set_piece_and_colour(castle_from_to_square.0, ROOK, chess_move.from_colour);
            self.set_piece_and_colour(castle_from_to_square.1, EMPTY, EMPTY);
        }

        // does this handle enpassant
        self.hash_of_previous_positions.pop();

        self.ply -= 1;
        let enemy_colour = self.side_to_move;
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
            self.can_castle_a1 = previous_ply_data.can_castle_a1;
            self.can_castle_a8 = previous_ply_data.can_castle_a8;
            self.can_castle_h1 = previous_ply_data.can_castle_h1;
            self.can_castle_h8 = previous_ply_data.can_castle_h8;
            self.en_passant_location = previous_ply_data.en_passant_location;
            self.player_colour = previous_ply_data.side_to_move;
        }
        self.ply_record.pop();

        if self.en_passant_location.is_some() {
            if chess_move.to.1 == self.en_passant_location.unwrap_or((0, 0)).1
                && chess_move.from_piece == PAWN
                && chess_move.to_piece == EMPTY
                && self
                    .en_passant_location
                    .unwrap()
                    .1
                    .abs_diff(chess_move.from.1)
                    == 1
                && self.en_passant_location.unwrap().0 == chess_move.from.0
            {
                // the player is doing en passant.
                // so remove the pawn at the en passant location
                self.set_piece_and_colour(self.en_passant_location.unwrap(), PAWN, enemy_colour);
            }
        }
        // add pawn back from en passant
    }
    pub fn is_piece_type_on_board_for_side(&self, piece: i8, colour: i8) -> bool {
        // go through each piece on the board, by colour to only get moves for side to move.
        for (row_index, row) in self.board_array.iter().enumerate() {
            for (column_index, piece_type) in row.iter().enumerate() {
                if piece_type != &piece {
                    continue;
                }
                let piece_colour = self.get_piece_colour((row_index, column_index));
                if piece_colour != colour {
                    continue;
                }
                return true;
            }
        }
        return false;
    }
    pub fn convert_notation_to_move(&self, chess_move: String) -> Result<Move, String> {
        let mut converted_move = Move::default();

        // convert "e1g1" "e1c1" "e8g8" "e8c8" into O-O or O-O-O
        if ((chess_move == "e1g1" && self.can_castle_h1)
            || (chess_move == "e1c1" && self.can_castle_a1))
            || ((chess_move == "e8g8" && self.can_castle_h8)
                || (chess_move == "e8c8" && self.can_castle_h1))
        {
            // chess_move = match chess_move.as_str() {
            //     "e1g1" => "O-O".to_string(),
            //     "e1c1" => "O-O-O".to_string(),
            //     "e8g8" => "O-O".to_string(),
            //     "e8c8" => "O-O-O".to_string(),
            //     _ => return Err("Invalid castling move".to_string()),
            // };
            // castling is a special case
            let from_to_squares = match chess_move.as_str() {
                "e1g1" => ((7, 4), (7, 6), (7, 7), (7, 5)),
                "e8g8" => ((0, 4), (0, 6), (0, 7), (0, 5)),
                "e1c1" => ((7, 4), (7, 2), (7, 0), (7, 3)),
                "e8c8" => ((0, 4), (0, 2), (0, 9), (0, 3)),
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
                promotion_to: None,
                en_passant: false,
                castle_from_to_square: Some((from_to_squares.2, from_to_squares.3)),
                castling_intermediary_square: None,
                sort_score: 0,
                search_score: 0,
                illegal_move: false,
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

        converted_move.from_colour = self.get_piece_colour(converted_move.from);

        converted_move.to_colour = self.get_piece_colour(converted_move.to);

        converted_move.from_piece = self.get_piece(converted_move.from);
        converted_move.to_piece = self.get_piece(converted_move.to);

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

        return self.get_piece(coordinates) == EMPTY;
    }
    pub fn hash_board_state(&self) -> String {
        // take board state and generate a hash to use to compare uniqueness of position

        let mut hasher = DefaultHasher::new();
        for row in self.board_array.iter() {
            for square in row.iter() {
                hasher.write_i8(*square);
            }
        }
        for row in self.colour_array.iter() {
            for square in row.iter() {
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
    pub fn get_king_location(&self, side: i8) -> Option<(usize, usize)> {
        // find king for side
        // go through each piece on the board, by colour to only get moves for side to move.
        for (row_index, row) in self.board_array.iter().enumerate() {
            for (column_index, piece) in row.iter().enumerate() {
                if *piece != KING {
                    continue;
                }

                let location = (row_index, column_index);
                let square_colour = &self.get_piece_colour((row_index, column_index));

                if square_colour != &side {
                    continue;
                }
                return Some(location);
            }
        }

        return None;
    }
}
pub fn print_board(board: &Board) {
    let mut row_string = String::new();

    for (row_index, row) in board.colour_array.iter().enumerate() {
        // println!("{:?}", row);
        for (column_index, _colour) in row.iter().enumerate() {
            // let location = (row_index, column_index);
            let square = board.get_piece((row_index, column_index));

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
    println!(" ");
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
                -1 => "B",
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
    println!("can castle on the A1 Rook: {}", board.can_castle_a1);
    println!("can castle on the H1 Rook: {}", board.can_castle_h1);
    println!("can castle on the A8 Rook: {}", board.can_castle_a8);
    println!("can castle on the H8 Rook: {}", board.can_castle_h8);

    println!(
        "en passant plocation {:?}",
        // board.en_passant,
        board.en_passant_location.unwrap_or((0, 0))
    );

    println!("game ply: {}", board.ply);
    println!("to move: {}", board.side_to_move);
    println!(
        "is white in check:{}",
        evaluate::is_in_check(board, WHITE, None)
    );
    println!(
        "is black in check:{}",
        evaluate::is_in_check(board, BLACK, None)
    );
}
