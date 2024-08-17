use crate::board::Board;
use crate::{constants::*, conversion::*};

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
