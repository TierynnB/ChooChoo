use crate::constants::EMPTY;

#[derive(Clone, Debug)]
pub struct Move {
    pub from: (usize, usize),
    pub from_piece: i8,
    pub from_colour: i8,
    pub to: (usize, usize),
    pub to_piece: i8,
    pub to_colour: i8,
    pub promotion_to: Option<i8>,
    pub en_passant: bool,
    pub castle_from_to_square: Option<((usize, usize), (usize, usize))>,
    pub castling_intermediary_square: Option<(usize, usize)>,
    pub sort_score: u8,
    pub search_score: i32,
    pub illegal_move: bool,
}
impl Default for Move {
    fn default() -> Self {
        // return a default instance of Move
        return Move {
            from: (0, 0),
            from_piece: EMPTY,
            from_colour: EMPTY,
            to: (0, 0),
            to_piece: EMPTY,
            to_colour: EMPTY,
            promotion_to: None,
            en_passant: false,
            castle_from_to_square: None,
            castling_intermediary_square: None,
            sort_score: 0,
            search_score: 0,
            illegal_move: false,
        };
    }
}
