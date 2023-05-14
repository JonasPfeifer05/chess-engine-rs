use std::fmt::{Debug, Display, format, Formatter, Write};
use std::net::SocketAddr;
use crate::game_board::piece::Color;
use crate::game_board::position::Position;

#[derive(Debug)]
pub enum ClientCommand {
    Join { code: String, peer: Option<SocketAddr> },
    Leave { peer: Option<SocketAddr> },
    New { fen: String },
    Killed { color: Color },
    Fen { peer: Option<SocketAddr> },
    Move { from: Position, to: Position, peer: Option<SocketAddr> }
}

pub enum ServerCommand {
    OkColor { color: Color },
    OkCode { code: String },
    OkPieceList { symbols: Vec<char> },
    OkFen { fen: String },
    Ok,
    Error { message: String }
}

impl Display for ServerCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerCommand::OkColor { color } => {
                f.write_str(&format!("Ok {color:?}"))
            }
            ServerCommand::OkCode { code } => {
                f.write_str(&format!("Ok {code}"))
            }
            ServerCommand::OkPieceList { .. } => {
                f.write_str("Ok TODO")
            }
            ServerCommand::OkFen { fen } => {
                f.write_str(&format!("Ok {fen}"))
            }
            ServerCommand::Ok => {
                f.write_str("Ok")
            }
            ServerCommand::Error { message } => {
                f.write_str(&format!("Error {message}"))
            }
        }
    }
}