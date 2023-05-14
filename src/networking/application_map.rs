use std::collections::HashMap;
use std::hash::Hash;
use std::iter::Map;
use std::net::{SocketAddr, TcpStream};
use std::thread::Scope;
use rand::Rng;
use crate::application::Game;
use crate::game_board::piece::Color;
use crate::game_board::piece::Color::{Black, White};

pub struct GameMap {
    code_to_application: HashMap<String, Game>,
    code_to_players: HashMap<String, Vec<(SocketAddr, Color)>>,
}

impl Default for GameMap {
    fn default() -> Self {
        Self {
            code_to_players: Default::default(),
            code_to_application: Default::default(),
        }
    }
}

impl GameMap {
    pub fn get_code_to_user(&self, peer: &SocketAddr) -> Result<String, String> {
        for entries in self.code_to_players.iter() {
            for user in entries.1 {
                if &user.0 == peer { return Ok(entries.0.clone()); }
            }
        }
        return Err("User is not assigned to any game".to_string());
    }

    pub fn remove_user(&mut self, code: &str, peer: &SocketAddr) {
        let users = self.code_to_players.get_mut(code).unwrap();
        let index = if let Some(index) = users.iter().position(|user| &user.0 == peer) { index } else { return; };
        users.remove(index);
    }

    pub fn new_game(&mut self) -> Result<String, String> {
        let random = hex::encode(&rand::thread_rng().gen::<[u8; 16]>());

        self.code_to_application.insert(random.clone(), Game::default());
        self.code_to_players.insert(random.clone(), Vec::new());

        Ok(random)
    }

    pub fn get_application(&mut self, code: &str) -> Option<&mut Game> {
        self.code_to_application.get_mut(code)
    }

    pub fn add_user(&mut self, code: &str, user: SocketAddr) -> Result<Color, String> {
        if !self.code_to_application.contains_key(code) {
            return Err("No such game!".to_string());
        }

        if let Some(application) = self.code_to_application.get(code) { application } else {
            return Err("No game associated with that code!".to_string());
        };

        if let Some(users) = self.code_to_players.get_mut(code) {
            if users.len() == 2 { return Err("There are already two players in that game!".to_string()); }

            if users.len() == 0 {
                users.push((user, Color::White));
                return Ok(White);
            } else {
                match users.get(0).unwrap().1 {
                    Color::White => {
                        users.push((user, Color::Black));
                        return Ok(Black);
                    }
                    Color::Black => {
                        users.push((user, Color::White));
                        return Ok(White);
                    }
                }
            }
        } else {
            return Err("Internal Server Error!".to_string());
        }
    }
}