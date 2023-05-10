use crate::game_board::piece::Color;

pub enum CSPCommand {
    // Server Responses
    Ok,
    Error,

    // Authorization
    Id { name: String },
    Leave { name: String },

    // Configuration
    New { fen: String },
    CancelNew,

    // Requests
    GetMoves { position: (u8,u8) },
    Killed { color: Color },
    Fen,

    // Executive
    Move { from: (u8,u8), to: (u8,u8) }
}