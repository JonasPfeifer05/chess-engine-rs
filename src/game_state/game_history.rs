use crate::game_board::position::Position;

#[derive(Debug)]
pub struct GameHistory {
    played_moves: Vec<(Position, Position)>,
}

impl Default for GameHistory {
    fn default() -> Self {
        Self {
            played_moves: vec![],
        }
    }
}

impl GameHistory {
    pub fn push(&mut self, from: Position, to: Position) {
        self.played_moves.push((from, to));
    }

    pub fn played_moves(&self) -> &[(Position, Position)] {
        &self.played_moves
    }

    pub fn last_n_moves(&self, mut n: usize) -> &[(Position, Position)]  {
        if self.played_moves.len() < n { n = self.played_moves.len() }

        let index = self.played_moves.len() - n;

        &self.played_moves[index..]
    }
}