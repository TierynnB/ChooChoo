use crate::board::Board;
use crate::constants::*;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

pub struct ZobristKeys {
    pub pieces: [[[u64; 64]; 7]; 2], // [colour][piece_type][square]
    pub side: u64,
    pub castling: [u64; 16],
    pub en_passant: [u64; 64], // index by square to be safe
}

impl ZobristKeys {
    pub fn new() -> Self {
        // Use a fixed seed for reproducible hashes
        let mut rng = StdRng::seed_from_u64(42);
        
        let mut pieces = [[[0u64; 64]; 7]; 2];
        for c in 0..2 {
            for p in 1..7 {
                for s in 0..64 {
                    pieces[c][p][s] = rng.gen();
                }
            }
        }
        
        let side = rng.gen();
        
        let mut castling = [0u64; 16];
        for i in 0..16 {
            castling[i] = rng.gen();
        }
        
        let mut en_passant = [0u64; 64];
        for s in 0..64 {
            en_passant[s] = rng.gen();
        }
        
        ZobristKeys {
            pieces,
            side,
            castling,
            en_passant,
        }
    }

    pub fn get_hash(&self, board: &Board) -> u64 {
        let mut hash = 0u64;
        
        // Pieces
        for row in 0..8 {
            for col in 0..8 {
                let piece = board.board_array[row][col];
                if piece != EMPTY {
                    let colour = board.colour_array[row][col];
                    let c_idx = if colour == WHITE { 0 } else { 1 };
                    hash ^= self.pieces[c_idx][piece as usize][row * 8 + col];
                }
            }
        }
        
        // Side to move
        if board.side_to_move == BLACK {
            hash ^= self.side;
        }
        
        // Castling rights
        let mut castling_idx = 0;
        if board.can_castle_h1 { castling_idx |= 1; } // White King side
        if board.can_castle_a1 { castling_idx |= 2; } // White Queen side
        if board.can_castle_h8 { castling_idx |= 4; } // Black King side
        if board.can_castle_a8 { castling_idx |= 8; } // Black Queen side
        hash ^= self.castling[castling_idx];
        
        // En Passant
        if let Some((row, col)) = board.en_passant_location {
            hash ^= self.en_passant[row * 8 + col];
        }
        
        hash
    }
}

// Global lazy initialization if needed, but better to pass it around in SearchEngine
// or store it in Board if we want it to be truly incremental.
