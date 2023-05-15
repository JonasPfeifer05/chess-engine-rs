use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use crate::csp::command::{ClientCommand, ServerCommand};
use crate::csp::csp_parser::CSPParser;
use crate::networking::application_map::GameMap;

pub struct Application {
    request_tx: Sender<(Sender<ServerCommand>, (ClientCommand, SocketAddr))>,
    request_rx: Receiver<(Sender<ServerCommand>, (ClientCommand, SocketAddr))>,

    listener_handle: Option<JoinHandle<()>>,

    game_map: GameMap,
}

impl Default for Application {
    fn default() -> Self {
        let (tx, rx) = channel();

        Self {
            request_tx: tx,
            request_rx: rx,
            listener_handle: None,
            game_map: Default::default(),
        }
    }
}

impl Application {
    pub fn listen(&mut self) {
        if self.listener_handle.is_some() { return }

        let tx = self.request_tx.clone();

        self.listener_handle = Some(thread::spawn(move || {
            let listener = TcpListener::bind("127.0.0.1:2222").unwrap();

            for stream in listener.incoming() {
                let tx = tx.clone();
                let stream = stream.unwrap();

                thread::spawn(move || handle_client(stream, tx));
            }
        }));
    }

    pub fn process(&mut self) {
        let (tx, (command, peer)) = self.request_rx.recv().unwrap();

        let response = match command {
            ClientCommand::New { fen } => {
                let code = self.game_map.new_game().unwrap();
                let application = self.game_map.get_application(&code).unwrap();
                if &fen != "default" && application.load_fen(&fen).is_err() {
                    ServerCommand::Error { message: "Failed to load fen".to_string() }
                } else {
                    ServerCommand::OkCode { code }
                }
            }
            ClientCommand::Join { code } => {
                let add_result = self.game_map.add_user(&code, peer);

                if let Err(err) = add_result {
                    ServerCommand::Error { message: err }
                } else {
                    if self.game_map.get_user_count(&code) == 2 {
                        self.game_map.get_application(&code).unwrap().state.start_game();
                    }
                    ServerCommand::OkColor { color: add_result.unwrap() }
                }
            }
            ClientCommand::Leave => {
                let code_result = self.game_map.get_code_to_user(&peer);
                if let Err(err) = code_result {
                    ServerCommand::Error { message: err }
                } else {
                    let code = code_result.unwrap();
                    self.game_map.remove_user(&code, &peer);
                    self.game_map.get_application(&code).unwrap().state.stop_game();
                    ServerCommand::Ok
                }
            }
            ClientCommand::Killed { .. } => {
                ServerCommand::Error {message: "Not implemented jet".to_string()}
            }
            ClientCommand::Fen => {
                let code_result = self.game_map.get_code_to_user(&peer);

                if let Err(err) = code_result {
                    ServerCommand::Error { message: err }
                } else {
                    let fen = self.game_map.get_application(&code_result.unwrap()).unwrap().fen();
                    ServerCommand::OkFen { fen }
                }
            }
            ClientCommand::Move { from, to} => {
                let code_result = self.game_map.get_code_to_user(&peer);

                if let Err(err) = code_result {
                    ServerCommand::Error { message: err }
                } else {
                    let code = code_result.unwrap();
                    let color = self.game_map.get_color_of_user(&code, &peer).unwrap();
                    let application = self.game_map.get_application(&code).unwrap();

                    if let Err(err) = application.movee(&from, &to, &color) {
                        ServerCommand::Error { message: err }
                    } else {
                        ServerCommand::Ok
                    }
                }
            }
        };

        if let Err(err) = tx.send(response) {
            eprintln!("{}", err);
        }
    }
}

fn handle_client(mut stream: TcpStream, request_tx: Sender<(Sender<ServerCommand>, (ClientCommand, SocketAddr))>) {
    let (tx, rx) = channel();

    let mut peek = vec![0; 1];
    let mut command_buffer = [0; 200];
    loop {
        if stream.peek(&mut peek).is_err() || peek[0] == 0 {
            eprintln!("Failed to read from client! Disconnecting");
            break;
        }

        let size = stream.read(&mut command_buffer);

        if let Err(_) = size {
            eprintln!("Failed to read from client! Disconnecting");
            break;
        } else if let Ok(size) = size {
            let command = String::from_utf8_lossy(&command_buffer[0..size]).to_string();
            let command = CSPParser::parse_client_command(&command);

            if let Err(mut err) = command {
                err.insert_str(0, "Error ");
                stream.write(err.as_bytes()).expect("Failed to write to client!");
                continue;
            }

            let command = command.unwrap();

            request_tx.send(
                (
                    tx.clone(),
                    (command, stream.peer_addr().expect("Failed to get peer address!"))
                )
            ).expect("Failed to write to channel!");
        }

        let response = rx.recv().unwrap();
        stream.write(format!("{response}").as_bytes()).expect("failed to write to client");
    }
}