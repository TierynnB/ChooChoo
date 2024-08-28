use crate::{board::*, constants::*, conversion::*, moves::*};
use std::vec;

pub struct CastlingSquaresAttacked {
    pub d1_attacked: bool,
    pub c1_attacked: bool,
    pub e1_attacked: bool,
    pub f1_attacked: bool,
    pub g1_attacked: bool,
    pub d8_attacked: bool,
    pub c8_attacked: bool,
    pub e8_attacked: bool,
    pub f8_attacked: bool,
    pub g8_attacked: bool,
}

impl Default for CastlingSquaresAttacked {
    fn default() -> Self {
        return CastlingSquaresAttacked {
            d1_attacked: false,
            c1_attacked: false,
            e1_attacked: false,
            f1_attacked: false,
            g1_attacked: false,
            d8_attacked: false,
            c8_attacked: false,
            e8_attacked: false,
            f8_attacked: false,
            g8_attacked: false,
        };
    }
}
pub fn get_pawn_attacks(
    square: (usize, usize),
    side_to_generate_for: i8,
    board: &Board,
) -> Vec<(usize, usize)> {
    let mut attacking_squares: Vec<(usize, usize)> = vec![];
    // does not include en passant

    let (row, column) = square;
    let direction_of_pawns: isize = match side_to_generate_for {
        1 => -1,
        2 => 1,
        _ => 0,
    };
    let pawn_attack_steps: [(isize, isize); 2] =
        [(direction_of_pawns, 1), (direction_of_pawns, -1)];

    // if populated by same colour piece, no move
    for (_index, move_steps) in pawn_attack_steps.iter().enumerate() {
        // if out of bounds, stop
        if (row as isize + move_steps.0) < 0
            || (row as isize + move_steps.0) > 7
            || (column as isize + move_steps.1) < 0
            || (column as isize + move_steps.1) > 7
        {
            continue;
        }
        let to_square_colour = board.get_piece_colour((
            (row as isize + move_steps.0) as usize,
            (column as isize + move_steps.1) as usize,
        ));
        if to_square_colour == side_to_generate_for || to_square_colour == EMPTY {
            continue;
        }

        attacking_squares.push((
            (row as isize + move_steps.0) as usize,
            (column as isize + move_steps.1) as usize,
        ))
    }

    return attacking_squares;
}
pub fn generate_pawn_moves(
    square: (usize, usize),
    side_to_generate_for: i8,
    board: &Board,
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
    let starting_row = if side_to_generate_for == 1 { 6 } else { 1 };

    // second rank for promotion
    let promotion_row = if side_to_generate_for == 1 { 1 } else { 6 };

    let (row, column) = square;

    // if in the zero rank, cant not exist and would be promoted.
    if row == 0 && direction_of_pawns == -1 {
        return moves;
    }
    // if square in front of pawn is not filled, can move there
    let index_of_square_in_front = (row as i8 + direction_of_pawns) as usize;

    let square_in_front = board.get_piece((index_of_square_in_front, column));

    if square_in_front != 0 {
        blocked = true;
    };

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
                castling_intermediary_square: None,
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
            castling_intermediary_square: None,
            sort_score: 0,
        });
    }

    // if there is a square diagonally forward from the pawn possessed by enemy
    let attack_squares = get_pawn_attacks(square, side_to_generate_for, board);

    for attack_square in attack_squares {
        let to_piece_type = board.get_piece((attack_square.0, attack_square.1));
        let to_square_colour = board.get_piece_colour((attack_square.0, attack_square.1));

        // if in the promotion row, you must also promote
        if row == promotion_row {
            for piece in [KNIGHT, BISHOP, ROOK, QUEEN] {
                moves.push(Move {
                    from: square,
                    from_piece: PAWN,
                    to: attack_square,
                    to_piece: to_piece_type,
                    from_colour: side_to_generate_for,
                    to_colour: to_square_colour,
                    notation_move: convert_array_location_to_notation(
                        square,
                        attack_square,
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
                    castling_intermediary_square: None,
                    sort_score: 0,
                });
            }
        } else {
            moves.push(Move {
                from: square,
                from_piece: PAWN,
                to: attack_square,
                to_piece: to_piece_type,
                from_colour: side_to_generate_for,
                to_colour: to_square_colour,
                notation_move: convert_array_location_to_notation(square, attack_square, None),
                en_passant: false,

                promotion_to: None,
                castle_from_to_square: None,
                castling_intermediary_square: None,
                sort_score: 0,
            });
        }
    }
    if row == starting_row {
        // if pawn on its starting square, can move two
        let index_of_square_in_front = if direction_of_pawns.is_negative() {
            row - 2
        } else {
            row + 2
        };
        let square_in_front = board.get_piece((index_of_square_in_front, column)); //board.board_array[index_of_square_in_front][column];

        // if square not empty, return.
        if square_in_front == 0 && !blocked {
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
                castling_intermediary_square: None,
                sort_score: 0,
            });
        }
    }

    // if previous move was en passant, and this pawn is on same row but off by one column, add en passant
    if let Some(move_info) = board.en_passant_location {
        if board.en_passant_location.is_some()
            && move_info.0 == row
            && move_info.1.abs_diff(column) == 1
        {
            // add en passant move v
            moves.push(Move {
                from: square,
                from_piece: PAWN,
                to: (index_of_square_in_front as usize, move_info.1),
                to_piece: EMPTY,
                from_colour: side_to_generate_for,
                to_colour: EMPTY,
                notation_move: convert_array_location_to_notation(
                    square,
                    (index_of_square_in_front as usize, move_info.1),
                    None,
                ),

                promotion_to: None,
                en_passant: false,
                castling_intermediary_square: None,
                castle_from_to_square: None,
                sort_score: 0,
            });
        }
    }

    return moves;
}
pub fn get_knight_attacks(
    square: (usize, usize),
    side_to_generate_for: i8,
    board: &Board,
) -> Vec<(usize, usize)> {
    let mut attacking_squares: Vec<(usize, usize)> = vec![];

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
        // if out of bounds, stop
        if (row as isize + move_steps.0) < 0
            || (row as isize + move_steps.0) > 7
            || (column as isize + move_steps.1) < 0
            || (column as isize + move_steps.1) > 7
        {
            continue;
        }
        let to_square_colour = board.get_piece_colour((
            (row as isize + move_steps.0) as usize,
            (column as isize + move_steps.1) as usize,
        ));
        if to_square_colour == side_to_generate_for {
            continue;
        }

        attacking_squares.push((
            (row as isize + move_steps.0) as usize,
            (column as isize + move_steps.1) as usize,
        ))
    }

    return attacking_squares;
}
pub fn generate_knight_moves(
    square: (usize, usize),
    side_to_generate_for: i8,
    board: &Board,
) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    let attack_squares = get_knight_attacks(square, side_to_generate_for, board);

    for attack_square in attack_squares {
        let to_piece_type = board.get_piece((attack_square.0, attack_square.1));
        let to_square_colour = board.get_piece_colour((attack_square.0, attack_square.1));

        moves.push(Move {
            from: square,
            from_piece: KNIGHT,
            to: (attack_square.0, attack_square.1),
            to_piece: to_piece_type,
            from_colour: side_to_generate_for,
            to_colour: to_square_colour,
            notation_move: convert_array_location_to_notation(
                square,
                (attack_square.0, attack_square.1),
                None,
            ),
            en_passant: false,

            promotion_to: None,
            castle_from_to_square: None,
            castling_intermediary_square: None,
            sort_score: 0,
        });
    }
    return moves;
}
pub fn get_bishop_attacks(
    square: (usize, usize),
    side_to_generate_for: i8,
    board: &Board,
) -> Vec<(usize, usize)> {
    let mut attacking_squares: Vec<(usize, usize)> = vec![];

    // from a bishops square, look along the 4 diagonals to see if it can move further
    let (row, column) = square;
    let bishop_move_directions: [(isize, isize); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    for direction in bishop_move_directions {
        for multiplier in 1..8 {
            if (row as isize + direction.0 * multiplier) < 0
                || (row as isize + direction.0 * multiplier) > 7
                || (column as isize + direction.1 * multiplier) < 0
                || (column as isize + direction.1 * multiplier) > 7
            {
                continue;
            }

            let to_square_colour = board.get_piece_colour((
                (row as isize + direction.0 * multiplier) as usize,
                (column as isize + direction.1 * multiplier) as usize,
            ));

            if to_square_colour == side_to_generate_for {
                break;
            }

            attacking_squares.push((
                (row as isize + direction.0 * multiplier) as usize,
                (column as isize + direction.1 * multiplier) as usize,
            ));
            // if captured a piece, stop multiplying and look in new direction
            if to_square_colour != side_to_generate_for && to_square_colour != EMPTY {
                break;
            }
        }
    }

    return attacking_squares;
}
pub fn generate_bishop_moves(
    square: (usize, usize),
    side_to_generate_for: i8,
    board: &Board,
) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    let attack_squares = get_bishop_attacks(square, side_to_generate_for, board);

    for attack_square in attack_squares {
        let to_piece_type = board.get_piece((attack_square.0, attack_square.1));
        let to_square_colour = board.get_piece_colour((attack_square.0, attack_square.1));

        moves.push(Move {
            from: square,
            from_piece: BISHOP,
            to: (attack_square.0, attack_square.1),
            to_piece: to_piece_type,
            from_colour: side_to_generate_for,
            to_colour: to_square_colour,
            notation_move: convert_array_location_to_notation(
                square,
                (attack_square.0, attack_square.1),
                None,
            ),
            en_passant: false,

            promotion_to: None,
            castle_from_to_square: None,
            castling_intermediary_square: None,
            sort_score: 0,
        });
    }

    return moves;
}
pub fn get_rook_attacks(
    square: (usize, usize),
    side_to_generate_for: i8,
    board: &Board,
) -> Vec<(usize, usize)> {
    let mut attacking_squares: Vec<(usize, usize)> = vec![];
    // from a rooks square, look along the 4 directions to see if it can move further
    let (row, column) = square;
    let rook_move_directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for direction in rook_move_directions {
        for multiplier in 1..8 {
            // if out of bounds, stop
            if (row as isize + direction.0 * multiplier) < 0
                || (row as isize + direction.0 * multiplier) > 7
                || (column as isize + direction.1 * multiplier) < 0
                || (column as isize + direction.1 * multiplier) > 7
            {
                continue;
            }
            let to_square_colour = board.get_piece_colour((
                (row as isize + direction.0 * multiplier) as usize,
                (column as isize + direction.1 * multiplier) as usize,
            ));

            if to_square_colour == side_to_generate_for {
                break;
            }

            attacking_squares.push((
                (row as isize + direction.0 * multiplier) as usize,
                (column as isize + direction.1 * multiplier) as usize,
            ));

            // if captured a piece, stop multiplying and look in new direction
            if to_square_colour != side_to_generate_for && to_square_colour != EMPTY {
                break;
            }
        }
    }

    return attacking_squares;
}
pub fn generate_rook_moves(
    square: (usize, usize),
    side_to_generate_for: i8,
    board: &Board,
) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    let attack_squares = get_rook_attacks(square, side_to_generate_for, board);

    for attack_square in attack_squares {
        let to_piece_type = board.get_piece((attack_square.0, attack_square.1));
        let to_square_colour = board.get_piece_colour((attack_square.0, attack_square.1));

        moves.push(Move {
            from: square,
            from_piece: ROOK,
            to: (attack_square.0, attack_square.1),
            to_piece: to_piece_type,
            from_colour: side_to_generate_for,
            to_colour: to_square_colour,
            notation_move: convert_array_location_to_notation(
                square,
                (attack_square.0, attack_square.1),
                None,
            ),
            en_passant: false,

            promotion_to: None,
            castle_from_to_square: None,
            castling_intermediary_square: None,
            sort_score: 0,
        });
    }

    return moves;
}
pub fn get_queen_moves(
    square: (usize, usize),
    side_to_generate_for: i8,
    board: &Board,
) -> Vec<(usize, usize)> {
    let mut attacking_squares: Vec<(usize, usize)> = vec![];
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
            // if out of bounds, stop
            if (row as isize + direction.0 * multiplier) < 0
                || (row as isize + direction.0 * multiplier) > 7
                || (column as isize + direction.1 * multiplier) < 0
                || (column as isize + direction.1 * multiplier) > 7
            {
                continue;
            }

            let to_square_colour = board.get_piece_colour((
                (row as isize + direction.0 * multiplier) as usize,
                (column as isize + direction.1 * multiplier) as usize,
            ));

            if to_square_colour == side_to_generate_for {
                break;
            }

            attacking_squares.push((
                (row as isize + direction.0 * multiplier) as usize,
                (column as isize + direction.1 * multiplier) as usize,
            ));

            // if captured a piece, stop multiplying and look in new direction
            if to_square_colour != side_to_generate_for && to_square_colour != EMPTY {
                break;
            }
        }
    }

    return attacking_squares;
}
pub fn generate_queen_moves(
    square: (usize, usize),
    side_to_generate_for: i8,
    board: &Board,
) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];

    let attack_squares = get_queen_moves(square, side_to_generate_for, board);

    for attack_square in attack_squares {
        let to_piece_type = board.get_piece((attack_square.0, attack_square.1));
        let to_square_colour = board.get_piece_colour((attack_square.0, attack_square.1));

        moves.push(Move {
            from: square,
            from_piece: QUEEN,
            to: (attack_square.0, attack_square.1),
            to_piece: to_piece_type,
            from_colour: side_to_generate_for,
            to_colour: to_square_colour,
            notation_move: convert_array_location_to_notation(
                square,
                (attack_square.0, attack_square.1),
                None,
            ),
            en_passant: false,

            promotion_to: None,
            castle_from_to_square: None,
            castling_intermediary_square: None,
            sort_score: 0,
        });
    }

    return moves;
}
pub fn get_king_attacks(
    square: (usize, usize),
    side_to_generate_for: i8,
    board: &Board,
) -> Vec<(usize, usize)> {
    let mut attacking_squares: Vec<(usize, usize)> = vec![];
    // from a rooks square, look along the 4 directions to see if it can move further
    // let _enemy_color = if side_to_generate_for == 1 { 2 } else { 1 };
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
        if column == 0 && direction.1 == -1 {
            continue;
        }

        if row == 0 && direction.0 == -1 {
            continue;
        }

        if column == 7 && direction.1 == 1 {
            continue;
        }

        if row == 7 && direction.0 == 1 {
            continue;
        }
        let to_square_colour = board.get_piece_colour((
            (row as isize + direction.0) as usize,
            (column as isize + direction.1) as usize,
        ));
        if to_square_colour == side_to_generate_for {
            continue;
        }

        attacking_squares.push((
            (row as isize + direction.0) as usize,
            (column as isize + direction.1) as usize,
        ))
    }

    return attacking_squares;
}
/// generate pseudo legal king moves,
/// this includes castling
/// this will check king is not being moved into check
pub fn generate_king_moves(
    square: (usize, usize),
    side_to_generate_for: i8,
    board: &Board,
) -> Vec<Move> {
    // when castling, take into account that the king is moving through the squares, not teleporting
    // only for those squares castling still possible
    let mut moves: Vec<Move> = vec![];
    let (row, column) = square;
    let attack_squares = get_king_attacks(square, side_to_generate_for, board);

    for attack_square in attack_squares {
        let to_piece_type = board.get_piece((attack_square.0, attack_square.1));
        let to_square_colour = board.get_piece_colour((attack_square.0, attack_square.1));

        moves.push(Move {
            from: square,
            from_piece: KING,
            to: (attack_square.0, attack_square.1),
            to_piece: to_piece_type,
            from_colour: side_to_generate_for,
            to_colour: to_square_colour,
            notation_move: convert_array_location_to_notation(
                square,
                (attack_square.0, attack_square.1),
                None,
            ),
            en_passant: false,

            promotion_to: None,
            castle_from_to_square: None,
            castling_intermediary_square: None,
            sort_score: 0,
        });

        // if captured a piece, stop multiplying and look in new direction
        if to_square_colour != side_to_generate_for && to_square_colour != EMPTY {
            continue;
        }
    }

    // castling

    if !board.has_white_king_moved && side_to_generate_for == WHITE {
        if board.a1_rook_not_moved
            && board.is_square_empty("b1")
            && board.is_square_empty("c1")
            && board.is_square_empty("d1")
        {
            //check if moving into d1 is check.

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
                castle_from_to_square: Some(((7, 0), (7, 3))),
                castling_intermediary_square: Some((7, 3)), //d1
                sort_score: 0,
            });
        }
        if board.h1_rook_not_moved && board.is_square_empty("f1") && board.is_square_empty("g1") {
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
                castle_from_to_square: Some(((7, 7), (7, 5))),
                castling_intermediary_square: Some((7, 5)), //f1
                sort_score: 0,
            });
        }
    }

    if !board.has_black_king_moved && side_to_generate_for == BLACK {
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
                castle_from_to_square: Some(((0, 0), (0, 3))),
                castling_intermediary_square: Some((0, 3)), //d8
                sort_score: 0,
            });
        }
        if board.h8_rook_not_moved && board.is_square_empty("f8") && board.is_square_empty("g8") {
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
                castle_from_to_square: Some(((0, 7), (0, 5))),
                castling_intermediary_square: Some((0, 5)), //f8
                sort_score: 0,
            });
        }
    }

    return moves;
}

pub fn generate_pseudo_legal_moves(board: &Board, side_to_generate_for: i8) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    // println!("square: {}", 1);
    // println!("side_to_generate_for: {}", side_to_generate_for);
    // go through each piece on the board, by colour to only get moves for side to move.
    for (row_index, row) in board.colour_array.iter().enumerate() {
        for (column_index, colour) in row.iter().enumerate() {
            if colour != &side_to_generate_for {
                continue;
            }
            let location = (row_index, column_index);
            let square = board.get_piece((row_index, column_index));

            let mut generated_moves = match square {
                1 => generate_pawn_moves(location, side_to_generate_for, board),
                2 => generate_knight_moves(location, side_to_generate_for, board),
                3 => generate_bishop_moves(location, side_to_generate_for, board),
                4 => generate_rook_moves(location, side_to_generate_for, board),
                5 => generate_queen_moves(location, side_to_generate_for, board),
                6 => generate_king_moves(location, side_to_generate_for, board),
                _ => vec![],
            };

            //check for all the non king pieces if they are attacking the castling squares

            moves.append(&mut generated_moves);
        }
    }

    return moves;
}
