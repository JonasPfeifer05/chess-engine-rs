use crate::game_board::board::Board;
use crate::game_board::position::{HorizontalPosition, Position, VerticalPosition};
use crate::movement::movement::{HorizontalOneRight, Move};

pub struct MoveEvaluator;

impl MoveEvaluator {
    /// Filters out invalid moves and only returns valid ones
    pub fn validate_moves<'a>(moves: &'a Vec<Move>, from: &Position, board: &Board) -> Vec<&'a Move> {
        let figure = board.get_piece(from).unwrap();

        let mut in_the_way = false;
        let moves: Vec<_> = moves.iter()
            .filter(|movee| {
                let x = from.horizontal as u8 as i8 + movee.relative_move().0;
                !(x < 0 || x > 8)
            })
            .filter(|movee| {
                let y = from.vertical as u8 as i8 + movee.relative_move().1;
                !(y < 0 || y > 8)
            })
            .filter(|movee| {
                let x =  HorizontalPosition::try_from((from.horizontal as u8 as i8 + movee.relative_move().0) as u8).unwrap();
                let y =  VerticalPosition::try_from((from.vertical as u8 as i8 + movee.relative_move().1) as u8).unwrap();

                if let Some(piece) =  board.get_piece(&Position::new(x,y)) {
                    in_the_way = true;
                }

                if movee.check_in_way() {
                    !in_the_way
                } else {
                    true
                }
            })
            .filter(|movee| {
                let x =  HorizontalPosition::try_from((from.horizontal as u8 as i8 + movee.relative_move().0) as u8).unwrap();
                let y =  VerticalPosition::try_from((from.vertical as u8 as i8 + movee.relative_move().1) as u8).unwrap();

                if let Some(piece) =  board.get_piece(&Position::new(x,y)) {
                    piece.color() != figure.color() && movee.can_kill()
                } else {
                    true && !movee.only_with_kill()
                }
            })
            .collect();

        moves
    }
}