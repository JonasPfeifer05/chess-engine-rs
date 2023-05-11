use std::collections::HashMap;
use std::hash::Hash;
use std::iter::Map;
use std::net::TcpStream;
use rand::Rng;
use crate::application::Application;
use crate::game_board::piece::Color;

pub struct ApplicationMap<'a> {
    code_to_application: HashMap<String, Application>,
    code_to_players: HashMap<String, Vec<(&'a TcpStream, Color)>>,
}

impl<'a> ApplicationMap<'a> {
    pub fn new_game(&mut self) -> Result<String, String> {
        let random = hex::encode(&rand::thread_rng().gen::<[u8; 16]>());

        self.code_to_application.insert(random.clone(), Application::default());

        Ok(random)
    }

    pub fn add_user(&mut self, code: String, user: &'a TcpStream) -> Result<(), String> {
        if !self.code_to_application.contains_key(&code) {
            return Err("No such game!".to_string());
        }

        if let Some(application) = self.code_to_application.get(&code) { application } else {
            return Err("No game associated with that code!".to_string());
        };

        if let Some(users) = self.code_to_players.get_mut(&code) {
            if users.len() == 2 { return Err("There are already two players in that game!".to_string()); }

            if users.len() == 0 {
                users.push((user, Color::White));
            } else {
                match users.get(0).unwrap().1 {
                    Color::White => {
                        users.push((user, Color::Black));
                    }
                    Color::Black => {
                        users.push((user, Color::White));
                    }
                }
            }
        }

        Ok(())
    }
}