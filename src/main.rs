use chess_engine_rs::game_board::piece::{Bishop, King, Knight, Pawn, Piece, PieceRegistry, Queen, Rook};
use chess_engine_rs::game_board::piece::Color::Black;

fn main() {
    PieceRegistry::register_symbol('p', Piece::new(Box::new(Pawn::default()), Black));
    PieceRegistry::register_symbol('r', Piece::new(Box::new(Rook), Black));
    PieceRegistry::register_symbol('n', Piece::new(Box::new(Knight), Black));
    PieceRegistry::register_symbol('b', Piece::new(Box::new(Bishop), Black));
    PieceRegistry::register_symbol('q', Piece::new(Box::new(Queen), Black));
    PieceRegistry::register_symbol('k', Piece::new(Box::new(King), Black));




}