use mpsc::channel;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{mpsc};
use std::sync::mpsc::Sender;
use std::thread;
use chess_engine_rs::application::Application;
use chess_engine_rs::csp::command::{ClientCommand, ServerCommand};
use chess_engine_rs::csp::csp_parser::CSPParser;
use chess_engine_rs::networking::application_map::GameMap;

// fn handle_client(mut stream: TcpStream, request_tx: Sender<(Sender<ServerCommand>, (ClientCommand, SocketAddr))>) {
//     let (tx, rx) = channel();
//
//     let mut peek = vec![0; 1];
//     let mut command_buffer = [0; 200];
//     loop {
//         if stream.peek(&mut peek).is_err() || peek[0] == 0 {
//             eprintln!("Failed to read from client! Disconnecting");
//             break;
//         }
//
//         let size = stream.read(&mut command_buffer);
//
//         if let Err(_) = size {
//             eprintln!("Failed to read from client! Disconnecting");
//             break;
//         } else if let Ok(size) = size {
//             let command = String::from_utf8_lossy(&command_buffer[0..size]).to_string();
//             let command = CSPParser::parse_client_command(&command);
//
//             if let Err(mut err) = command {
//                 err.insert_str(0, "Error ");
//                 stream.write(err.as_bytes()).expect("Failed to write to client!");
//                 continue;
//             }
//
//             let mut command = command.unwrap();
//
//             request_tx.send(
//                 (
//                     tx.clone(),
//                     (command, stream.peer_addr().expect("Failed to get peer address!"))
//                 )
//             ).expect("Failed to write to channel!");
//         }
//
//         let response = rx.recv().unwrap();
//         stream.write(format!("{response}").as_bytes()).expect("failed to write to client");
//     }
// }

fn main() {
    let mut app = Application::default();
    app.listen();
    loop {
        app.process();
    }
}