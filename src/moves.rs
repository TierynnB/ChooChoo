#[derive(Clone, Debug)]
pub struct Move {
    pub from: (usize, usize),
    pub from_piece: i8,
    pub from_colour: i8,
    pub to: (usize, usize),
    pub to_piece: i8,
    pub to_colour: i8,
    pub notation_move: String,
    pub promotion_to: Option<i8>,
    pub en_passant: bool,
    pub castle_from_to_square: Option<((usize, usize), (usize, usize))>,
    pub castling_intermediary_square: Option<(usize, usize)>,
    pub sort_score: i32,
}
impl Default for Move {
    fn default() -> Self {
        // return a default instance of Move
        return Move { 
            from: (0, 0),
            from_piece: 0,
            from_colour: 0,
            to: (0, 0),
            to_piece: 0,
            to_colour: 0,

            notation_move: String::default(),
            promotion_to: None,
            en_passant: false,
            castle_from_to_square: None,
            castling_intermediary_square: None,
            sort_score: 0,
        };
    }
}
