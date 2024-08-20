use std::vec;

use rand::seq::index;

use crate::{board::*, constants::*, conversion::*, evaluate::*, moves::*};

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
        // if in the promotion row, you must also promote
        if row == promotion_row {
            for piece in [KNIGHT, BISHOP, ROOK, QUEEN] {
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
        } else {
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
    }
    // attack other diagonal
    square_attack_colour = board.colour_array[index_of_square_in_front][column - 1];
    square_attack_piece = board.board_array[index_of_square_in_front][column - 1];
    if square_attack_colour != side_to_generate_for
        && square_attack_colour != -1
        && square_attack_colour != 0
    {
        // if in the promotion row, you must also promote
        if row == promotion_row {
            for piece in [KNIGHT, BISHOP, ROOK, QUEEN] {
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
        } else {
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
    king_check_depth: i8,
) -> Vec<Move> {
    // get sides king location
    // let king_location = board.get_king_location(side_to_generate_for);
    let mut opponent_moves: Vec<Move> = vec![];
    // generate opponent moves
    if king_check_depth > 0 {
        opponent_moves = generate_pseudo_legal_moves(
            board,
            if side_to_generate_for == WHITE {
                BLACK
            } else {
                WHITE
            },
            king_check_depth - 1,
        );
    }

    // when castling, take into account that the king is moving through the squares, not teleporting
    // only for those squares castling still possible
    let mut castling_squares_being_attacked = CastlingSquaresAttacked::default();

    if !board.has_king_moved
        && (board.a1_rook_not_moved
            || board.h1_rook_not_moved
            || board.a8_rook_not_moved
            || board.h8_rook_not_moved)
    {
        for enemy_move in &opponent_moves {
            match enemy_move.to {
                (9, 5) => castling_squares_being_attacked.d1_attacked = true, //d1
                (9, 4) => castling_squares_being_attacked.c1_attacked = true, //c1
                (9, 6) => castling_squares_being_attacked.e1_attacked = true, //e1
                (9, 7) => castling_squares_being_attacked.f1_attacked = true, //f1
                (9, 8) => castling_squares_being_attacked.g1_attacked = true, //g1
                (2, 5) => castling_squares_being_attacked.d8_attacked = true, //d8
                (2, 4) => castling_squares_being_attacked.c8_attacked = true, //c8
                (2, 6) => castling_squares_being_attacked.e8_attacked = true, //e8
                (2, 7) => castling_squares_being_attacked.f8_attacked = true, //f8
                (2, 8) => castling_squares_being_attacked.g8_attacked = true, //g8
                _ => {}
            }
        }
    }

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

        let mut move_is_legal = true;
        if square_move == -1 || square_move == side_to_generate_for {
            continue;
        }
        // check not in attacked squares
        for enemy_move in &opponent_moves {
            if enemy_move.to.0 == ((row as isize + direction.0) as usize)
                && enemy_move.to.1 == ((column as isize + direction.1) as usize)
            {
                move_is_legal = false;
                break;
            }
        }
        if !move_is_legal {
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
                && !castling_squares_being_attacked.c1_attacked
                && !castling_squares_being_attacked.d1_attacked
                && !castling_squares_being_attacked.e1_attacked
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
                    castle_from_to_square: None,
                    sort_score: 0,
                });
            }
            if board.h1_rook_not_moved
                && board.is_square_empty("f1")
                && board.is_square_empty("g1")
                && !castling_squares_being_attacked.f1_attacked
                && !castling_squares_being_attacked.g1_attacked
                && !castling_squares_being_attacked.e1_attacked
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
                && !castling_squares_being_attacked.c8_attacked
                && !castling_squares_being_attacked.d8_attacked
                && !castling_squares_being_attacked.e8_attacked
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
            if board.h8_rook_not_moved
                && board.is_square_empty("f8")
                && board.is_square_empty("g8")
                && !castling_squares_being_attacked.f8_attacked
                && !castling_squares_being_attacked.g8_attacked
                && !castling_squares_being_attacked.e8_attacked
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

pub fn generate_pseudo_legal_moves(
    board: &Board,
    side_to_generate_for: i8,
    king_check_depth: i8,
) -> Vec<Move> {
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
                6 => generate_king_moves(location, side_to_generate_for, board, king_check_depth),
                _ => vec![],
            };

            //check for all the non king pieces if they are attacking the castling squares

            moves.append(&mut generated_moves);
        }
    }

    return moves;
}
