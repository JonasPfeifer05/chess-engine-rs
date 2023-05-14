use crate::game_board::board::Board;
use crate::game_board::position::{Position, PositionBuilder};
use crate::movement::movement::{Move};

pub struct MoveEvaluator;

impl MoveEvaluator {
    /// Filters out invalid moves and only returns valid ones
    pub fn validate_moves(from: &Position, board: &Board) -> Vec<Position> {
        let figure = board.get_piece(from).unwrap();

        let mut moves: Vec<Position> = Vec::new();

        for moveset in figure.movable().get_move_sets() {
            for movee in moveset.get_relative_moves(figure.color()) {
                match movee {
                    Move::SingleMove { relative_move, can_kill, only_with_kill } => {
                        let target = if let Ok(pos) = PositionBuilder::add(from, relative_move) { pos } else { continue };

                        if let Some(piece) = board.get_piece(&target) {
                            if piece.color() == figure.color() { continue; }
                            else if !can_kill { continue; }
                        } else if only_with_kill { continue; }

                        moves.push(PositionBuilder::add(from, relative_move).unwrap());
                    }
                    Move::ChainedMove { relative_moves } => {
                        for relative_move in relative_moves {
                            let target = if let Ok(pos) = PositionBuilder::add(from, relative_move) { pos } else { break; };

                            if let Some(piece) = board.get_piece(&target) {
                                if piece.color() == figure.color() { break; }
                                moves.push(PositionBuilder::add(from, relative_move).unwrap());
                                break;
                            }
                            moves.push(PositionBuilder::add(from, relative_move).unwrap());
                        }
                    }
                }
            }
        }

        moves
    }
}