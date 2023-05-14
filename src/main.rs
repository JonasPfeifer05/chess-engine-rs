use mpsc::channel;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::windows::io::{AsSocket, BorrowedSocket};
use std::sync::{mpsc};
use std::sync::mpsc::Sender;
use std::thread;
use chess_engine_rs::csp::command::{ClientCommand, ServerCommand};
use chess_engine_rs::csp::csp_parser::CSPParser;
use chess_engine_rs::game_board::piece::{Bishop, King, Knight, Pawn, Piece, PieceRegistry, Queen, Rook};
use chess_engine_rs::game_board::piece::Color::Black;
use chess_engine_rs::networking::application_map::ApplicationMap;

fn handle_client(mut stream: TcpStream, request_tx: Sender<(Sender<ServerCommand>, ClientCommand)>) {
    let (tx, rx) = channel();

    let mut peek = vec![0; 1];
    let mut command_buffer = [0; 200];
    loop {
        if stream.peek(&mut peek).is_err() || peek[0] == 0 {
            break;
        }


        let size = stream.read(&mut command_buffer);

        if let Err(_) = size { break; } else if let Ok(size) = size {
            let command = String::from_utf8_lossy(&command_buffer[0..size]).to_string();
            let command = CSPParser::parse_client_command(&command);

            if command.is_none() {
                stream.write(b"Error").expect("Failed to write to client!");
                continue;
            }
            let mut command = command.unwrap();
            match &mut command {
                ClientCommand::Join { peer, .. } => { *peer = Some(stream.peer_addr().unwrap()); }
                ClientCommand::Leave { peer } => { *peer = Some(stream.peer_addr().unwrap()); }
                ClientCommand::Fen { peer } => { *peer = Some(stream.peer_addr().unwrap()); }
                ClientCommand::Move { peer, .. } => { *peer = Some(stream.peer_addr().unwrap()); }
                _ => {}
            }

            request_tx.send((tx.clone(), command)).expect("Failed to write to channel!");
        }

        let response = rx.recv().unwrap();

        stream.write(format!("{response}").as_bytes()).expect("failed to write to client");
    }
}

fn main() {
    PieceRegistry::register_symbol('p', Piece::new(Box::new(Pawn::default()), Black));
    PieceRegistry::register_symbol('r', Piece::new(Box::new(Rook), Black));
    PieceRegistry::register_symbol('n', Piece::new(Box::new(Knight), Black));
    PieceRegistry::register_symbol('b', Piece::new(Box::new(Bishop), Black));
    PieceRegistry::register_symbol('q', Piece::new(Box::new(Queen), Black));
    PieceRegistry::register_symbol('k', Piece::new(Box::new(King), Black));

    let (tx, rx) = channel();

    let listener_handle = thread::spawn(move || {
        let listener = TcpListener::bind("127.0.0.1:2222").unwrap();

        for stream in listener.incoming() {
            let tx = tx.clone();
            let stream = stream.unwrap();

            thread::spawn(move || handle_client(stream, tx));
        }
    });

    let mut application_map = ApplicationMap::default();

    loop {
        let (tx, command) = rx.recv().unwrap();

        match command {
            ClientCommand::New { fen } => {
                let code = application_map.new_game().unwrap();

                if fen != "default" {
                    let application = application_map.get_application(&code).unwrap();

                    if application.load_fen(&fen).is_err() {
                        tx.send(ServerCommand::Error { message: "Failed to load fen".to_string() });
                    };
                } else {
                    tx.send(ServerCommand::OkCode { code });
                }
            }
            ClientCommand::Join { code, peer } => {
                let result = application_map.add_user(&code, peer.unwrap());

                if let Err(err) = result {
                    tx.send(ServerCommand::Error { message: err });
                } else if let Ok(color) = result {
                    tx.send(ServerCommand::OkColor { color });
                }
            }
            ClientCommand::Leave { peer } => {
                let peer = peer.unwrap();

                if let Err(err) = application_map.get_code_to_user(&peer) {
                    tx.send(ServerCommand::Error { message: err });
                } else if let Ok(code) = application_map.get_code_to_user(&peer) {
                    application_map.remove_user(&code, &peer);

                    tx.send(ServerCommand::Ok);
                }
            }
            ClientCommand::Killed { .. } => {
                tx.send(ServerCommand::Ok);
            }
            ClientCommand::Fen { peer } => {
                let peer = peer.unwrap();

                if let Err(err) = application_map.get_code_to_user(&peer) {
                    tx.send(ServerCommand::Error { message: err });
                } else if let Ok(code) = application_map.get_code_to_user(&peer) {
                    let fen = application_map.get_application(&code).unwrap().fen();

                    tx.send(ServerCommand::OkFen { fen });
                }
            }
            ClientCommand::Move { from, to, peer } => {
                let peer = peer.unwrap();

                if let Err(err) = application_map.get_code_to_user(&peer) {
                    tx.send(ServerCommand::Error { message: err });
                } else if let Ok(code) = application_map.get_code_to_user(&peer) {
                    let application = application_map.get_application(&code).unwrap();

                    if let Err(err) = application.movee(&from, &to) {
                        tx.send(ServerCommand::Error { message: err });
                    } else {
                        tx.send(ServerCommand::Ok);
                    }
                }
            }
        }
    }
}