use crate::csp::command::ClientCommand;
use crate::csp::command::ClientCommand::{Fen, Join, Killed, Leave, Move, New};
use crate::game_board::piece::Color;
use crate::game_board::position::Position;

pub struct CSPParser;

impl CSPParser {
    pub fn parse_client_command(command: &str) -> Result<ClientCommand, String> {
        let parts: Vec<_> = command.trim().split(" ").map(|x| x.to_lowercase()).collect();
        if parts.len() == 0 { return Err("No command passed".to_string()); }

        match parts.first().unwrap().as_str() {
            "join" => {
                if parts.len() < 2 { return Err("Not enough arguments passed for join".to_string()); }
                Ok(Join { code: parts.get(1).unwrap().clone() })
            }
            "leave" => { Ok(Leave) }
            "new" => {
                if parts.len() < 2 { return Err("Not enough arguments passed for new".to_string()); }
                Ok(New { fen: parts.get(1).unwrap().clone() })
            }
            "killed" => {
                if parts.len() < 2 { return Err("Not enough parts passed for killed".to_string()); }
                Ok(Killed {
                    color: match parts.get(1).unwrap().to_lowercase().as_str() {
                        "white" => { Color::White }
                        "black" => { Color::Black }
                        &_ => { return Err("Invalid color passed".to_string()); }
                    }
                })
            }
            "fen" => { Ok(Fen) }
            "move" => {
                if parts.len() < 3 { return Err("Not enough parts passed for move".to_string()); }
                let from: Vec<_> = parts.get(1).unwrap().chars().collect();
                if from.len() != 2 { return Err("Invalid 'from' position passed".to_string()); }
                let to: Vec<_> = parts.get(1).unwrap().chars().collect();
                if to.len() != 2 { return Err("Invalid 'to' position passed".to_string()); }

                let from_x = if let Some(x) = from.get(0).unwrap().to_digit(10) { x }
                else { return Err("Invalid 'from' position passed".to_string()); };
                let from_y = if let Some(y) = from.get(1).unwrap().to_digit(10) { y }
                else { return Err("Invalid 'from' position passed".to_string()); };

                let from = Position::new(from_x as u8, from_y as u8)?;

                let to_x = if let Some(x) = to.get(0).unwrap().to_digit(10) { x }
                else { return Err("Invalid 'to' position passed".to_string()); };
                let to_y = if let Some(y) = to.get(1).unwrap().to_digit(10) { y }
                else { return Err("Invalid 'to' position passed".to_string()); };

                let to = Position::new(to_x as u8, to_y as u8)?;

                Ok(Move {from, to})
            }
            &_ => Err("Unknown command passed".to_string())
        }
    }
}

