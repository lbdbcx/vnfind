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
pub struct DataBaseStore {
    count: u64,
    games: HashMap<u64, Game>,
}

#[derive(Debug)]
pub struct DataBase {
    store: DataBaseStore,
    pub tag_set: HashSet<String>,
    pub property_set: HashSet<String>,
}
const SAVE_NAME: &str = "game.dat";
impl DataBaseStore {
    pub fn empty() -> Self {
        Self {
            count: 0,
            games: HashMap::new(),
        }
    }

    fn save(&self) {
        let path = crate::data_path().join(SAVE_NAME);
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
                "In data.rs > DataBase::save() > File::create({:?}) | {}",
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
    fn load(path: Option<&str>) -> Option<Self> {
        let path = match path {
            Some(p) => p.into(),
            None => super::data_path().join(SAVE_NAME),
        };

        let f = File::open(&path);
        if f.is_err() {
            log::error(&format!(
                "In data.rs > DataBase::load() > File::open({:?}) | {}",
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

        let res = serde_json::from_str::<DataBaseStore>(&s);
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
    fn default() -> Self {
        Self::load(None).unwrap_or(Self::empty())
    }
    fn insert(&mut self, mut g: Game) -> u64 {
        self.count += 1;
        g.id = self.count;
        self.games.insert(self.count, g);
        self.save();
        self.count
    }
    fn modify(&mut self, id: u64, new_game: Game) {
        let _ = self.games.insert(id, new_game);
        self.save();
    }
}

impl DataBase {
    pub fn empty() -> Self {
        let mut res = Self {
            store: DataBaseStore::empty(),
            tag_set: HashSet::new(),
            property_set: HashSet::new(),
        };
        res.build();
        res
    }
    pub fn default() -> Self {
        let mut res = Self {
            store: DataBaseStore::default(),
            tag_set: HashSet::new(),
            property_set: HashSet::new(),
        };
        res.build();
        res
    }
    fn update(&mut self, g: &Game) {
        for k in g.property.keys() {
            self.property_set.insert(k.to_owned());
        }
        for t in &g.tag {
            self.tag_set.insert(t.to_owned());
        }
    }
    fn build(&mut self) {
        for game in self.store.games.values() {
            // why I cannot ?
            // self.update(game);
            for k in game.property.keys() {
                self.property_set.insert(k.to_owned());
            }
            for t in &game.tag {
                self.tag_set.insert(t.to_owned());
            }
        }
    }
    pub fn insert(&mut self, g: Game) -> u64 {
        self.update(&g);
        self.store.insert(g)
    }
    pub fn modify(&mut self, id: u64, new_game: Game) {
        self.update(&new_game);
        self.store.modify(id, new_game);
    }
    pub fn search(&self, query: &str) -> Vec<u64> {
        if query.is_empty() {
            return self.store.games.keys().copied().collect();
        }
        self.store
            .games
            .iter()
            .filter(|(_, g)| g.have(query))
            .map(|(id, _)| id)
            .copied()
            .collect()
    }
}

impl DataProviver for DataBase {
    fn get_game(&self, id: u64) -> Option<&Game> {
        self.store.games.get(&id)
    }
    fn get_all_id(&self) -> HashSet<u64> {
        self.store.games.keys().copied().collect()
    }
}
impl DataProviver for Box<DataBase> {
    fn get_game(&self, id: u64) -> Option<&Game> {
        self.store.games.get(&id)
    }

    fn get_all_id(&self) -> HashSet<u64> {
        self.store.games.keys().copied().collect()
    }
}
