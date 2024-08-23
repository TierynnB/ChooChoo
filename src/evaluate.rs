use crate::board::Board;
use crate::{constants::*, conversion::*, movegen::*};

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
