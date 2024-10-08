use crate::board::Board;
use crate::constants::*;
use crate::conversion;
use crate::evaluate;
use crate::evaluate::evaluate;
use crate::movegen;
use crate::moves::*;
use std::time::Instant;

pub struct TranspositionTableEntry {
    pub position_hash: u64,
    pub depth_distance: i8,
    pub position_terminal_score: i32,
}
pub struct MoveNode {
    pub move_notation: String,
    pub nodes: i128,
}
pub struct BestMoves {
    pub best_move: Move,
    pub best_score: i32,
}
pub struct SearchEngine {
    pub nodes: i128,
    pub start: Instant,
    pub move_nodes: Vec<MoveNode>,
    pub depth: i8,
    pub current_depth: i8,
    pub wtime: u128,
    pub movetime: u128,
    pub btime: u128,
    pub winc: u128,
    pub binc: u128,
    pub use_time_management: bool,
    pub searching_side: i8,
    pub move_overhead: u128,
    pub transposition_table: Vec<TranspositionTableEntry>,
}

pub fn order_moves(moves: &mut Vec<Move>) {
    for i in 0..moves.len() {
        let move_to_score = moves.get_mut(i).unwrap();
        let value = MVV_LVA[move_to_score.to_piece as usize][move_to_score.from_piece as usize];

        move_to_score.sort_score += value;
    }

    moves.sort_by(|a, b| b.sort_score.cmp(&a.sort_score));
}
impl SearchEngine {
    pub fn new() -> Self {
        SearchEngine {
            nodes: 0,
            start: Instant::now(),
            move_nodes: Vec::new(),
            depth: 3,
            current_depth: 1,
            winc: 0,
            movetime: 0,
            move_overhead: 10,
            wtime: 0,
            binc: 0,
            btime: 0,
            use_time_management: false,
            searching_side: WHITE,
            transposition_table: Vec::new(),
        }
    }
    fn clear_tt(&mut self) {
        self.transposition_table = Vec::new();
    }
    fn delete_tt_pos_for_hash(&mut self, position_hash: u64) {
        self.transposition_table
            .retain(|entry| entry.position_hash != position_hash);
    }
    fn add_position_to_tt(
        &mut self,
        position_hash: u64,
        position_terminal_score: i32,
        depth_distance: i8,
    ) {
        self.transposition_table.push(TranspositionTableEntry {
            position_hash,
            position_terminal_score,
            depth_distance: depth_distance,
        });
    }
    fn get_position_from_tt(&self, position_hash: u64) -> Option<&TranspositionTableEntry> {
        for entry in self.transposition_table.iter() {
            if entry.position_hash == position_hash {
                return Some(entry);
            }
        }
        return None;
    }
    pub fn get_allowed_time(&self, side: i8) -> u128 {
        if self.use_time_management {
            if self.movetime > 0 {
                return self.movetime - 2 * self.move_overhead;
            }

            let time_left = if side == WHITE {
                self.wtime
            } else {
                self.btime
            };

            let increment = if side == WHITE { self.winc } else { self.binc };
            // println!("{} {}", self.move_overhead, increment);
            return (time_left / 30 + increment - 2 * self.move_overhead) as u128;
        } else {
            return 10000;
        }
    }
    pub fn minimax(
        &mut self,
        board: &mut Board,
        depth: i8,
        maximizing_player: bool,
        mut alpha: i32,
        mut beta: i32,
    ) -> i32 {
        // the move needs to record its own evaluation
        if depth == 0 {
            self.nodes += 1;

            return evaluate::evaluate(&board);
        };

        // generate moves for current depth of board
        let mut moves_for_current_depth =
            movegen::generate_pseudo_legal_moves(board, board.side_to_move, false);

        order_moves(&mut moves_for_current_depth);

        if maximizing_player {
            let mut max_eval = i32::MIN;
            for generated_move in moves_for_current_depth.iter() {
                board.make_move(generated_move);

                let eval = self.minimax(board, depth - 1, false, alpha, beta);

                board.un_make_move(generated_move);
                max_eval = std::cmp::max(max_eval, eval);
                alpha = std::cmp::max(alpha, eval);
                if beta <= alpha {
                    break;
                }
            }
            return max_eval;
        // and best outcome for minimising player (enemy)
        } else {
            let mut min_eval = i32::MAX;
            for generated_move in moves_for_current_depth.iter() {
                board.make_move(generated_move);

                let eval = self.minimax(board, depth - 1, true, alpha, beta);

                board.un_make_move(generated_move);
                min_eval = std::cmp::min(min_eval, eval);
                beta = std::cmp::min(beta, eval);
                if beta <= alpha {
                    break;
                }
            }
            return min_eval;
        }
    }
    pub fn quiescence_search(&mut self, board: &mut Board, mut alpha: i32, beta: i32) -> i32 {
        // if in check, return.
        // if evaluate::is_in_check(board, board.side_to_move, None) {
        //     return evaluate(board);
        // }

        // if evaluate::is_in_check(board, board.side_to_move * -1, None) {
        //     return evaluate(board);
        // }
        // searches the captures available
        let stand_pat = evaluate(board);
        if stand_pat >= beta {
            return beta;
        }
        if alpha < stand_pat {
            alpha = stand_pat;
        }

        // generate moves for current depth of board
        let mut moves_for_current_depth =
            movegen::generate_pseudo_legal_moves(board, board.side_to_move, false);

        order_moves(&mut moves_for_current_depth);

        for generated_move in moves_for_current_depth.iter() {
            if generated_move.to_piece == EMPTY {
                continue;
            }
            if generated_move.to_piece == KING {
                return alpha;
            }

            board.make_move(generated_move);
            let score = -self.quiescence_search(board, -beta, -alpha);
            board.un_make_move(generated_move);

            if score >= beta {
                return beta;
            }

            if score > alpha {
                alpha = score;
            }
        }

        return alpha;
    }
    pub fn alpha_beta(&mut self, board: &mut Board, depth: i8, mut alpha: i32, beta: i32) -> i32 {
        let mut best_value = i32::MIN;
        if depth == 0 {
            self.nodes += 1;
            return self.quiescence_search(board, alpha, beta); //
                                                               // return evaluate::evaluate(&board);
        };

        if let Some(entry) = self.get_position_from_tt(conversion::hash_board_state(board)) {
            return entry.position_terminal_score;
        }

        let mut moves_for_current_depth =
            movegen::generate_pseudo_legal_moves(board, board.side_to_move, false);

        order_moves(&mut moves_for_current_depth);

        for generated_move in moves_for_current_depth.iter() {
            board.make_move(generated_move);

            let eval = -self.alpha_beta(board, depth - 1, -beta, -alpha);

            // let position_hash = conversion::hash_board_state(board);
            // if self.get_position_from_tt(position_hash, depth).is_none() {
            //     self.delete_tt_pos_for_hash(position_hash);
            //     self.add_position_to_tt(conversion::hash_board_state(board), eval, depth);
            // }

            board.un_make_move(generated_move);

            best_value = std::cmp::max(best_value, eval);
            alpha = std::cmp::max(alpha, eval);

            if eval >= beta {
                return best_value;
            }
        }
        return best_value;
    }

