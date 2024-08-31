use crate::board::Board;
use crate::constants;
// use crate::evaluate;
pub fn convert_fen_to_board(fen: &str) -> Board {
    // implementation here

    // split by whitespace

    let mut board = Board::init();

    board.clear_board();

    board.can_castle_a1 = false;
    board.can_castle_a8 = false;
    board.can_castle_h1 = false;
    board.can_castle_h8 = false;

    // board is 12 x 12, but fen is 8x8. Need to convert
    // board starts at 2,2 to 2,10
    // and 10,2 and 10,10
    for (index, section) in fen.split_whitespace().enumerate() {
        match index {
            0 => {
                let mut current_row = 0;
                for (_char_index, characters) in section.split('/').enumerate() {
                    let mut current_column = 0;
                    for (_char_index, character) in characters.chars().enumerate() {
                        if current_column > 7 {
                            break;
                        }

                        if character.is_alphabetic() {
                            // board.board_array[current_row][current_column] =
                            //     convert_alphabetic_to_piece(character);
                            let piece_colour = if character.is_uppercase() { 1 } else { 2 };

                            board.set_piece_and_colour(
                                (current_row, current_column),
                                convert_alphabetic_to_piece(character),
                                piece_colour,
                            );
                            current_column += 1;
                        }

                        if character.is_numeric() {
                            current_column += character.to_digit(10).unwrap() as usize;
                        }
                    }

                    current_row += 1;
                    if current_row > 7 {
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
                        'k' => board.can_castle_h8 = true,
                        'q' => board.can_castle_a8 = true,
                        'K' => board.can_castle_h1 = true,
                        'Q' => board.can_castle_a1 = true,
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
                // board.en_passant = true;
                let en_passant_row = section.chars().nth(1).unwrap();
                match en_passant_row {
                    '3' => {
                        for (board_row_index, board_row) in
                            constants::BOARD_COORDINATES.iter().enumerate()
                        {
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
                        for (board_row_index, board_row) in
                            constants::BOARD_COORDINATES.iter().enumerate()
                        {
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
            5 => board.ply = section.parse::<i32>().unwrap(),
            _ => {}
        }
    }

    return board;
}

pub fn get_piece_square_value(location: (usize, usize), piece_type: i8, colour: i8) -> i32 {
    if colour == constants::WHITE {
        return match piece_type {
            constants::PAWN => constants::MG_PAWN_TABLE[location.0 - 2][location.1 - 2],
            constants::KNIGHT => constants::MG_KNIGHT_TABLE[location.0 - 2][location.1 - 2],
            constants::BISHOP => constants::MG_BISHOP_TABLE[location.0 - 2][location.1 - 2],
            constants::ROOK => constants::MG_ROOK_TABLE[location.0 - 2][location.1 - 2],
            constants::QUEEN => constants::MG_QUEEN_TABLE[location.0 - 2][location.1 - 2],
            constants::KING => constants::MG_KING_TABLE[location.0 - 2][location.1 - 2],
            _ => 0,
        };
    } else {
        return match piece_type {
            constants::PAWN => constants::MG_PAWN_TABLE[7 - (location.0 - 2)][7 - (location.1 - 2)],
            constants::KNIGHT => {
                constants::MG_KNIGHT_TABLE[7 - (location.0 - 2)][7 - (location.1 - 2)]
            }
            constants::BISHOP => {
                constants::MG_BISHOP_TABLE[7 - (location.0 - 2)][7 - (location.1 - 2)]
            }
            constants::ROOK => constants::MG_ROOK_TABLE[7 - (location.0 - 2)][7 - (location.1 - 2)],
            constants::QUEEN => {
                constants::MG_QUEEN_TABLE[7 - (location.0 - 2)][7 - (location.1 - 2)]
            }
            constants::KING => constants::MG_KING_TABLE[7 - (location.0 - 2)][7 - (location.1 - 2)],
            _ => 0,
        };
    }
}

pub fn convert_alphabetic_to_piece(character: char) -> i8 {
    match character.to_ascii_uppercase() {
        'K' => constants::KING,
        'Q' => constants::QUEEN,
        'R' => constants::ROOK,
        'B' => constants::BISHOP,
        'N' => constants::KNIGHT,
        'P' => constants::PAWN,
        _ => -1,
    }
}

pub fn convert_array_location_to_notation(
    from: (usize, usize),
    to: (usize, usize),
    promotion: Option<String>,
) -> String {
    let mut notation_move: String = Default::default();
    if from.0 > 7 || from.1 > 7 || to.0 > 7 || to.1 > 7 {
        return notation_move;
    }

    let start_location = constants::BOARD_COORDINATES[from.0][from.1];
    let end_location = constants::BOARD_COORDINATES[to.0][to.1];

    notation_move.push_str(start_location);
    notation_move.push_str(end_location);

    if promotion.is_some() {
        // println!("promoted {}", &promotion.clone().unwrap());
        notation_move.push_str(&promotion.unwrap().clone())
    }
    return notation_move;
}
pub fn convert_notation_to_location(chess_move: &str) -> Option<(usize, usize)> {
    let mut location = (0, 0);

    // get first two characters
    for (board_row_index, board_row) in constants::BOARD_COORDINATES.iter().enumerate() {
        for (column_index, square_coordinate) in board_row.iter().enumerate() {
            if *square_coordinate == chess_move {
                location.0 = board_row_index;
                location.1 = column_index;
                break;
            }
        }
    }
    return Some(location);
}
/// convert current board state into fen
pub fn convert_board_to_fen(_board: &Board) -> String {
    let fen_string = String::new();

    // loop over each rank, adding to fen string

    // then add the color (w / b) whose turn it is

    // then add the castling rights (KQkq)

    // then add the en passant square (e3) - the square behind the pawn

    // then add the halfmove clock (h3) - how many halfmoves since the last capture or pawn advancement

    return fen_string;
}
