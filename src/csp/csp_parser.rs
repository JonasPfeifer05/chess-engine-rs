use crate::csp::command::ClientCommand;
use crate::csp::command::ClientCommand::{Fen, Join, Killed, Leave, Move, New};
use crate::game_board::piece::Color;
use crate::game_board::position::{HorizontalPosition, Position, VerticalPosition};

pub struct CSPParser;

impl CSPParser {
    pub fn parse_client_command(command: &str) -> Option<ClientCommand> {
        let parts: Vec<_> = command.trim().split(" ").map(|x| x.to_lowercase()).collect();
        if parts.len() == 0 { return None; }

        match parts.first().unwrap().as_str() {
            "join" => {
                if parts.len() < 2 { return None; }
                Some(Join { code: parts.get(1).unwrap().clone(), peer: None })
            }
            "leave" => { Some(Leave { peer: None }) }
            "new" => {
                if parts.len() < 2 { return None; }
                Some(New { fen: parts.get(1).unwrap().clone() })
            }
            "killed" => {
                if parts.len() < 2 { return None; }
                Some(Killed {
                    color: match parts.get(1).unwrap().to_lowercase().as_str() {
                        "white" => { Color::White }
                        "black" => { Color::Black }
                        &_ => { return None; }
                    }
                })
            }
            "fen" => { Some(Fen { peer: None }) }
            "move" => {
                if parts.len() < 3 { return None; }
                let from: Vec<_> = parts.get(1).unwrap().chars().collect();
                if from.len() != 2 { return None; }
                let to: Vec<_> = parts.get(1).unwrap().chars().collect();
                if to.len() != 2 { return None; }

                let from = {
                    if !from.get(0).unwrap().is_digit(10) { return None; }
                    if !from.get(1).unwrap().is_digit(10) { return None; }
                    Position::new(
                        HorizontalPosition::try_from(
                            from.get(0).unwrap().to_digit(10).unwrap() as u8
                        ).unwrap(),
                        VerticalPosition::try_from(
                            from.get(1).unwrap().to_digit(10).unwrap() as u8
                        ).unwrap()
                    )
                };

                let to = {
                    if !to.get(0).unwrap().is_digit(10) { return None; }
                    if !to.get(1).unwrap().is_digit(10) { return None; }
                    Position::new(
                        HorizontalPosition::try_from(
                            to.get(0).unwrap().to_digit(10).unwrap() as u8
                        ).unwrap(),
                        VerticalPosition::try_from(
                            to.get(1).unwrap().to_digit(10).unwrap() as u8
                        ).unwrap()
                    )
                };

                Some(Move {from, to, peer: None })
            }
            &_ => None
        }
    }
}

