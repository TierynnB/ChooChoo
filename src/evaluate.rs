use crate::board::Board;
use crate::{constants::*, conversion, movegen::*};

pub fn evaluate(board: &Board, searching_side: i8) -> i32 {
    let mut score: i32 = 0;
    // go through each piece on the board, by colour to only get moves for side to move.
    for (row_index, row) in board.colour_array.iter().enumerate() {
        for (column_index, colour) in row.iter().enumerate() {
            // let location = (row_index, column_index);
            let square = board.get_piece((row_index, column_index));

            if square == EMPTY {
                continue;
            }

            let mut score_for_piece_type =
                conversion::get_piece_square_value((row_index, column_index), square, *colour);
            score_for_piece_type += match square {
                PAWN => 82,
                KNIGHT => 337,
                BISHOP => 365,
                ROOK => 525,
                QUEEN => 1025,
                KING => 10000,
                _ => 0,
            };
            // if for other side, make negative.
            if colour != &searching_side {
                score_for_piece_type *= -1;
            }

            score += score_for_piece_type as i32;
        }
    }
    return score;
    // count and addup pieces.
}
pub fn is_in_check(
    board: &Board,
    side_to_check: i8,
    aditional_square_to_check: Option<(usize, usize)>,
) -> bool {
    let opponent_colour = if side_to_check == WHITE { BLACK } else { WHITE };
    let king_location = board.get_king_location(side_to_check);

    for (row_index, row) in board.colour_array.iter().enumerate() {
        for (column_index, square_colour) in row.iter().enumerate() {
            if square_colour != &opponent_colour {
                continue;
            }
            let piece_type = board.get_piece((row_index, column_index));

            let outcome = is_attacked_by_piece_from_square(
                board,
                (row_index, column_index),
                piece_type,
                king_location,
                opponent_colour,
            );

            if outcome {
                return outcome;
            }

            if aditional_square_to_check.is_some() {
                let outcome = is_attacked_by_piece_from_square(
                    board,
                    (row_index, column_index),
                    piece_type,
                    aditional_square_to_check.unwrap(),
                    opponent_colour,
                );

                if outcome {
                    return outcome;
                }
            }
        }
    }
    return false;

    // in case of castling, check if they attack the intermediary squares.
}
// its given a square, and an enemy piece.
// and if the square is attacked by the enemy piece it returns true.
pub fn is_attacked_by_piece_from_square(
    board: &Board,
    square_from: (usize, usize),
    piece_type: i8,
    square_to: (usize, usize),
    side_to_generate_for: i8,
) -> bool {
    let difference_in_row = (square_to.0 as i32 - square_from.0 as i32).abs();
    let difference_in_column = (square_to.1 as i32 - square_from.1 as i32).abs();

    match piece_type {
        PAWN => {
            if difference_in_column > 1
                || difference_in_row > 1
                || (difference_in_column == 0 && difference_in_row == 1)
                || (difference_in_column == 1 && difference_in_row == 0)
            {
                return false;
            };
            for attack in get_pawn_attacks(square_from, side_to_generate_for, board) {
                if attack == square_to {
                    return true;
                }
            }
        }
        KNIGHT => {
            if difference_in_row > 2
                || difference_in_column > 2
                || (difference_in_row < 2 && difference_in_column < 2)
            {
                return false;
            };
            for attack in get_knight_attacks(square_from, side_to_generate_for, board) {
    
                if attack == square_to {
                   
                    return true;
                }
            }
        }
        BISHOP => {
            if difference_in_column == 0 || difference_in_row == 0 {
                return false;
            }

            if difference_in_row != difference_in_column {
                return false;
            }

            for attack in get_bishop_attacks(square_from, side_to_generate_for, board) {
                if attack == square_to {
                    return true;
                }
            }
        }
        ROOK => {
            if difference_in_row != 0 && difference_in_column != 0 {
                return false;
            }
            for attack in get_rook_attacks(square_from, side_to_generate_for, board) {
                if attack == square_to {
                    return true;
                }
            }
        }
        QUEEN => {
            if (difference_in_row != 0 && difference_in_column != 0)
                && (difference_in_row != difference_in_column)
            {
                return false;
            }
            for attack in get_queen_moves(square_from, side_to_generate_for, board) {
                if attack == square_to {
                    return true;
                }
            }
        }
        KING => {
            if difference_in_row > 1 || difference_in_column > 1 {
                return false;
            }
            for attack in get_king_attacks(square_from, side_to_generate_for, board) {
                if attack == square_to {
                    return true;
                }
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

#[cfg(test)]
mod tests {
    use crate::conversion;
    use crate::evaluate;
    #[test]
    fn evaluate_even_1() {
        let board = conversion::convert_fen_to_board(
            "rnbqkbnr/8/8/pppppppp/PPPPPPPP/8/8/RNBQKBNR w KQkq a6 0 9",
        );

        let eval = evaluate::evaluate(&board, 1);
        assert!(eval == 0, "Not around 0 eval");
    }
    #[test]
    fn evaluate_even_2() {
        let board = conversion::convert_fen_to_board(
            "2b2N1k/1p5P/1P2p2P/4Pp2/4pP2/1p2P2p/1P5p/2B2n1K w - - 0 1",
        );

        let eval = evaluate::evaluate(&board, 1);
        assert!(eval == 0, "Not around 0 eval");
    }
    #[test]
    fn evaluate_even_3() {
        let board = conversion::convert_fen_to_board(
            "b1nr1k1n/pp1Bp1p1/8/2p5/2P5/8/PP1bP1P1/B1NR1K1N w Qq - 0 7",
        );

        let eval = evaluate::evaluate(&board, 1);
        assert!(eval == 0, "Not around 0 eval");
    }
    #[test]
    fn evaluate_white_1() {
        let board = conversion::convert_fen_to_board("Q1k5/8/1K6/8/8/5B2/8/8 b - - 0 64");

        let eval = evaluate::evaluate(&board, 1);
        assert!(eval > 1000, "position does not favour white!");
    }

    #[test]
    fn evaluate_white_2() {
        let board =
            conversion::convert_fen_to_board("5k2/5p2/4pQp1/4P1Np/7P/6P1/4qP1K/8 b - - 10 41");

        let eval = evaluate::evaluate(&board, 1);
        assert!(eval > 100, "position does not favour white!");
    }
    #[test]
    fn evaluate_black_1() {
        let board = conversion::convert_fen_to_board("1k6/7p/4q3/3n4/3K4/2q5/7P/8 w - - 2 50");

        let eval = evaluate::evaluate(&board, 2);
        assert!(eval > 1000, "position does not favour black!");
    }

    // test black favoured position favour black
}
