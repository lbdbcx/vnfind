use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::log;

/* may totally change to a json object ? */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: u64, // local id
    pub property: HashMap<String, String>,
    // pub num_property: HashMap<String, f64>,
    pub tag: HashSet<String>,
}
// impl Ord for Game {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         u64::cmp(&self.id, &other.id)
//     }
// }
// impl PartialOrd for Game {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }
// impl Eq for Game {}
// impl PartialEq for Game {
//     fn eq(&self, other: &Self) -> bool {
//         u64::eq(&self.id, &other.id)
//     }
// }

impl Game {
    pub fn new() -> Self {
        Self {
            id: 0,
            tag: HashSet::new(),
            // num_property: HashMap::new(),
            property: HashMap::new(),
        }
    }
    pub fn add_tag(&mut self, tag: &str) {
        let _ = self.tag.insert(tag.to_owned());
    }
    // pub fn add_num_property(&mut self, tag: &str, num: f64) {
    //     let _ = self.num_property.insert(tag.to_owned(), num);
    // }
    pub fn add_property(&mut self, name: &str, value: &str) {
        let _ = self.property.insert(name.to_owned(), value.to_owned());
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tag.contains(tag)
    }
    pub fn get_property(&self, name: &str) -> Option<&str> {
        self.property.get(name).map(|x| x.as_str())
    }
    pub fn have(&self, s: &str) -> bool {
        if self.tag.contains(s) || self.property.contains_key(s) {
            true
        } else {
            for x in self.property.values() {
                if x.contains(s) {
                    return true;
                }
            }
            false
        }
    }
    pub fn get_num_property(&self, name: &str) -> Option<f64> {
        self.property
            .get(name)
            .map(|x| x.parse::<f64>())
            .transpose()
            .ok()?
    }
    pub fn get_any(&self, name: &str) -> &str {
        if let Some(s) = self.get_property(name) {
            return s;
        }
        if self.has_tag(name) {
            "*"
        } else {
            ""
        }
    }
    pub fn satisfy(&self, filt: &Filter) -> bool {
        match filt {
            Filter::Have(s) => self.have(s),
            Filter::PropertyEqual(key, value) => {
                let game_property = self.property.get(key);
                if let Some(s) = game_property {
                    s.contains(value)
                } else {
                    false
                }
            }
            Filter::NumEqual(key, num) => {
                let game_num = self.get_num_property(key);
                if let Some(game_num) = game_num {
                    game_num == *num
                } else {
                    false
                }
            }
            Filter::NumGreater(key, num) => {
                let game_num = self.get_num_property(key);
                if let Some(game_num) = game_num {
                    game_num > *num
                } else {
                    false
                }
            }
            Filter::NumLess(key, num) => {
                let game_num = self.get_num_property(key);
                if let Some(game_num) = game_num {
                    game_num < *num
                } else {
                    false
                }
            }
            Filter::Not(f) => !self.satisfy(f),
            Filter::Or(f1, f2) => self.satisfy(f1) || self.satisfy(f2),
            Filter::And(f1, f2) => self.satisfy(f1) && self.satisfy(f2),
        }
    }
}

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
