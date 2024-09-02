use crate::board::Board;
use crate::constants::*;
use crate::conversion;
use crate::evaluate;
use crate::movegen::*;
use crate::moves::*;
use std::time::Instant;
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
    pub wtime: u128,
    pub btime: u128,
    pub winc: u128,
    pub binc: u128,
    pub use_time_management: bool,
    pub searching_side: i8,
}

pub fn order_moves(moves: &mut Vec<Move>) {
    for i in 0..moves.len() {
        let move_to_score = moves.get_mut(i).unwrap();
        let value = MVV_LVA[move_to_score.to_piece as usize][move_to_score.from_piece as usize];
        // println!("{} {}", move_to_score.notation_move, value);
        move_to_score.sort_score += value;
    }

    moves.sort_by(|a, b| a.sort_score.cmp(&b.sort_score));
    // sort moves
    // captures first
}
impl SearchEngine {
    pub fn new() -> Self {
        SearchEngine {
            nodes: 0,
            start: Instant::now(),
            move_nodes: Vec::new(),
            depth: 3,
            winc: 0,
            wtime: 0,
            binc: 0,
            btime: 0,
            use_time_management: false,
            searching_side: WHITE,
        }
    }
    pub fn get_allowed_time(&self, side: i8) -> u128 {
        if self.use_time_management {
            let time_left = if side == WHITE {
                self.wtime
            } else {
                self.btime
            };

            let increment = if side == WHITE { self.winc } else { self.binc };

            return time_left / 30 + increment;
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
            return evaluate::evaluate(&board, self.searching_side);
        };

        if self.use_time_management {
            if self.start.elapsed().as_millis() > self.get_allowed_time(self.searching_side) {
                return evaluate::evaluate(&board, self.searching_side);
            }
        }
        // generate moves for current depth of board
        let mut moves_for_current_depth =
            generate_pseudo_legal_moves(board, board.side_to_move, false);
        order_moves(&mut moves_for_current_depth);
        if maximizing_player {
            let mut max_eval = -1000;
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
            let mut min_eval = 1000;
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

    pub fn search(&mut self, board: &mut Board) -> (Move, Vec<BestMoves>) {
        // adding in iterative deepening?
        let mut searching = true;
        let mut best_move = Move::default();
        let mut best_score = -1000;
        let mut best_moves = Vec::new();
        let mut local_depth = 1;

        self.searching_side = board.side_to_move;
        self.nodes = 0;
        self.start = Instant::now();

        // itereative deepening
        // starts at depth 1, then after each depth search,
        // check elapsed time, sort moves, and search again at depth += 1

        let current_side = board.side_to_move;

        let currently_in_check = evaluate::is_in_check(board, current_side, None);

        // generate moves for current depth of board
        let mut moves_for_current_depth =
            generate_pseudo_legal_moves(board, board.side_to_move, currently_in_check);
        order_moves(&mut moves_for_current_depth);

        while searching {
            for generated_move in moves_for_current_depth.iter_mut() {
                board.make_move(generated_move);

                if board.has_positions_repeated() {
                    generated_move.illegal_move = true;
                    board.un_make_move(generated_move);
                    continue;
                }

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

                generated_move.search_score =
                    self.minimax(board, local_depth, true, i32::MIN, i32::MAX);

                board.un_make_move(generated_move);
            }

            moves_for_current_depth.retain(|m| !m.illegal_move);

            if self.use_time_management {
                if self.start.elapsed().as_millis() > self.get_allowed_time(self.searching_side) {
                    println!("time limit reached");
                    searching = false;
                }
            }

            if local_depth < self.depth
                || (!(self.start.elapsed().as_millis()
                    > self.get_allowed_time(self.searching_side))
                    && self.use_time_management)
            {
                local_depth += 1;
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
        // println!("ply {} side {}", board.ply, board.side_to_move);
        let current_side = board.side_to_move;

        let currently_in_check = evaluate::is_in_check(board, current_side, None);

        let moves_for_current_depth =
            generate_pseudo_legal_moves(board, board.side_to_move, currently_in_check);

        for generated_move in moves_for_current_depth.iter() {
            board.make_move(generated_move);

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
