use crate::game_board::piece::Color;
use crate::game_board::position::Position;

#[derive(Debug)]
pub enum ClientCommand {
    Join { code: String },
    Leave,
    New { fen: String },
    Killed { color: Color },
    Fen,
    Move { from: Position, to: Position }
}

pub enum ServerCommand {
    OkColor { color: Color },
    OkCode { code: String },
    OkPieceList { symbols: Vec<char> },
    OkFen { fen: String },
    Ok,
    Error { message: String }
}