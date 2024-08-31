use crate::board::Board;
use crate::constants::*;
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
    pub wtime: i32,
    pub btime: i32,
    pub winc: i32,
    pub binc: i32,
}

pub fn order_moves(moves: &mut Vec<Move>) {
    for i in 0..moves.len() {
        let move_to_score = moves.get_mut(i).unwrap();
        let value = MVV_LVA[move_to_score.to_piece as usize][move_to_score.from_piece as usize];
        move_to_score.sort_score += value as i32;
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
            depth: 0,
            winc: 0,
            wtime: 0,
            binc: 0,
            btime: 0,
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

    pub fn search(&mut self, board: &mut Board, depth: i8) -> (Move, Vec<BestMoves>) {
        // adding in iterative deepening?

        let mut best_move = Move::default();
        let mut best_score = -1000;
        let mut best_moves = Vec::new();

        self.nodes = 0;
        self.start = Instant::now();
        let current_side = board.side_to_move;

        let currently_in_check = evaluate::is_in_check(board, current_side, None);

        // generate moves for current depth of board
        let mut moves_for_current_depth =
            generate_pseudo_legal_moves(board, board.side_to_move, currently_in_check);
        order_moves(&mut moves_for_current_depth);
        for generated_move in moves_for_current_depth.iter() {
            board.make_move(generated_move);

            // check not moving self into check
            if evaluate::is_in_check(
                board,
                current_side,
                generated_move.castling_intermediary_square,
            ) {
                board.un_make_move(generated_move);
                continue;
            }

            let score = self.minimax(board, depth, true, i32::MIN, i32::MAX);

            // store the score against each move?
            if score > best_score {
                best_score = score;
                best_move = generated_move.clone();
                best_moves.push(BestMoves {
                    best_move: generated_move.clone(),
                    best_score,
                });
            }

            board.un_make_move(generated_move);
        }
        // add secondary sort by 'sortscore'.
        best_moves.sort_by(|a, b| {
            let score_cmp = b.best_score.cmp(&a.best_score);
            if score_cmp == std::cmp::Ordering::Equal {
                b.best_move.sort_score.cmp(&a.best_move.sort_score)
            } else {
                score_cmp
            }
        });

        return (best_move, best_moves);
    }

    pub fn perft(&mut self, board: &mut Board, depth: i8, first_call: bool) -> i128 {
        let mut nodes_per_root_move: i128 = 0;
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
                    move_notation: generated_move.notation_move.clone(),
                    nodes: nodes_per_root_move,
                });
                self.nodes += nodes;
            }
        }

        return nodes;
    }

    pub fn bench() {
        // run a series of fens,
        // output total nodes searched
        // are these legal moves?
    }
}
