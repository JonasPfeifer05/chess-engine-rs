use std::any::Any;
use std::collections::HashMap;
use std::io::stdin;
use std::net::{Ipv4Addr, SocketAddrV4};
use chess_engine_rs::application::Application;
use chess_engine_rs::csp::csp_parser::CSPParser;

fn main() {
    let mut input = String::new();
    loop {
        stdin().read_line(&mut input).unwrap();
        let command = CSPParser::parse_client_command(&input);
        input.clear();
        println!("{:?}", command);
    }
}