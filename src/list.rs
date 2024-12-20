use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::game::Game;
use crate::log;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameList {
    pub games: HashSet<u64>,
    pub name: String,
}

impl GameList {
    pub fn new() -> Self {
        Self {
            games: HashSet::new(),
            name: String::new(),
        }
    }
    pub fn set_name(&mut self, s: &str) {
        self.name = s.to_string();
    }
    pub fn set_all(&mut self, db: &impl DataProviver) {
        self.games = db.get_all_id();
    }
    pub fn add_game(&mut self, gid: u64) {
        let _ = self.games.insert(gid);
    }
    pub fn del_game(&mut self, gid: u64) {
        let _ = self.games.remove(&gid);
    }
    pub fn apply(&mut self, filt: Filter, db: &impl DataProviver) {
        self.games.retain(|&game_id| {
                let game = db.get_game(game_id);
                if let Some(game) = game {
                    game.satisfy(&filt)
                } else {
                    log::error(&format!(
                        "In game.rs > GameList::apply() > db.get_game({}) | return none which is unexpected",
                        game_id
                    ));
                    false
                }
            });
    }
    pub fn apply_many(&mut self, filts: Vec<Filter>, db: &impl DataProviver) {
        for f in filts {
            self.apply(f, db);
        }
    }
    pub fn len(&self) -> usize {
        self.games.len()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Filter {
    Have(String),
    PropertyEqual(String, String),
    NumEqual(String, f64),
    NumGreater(String, f64),
    NumLess(String, f64),
    Not(Box<Filter>),
    Or(Box<Filter>, Box<Filter>),
    And(Box<Filter>, Box<Filter>),
}

pub trait DataProviver {
    fn get_game(&self, id: u64) -> Option<&Game>;
    fn get_all_id(&self) -> HashSet<u64>;
}
