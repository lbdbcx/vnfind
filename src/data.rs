use crate::{
    game::{DataProviver, Game},
    log,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{Read, Write},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct DataBase {
    count: u64,
    games: HashMap<u64, Game>,
}

const SAVE_NAME: &str = "game.dat";

impl DataBase {
    pub fn empty() -> Self {
        Self {
            count: 0,
            games: HashMap::new(),
        }
    }
    fn default_path() -> String {
        let path = std::env::current_exe();
        if path.is_err() {
            log::warn("Cannot get executable's path > using relative path!");
            return SAVE_NAME.to_string();
        }
        let path = path.unwrap();
        let path = path.parent();
        if path.is_none() {
            log::warn("Cannot get executable's path > using relative path!");
            return SAVE_NAME.to_string();
        }
        let path = path.unwrap();
        let path = path.join(SAVE_NAME);
        let path = path.to_str();
        if path.is_none() {
            log::warn("Cannot get executable's path > using relative path!");
            return SAVE_NAME.to_string();
        }
        path.unwrap().to_string()
    }
    pub fn save(&self) {
        let path = Self::default_path();
        let json = serde_json::to_string(self);
        if json.is_err() {
            log::error(&format!(
                "In data.rs > DataBase::save() > serde_json::to_string() | {}",
                json.err().unwrap()
            ));
            return;
        }
        let json = json.unwrap();

        let f = File::create(&path);
        if f.is_err() {
            log::error(&format!(
                "In data.rs > DataBase::save() > File::create({}) | {}",
                path,
                f.err().unwrap()
            ));
            return;
        }
        let mut f = f.unwrap();

        if let Err(e) = f.write(json.as_bytes()) {
            log::error(&format!(
                "In data.rs > DataBase::save() > f.write() | {}",
                e
            ));
        }
    }
    pub fn load(path: Option<&str>) -> Option<Self> {
        let path = match path {
            Some(p) => p.to_string(),
            None => Self::default_path(),
        };

        let f = File::open(&path);
        if f.is_err() {
            log::error(&format!(
                "In data.rs > DataBase::load() > File::open({}) | {}",
                path,
                f.err().unwrap()
            ));
            return None;
        }
        let mut f = f.unwrap();

        let mut s = String::new();
        if let Err(e) = f.read_to_string(&mut s) {
            log::error(&format!(
                "In data.rs > DataBase::load() > f.read_to_string() | {}",
                e
            ));
            return None;
        }

        let res = serde_json::from_str::<DataBase>(&s);
        if res.is_err() {
            log::error(&format!(
                "In data.rs > DateBase::load() > serde_json::from_str() | {} |\n{}",
                res.err().unwrap(),
                s
            ));
            return None;
        };
        res.ok()
    }
    pub fn default() -> Self {
        Self::load(None).unwrap_or(Self::empty())
    }
    pub fn insert(&mut self, mut g: Game) {
        self.count += 1;
        g.id = self.count;
        self.games.insert(self.count, g);
        self.save();
    }
    pub fn modify(&mut self, id: u64, new_game: Game) {
        let _ = self.games.insert(id, new_game);
        self.save();
    }
}

impl DataProviver for DataBase {
    fn get_game(&self, id: u64) -> Option<&Game> {
        self.games.get(&id)
    }
    fn get_all_id(&self) -> HashSet<u64> {
        self.games.keys().copied().collect()
    }
}
impl DataProviver for Box<DataBase> {
    fn get_game(&self, id: u64) -> Option<&Game> {
        self.games.get(&id)
    }

    fn get_all_id(&self) -> HashSet<u64> {
        self.games.keys().copied().collect()
    }
}