    pub fn search(&mut self, board: &mut Board) -> (Move, Vec<BestMoves>) {
        // adding in iterative deepening?
        let mut searching = true;
        let mut best_move = Move::default();
        let mut best_score = i32::MIN;
        let mut best_moves = Vec::new();

        self.clear_tt();
        self.searching_side = board.side_to_move;
        self.nodes = 0;
        self.start = Instant::now();
        let current_side = board.side_to_move;

        let currently_in_check = evaluate::is_in_check(board, current_side, None);

        // generate moves for current depth of board
        let mut moves_for_current_depth =
            movegen::generate_pseudo_legal_moves(board, board.side_to_move, currently_in_check);
        order_moves(&mut moves_for_current_depth);

        while searching {
            for generated_move in moves_for_current_depth.iter_mut() {
                if generated_move.illegal_move {
                    continue;
                }
                board.make_move(generated_move);

                // check not moving self into check
                if evaluate::is_in_check(
                    board,
                    current_side,
                    generated_move.castling_intermediary_square,
                ) {
                    generated_move.illegal_move = true;
                    board.un_make_move(generated_move);
                    continue;
                }

                if board.has_positions_repeated() {
                    generated_move.illegal_move = true;
                    board.un_make_move(generated_move);
                    continue;
                }
                // let position_hash = conversion::hash_board_state_for_tt(board);
                // match self.get_position_from_tt(position_hash) {
                //     Some(entry) => {
                //         generated_move.search_score = entry.position_terminal_score;
                //     }
                //     None => {

                //         self.add_position_to_tt(position_hash, generated_move.search_score);
                //     }
                // }
                generated_move.search_score =
                    -self.alpha_beta(board, self.current_depth, i32::MIN + 1, i32::MAX);
                board.un_make_move(generated_move);

                if self.use_time_management {
                    if self.start.elapsed().as_millis() > self.get_allowed_time(self.searching_side)
                    {
                        searching = false;
                        break;
                    }
                }
            }

            if searching && self.use_time_management {
                if self.start.elapsed().as_millis() > self.get_allowed_time(self.searching_side) {
                    println!("time limit reached");
                    searching = false;
                }
            }

            if searching && (self.current_depth < self.depth || self.use_time_management) {
                self.current_depth += 1;
            } else {
                searching = false;
            }

            moves_for_current_depth.sort_by(|a, b| {
                let score_cmp = b.search_score.cmp(&a.search_score);
                if score_cmp == std::cmp::Ordering::Equal {
                    b.sort_score.cmp(&a.sort_score)
                } else {
                    score_cmp
                }
            });
        }

        for generated_move in moves_for_current_depth.iter() {
            // println!(
            //     "{} {} {} {}",
            //     conversion::convert_move_to_notation(generated_move),
            //     generated_move.search_score,
            //     generated_move.sort_score,
            //     generated_move.illegal_move
            // );
            if generated_move.illegal_move {
                continue;
            }

            if generated_move.search_score > best_score {
                best_score = generated_move.search_score;
                best_move = generated_move.clone();
            }
            best_moves.push(BestMoves {
                best_move: generated_move.clone(),
                best_score: generated_move.search_score,
            });
        }

        return (best_move, best_moves);
    }

