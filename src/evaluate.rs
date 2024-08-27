use crate::board::Board;
use crate::{constants::*, movegen::*};

pub fn evaluate(board: &Board) -> i32 {
    let mut score: i32 = 0;
    // go through each piece on the board, by colour to only get moves for side to move.
    for (row_index, row) in board.colour_array.iter().enumerate() {
        for (column_index, colour) in row.iter().enumerate() {
            // let location = (row_index, column_index);
            let square = board.get_piece((row_index, column_index));

            if square == EMPTY {
                continue;
            }

            // let mut score_for_piece_type = get_piece_square_value(location, square, *colour);
            let mut score_for_piece_type = match square {
                PAWN => 82,
                KNIGHT => 337,
                BISHOP => 365,
                ROOK => 525,
                QUEEN => 1025,
                _ => 0,
            };
            // if for other side, make negative.
            if colour != &board.side_to_move {
                score_for_piece_type *= -1;
            }

            score += score_for_piece_type as i32;

            // debug!("row {} column {} square {} {} {}", row_index, column_index, square, score_for_piece_type);
        }
    }
    return score;
    // count and addup pieces.
}
pub fn is_in_check(board: &Board, side_to_check: i8) -> bool {
    let opponent_colour = if side_to_check == WHITE { BLACK } else { WHITE };

    // get sides king location
    let king_location = board.get_king_location(side_to_check);

    if generate_pseudo_legal_moves(board, opponent_colour, 1)
        .iter()
        .any(|x| x.to == king_location)
    {
        return true;
    }

    return false;
}

pub fn is_attacked_by_piece_from_square(
    board: &Board,
    square_from: (usize, usize),
    pieceType: i8,
    square_to: (usize, usize),
) -> bool {
    let difference_in_row = (square_to.0 as i32 - square_from.0 as i32).abs();
    let difference_in_column = (square_to.1 as i32 - square_from.1 as i32).abs();

    match pieceType {
        PAWN => {
            if difference_in_column > 1 || difference_in_row > 1 {
                return false;
            };

            // check if pawn is diagonal from the square_to.

            // include en passant.
        }
        KNIGHT => {
            if difference_in_row > 3 || difference_in_column > 3 {
                return false;
            };
        }
        BISHOP => {
            if difference_in_row % difference_in_column != 0 {
                return false;
            }
        }
        ROOK => {
            if difference_in_row != 0 && difference_in_column != 0 {
                return false;
            }
        }
        QUEEN => {
            if (difference_in_row != 0 && difference_in_column != 0)
                && (difference_in_row % difference_in_column != 0)
            {
                return false;
            }
        }
        KING => {
            if difference_in_row > 1 || difference_in_column > 1 {
                return false;
            }
        }
        _ => return false,
    }
    // for a given piece, on square from, does it attack the square_to?
    // can easily ignore pawns, kings, and knights outside a certain range

    // ignore rooks on wrong rank / file

    // check if bishop or queen on same diagonal.

    // is attack blocked by another piece (of any colour)

    // if queen not attacking along diagonal, check on correct file for rook-like attack.

    return false;
}