    pub fn perft(&mut self, board: &mut Board, depth: i8, first_call: bool) -> i128 {
        let mut nodes_per_root_move: i128;
        let mut nodes: i128 = 0;
        if depth == 0 {
            return 1;
        }

        let current_side = board.side_to_move;

        let currently_in_check = evaluate::is_in_check(board, current_side, None);

        let mut moves_for_current_depth =
            movegen::generate_pseudo_legal_moves(board, board.side_to_move, currently_in_check);

        for generated_move in moves_for_current_depth.iter_mut() {
            board.make_move(generated_move);

            if board.has_positions_repeated() {
                generated_move.illegal_move = true;
                board.un_make_move(generated_move);
                continue;
            }

            if evaluate::is_in_check(
                board,
                current_side,
                generated_move.castling_intermediary_square,
            ) {
                board.un_make_move(generated_move);
                continue;
            }

            nodes_per_root_move = self.perft(board, depth - 1, false);
            nodes += nodes_per_root_move;
            board.un_make_move(generated_move);

            if first_call {
                // update root node here with number
                self.move_nodes.push(MoveNode {
                    move_notation: conversion::convert_move_to_notation(generated_move),
                    nodes: nodes_per_root_move,
                });
                self.nodes += nodes;
            }
        }

        return nodes;
    }
}

#[cfg(test)]
mod tests {
    use crate::conversion;
    use crate::search::Board;
    use crate::search::SearchEngine;
    #[test]
    fn perft_1_startpos() {
        let mut engine = SearchEngine::new();
        let mut board = Board::init();

        let nodes = engine.perft(&mut board, 1, true);
        assert_eq!(nodes, 20);
    }
    #[test]
    fn perft_2_startpos() {
        let mut engine = SearchEngine::new();
        let mut board = Board::init();

        let nodes = engine.perft(&mut board, 2, true);
        assert_eq!(nodes, 400);
    }

    #[test]
    fn perft_3_startpos() {
        let mut engine = SearchEngine::new();
        let mut board = Board::init();

        let nodes = engine.perft(&mut board, 3, true);
        assert_eq!(nodes, 8902);
    }
    #[test]
    fn perft_4_startpos() {
        let mut engine = SearchEngine::new();
        let mut board = Board::init();

        let nodes = engine.perft(&mut board, 4, true);
        assert_eq!(nodes, 197281);
    }

    #[test]
    fn perft_1_kiwipete() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ",
        );

        let nodes = engine.perft(&mut board, 1, true);
        assert_eq!(nodes, 48);
    }
    #[test]
    fn perft_2_kiwipete() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ",
        );

        let nodes = engine.perft(&mut board, 2, true);
        assert_eq!(nodes, 2039);
    }
    #[test]
    fn perft_3_kiwipete() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ",
        );

        let nodes = engine.perft(&mut board, 3, true);
        assert_eq!(nodes, 97862);
    }

    #[test]
    fn perft_1_position_3() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ");

        let nodes = engine.perft(&mut board, 1, true);
        assert_eq!(nodes, 14);
    }
    #[test]
    fn perft_2_position_3() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ");

        let nodes = engine.perft(&mut board, 2, true);
        assert_eq!(nodes, 191);
    }
    #[test]
    fn perft_3_position_3() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ");

        let nodes = engine.perft(&mut board, 3, true);
        assert_eq!(nodes, 2812);
    }
    #[test]
    fn perft_4_position_3() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ");

        let nodes = engine.perft(&mut board, 4, true);
        assert_eq!(nodes, 43238);
    }
    #[test]
    fn perft_5_position_3() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ");

        let nodes = engine.perft(&mut board, 5, true);
        assert_eq!(nodes, 674624);
    }

    #[test]
    fn perft_1_position_4() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        );

        let nodes = engine.perft(&mut board, 1, true);
        assert_eq!(nodes, 6);
    }
    #[test]
    fn perft_2_position_4() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        );

        let nodes = engine.perft(&mut board, 2, true);
        assert_eq!(nodes, 264);
    }
    #[test]
    fn perft_3_position_4() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        );

        let nodes = engine.perft(&mut board, 3, true);
        assert_eq!(nodes, 9467);
    }
    #[test]
    fn perft_4_position_4() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board(
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        );

        let nodes = engine.perft(&mut board, 4, true);
        assert_eq!(nodes, 422333);
    }

    #[test]
    fn perft_1_position_5() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board(
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        );

        let nodes = engine.perft(&mut board, 1, true);
        assert_eq!(nodes, 44);
    }
    #[test]
    fn perft_2_position_5() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board(
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        );

        let nodes = engine.perft(&mut board, 2, true);
        assert_eq!(nodes, 1486);
    }
    #[test]
    fn perft_3_position_5() {
        let mut engine = SearchEngine::new();
        let mut board = conversion::convert_fen_to_board(
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        );

        let nodes = engine.perft(&mut board, 3, true);
        assert_eq!(nodes, 62379);
    }
}
