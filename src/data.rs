use crate::{
    game::Game,
    list::{DataProviver, GameList},
    log,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs::{read_to_string, write, File},
    io::{Read, Write},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct DataBaseStore {
    count: u64,
    games: HashMap<u64, Game>,
    lists: Vec<GameList>,
}

#[derive(Debug)]
pub struct DataBase {
    store: DataBaseStore,
    pub tag_set: HashSet<String>,
    pub property_set: HashSet<String>,
}
const SAVE_NAME: &str = "vnfind.dat";
impl DataBaseStore {
    pub fn empty() -> Self {
        Self {
            count: 0,
            games: HashMap::new(),
            lists: Vec::new(),
        }
    }

    fn save(&self) {
        let path = crate::config::data_path().join(SAVE_NAME);
        let json = serde_json::to_string(self);
        if json.is_err() {
            log::error(&format!(
                "In data.rs > DataBase::save() > serde_json::to_string() | {}",
                json.err().unwrap()
            ));
            return;
        }
        let json = json.unwrap();

        if let Err(e) = write(&path, json.as_bytes()) {
            log::error(&format!(
                "In data.rs > DataBase::save() > fs::write({:?}) | {}",
                path, e
            ));
        }
    }
    fn load(path: Option<&str>) -> Option<Self> {
        let path = match path {
            Some(p) => p.into(),
            None => super::config::data_path().join(SAVE_NAME),
        };

        let s = match read_to_string(&path) {
            Ok(x) => x,
            Err(e) => {
                log::error(&format!(
                    "In data.rs > DataBase::load() > fs::read_to_string({:?}) | {}",
                    path, e
                ));
                return None;
            }
        };

        let res = serde_json::from_str::<DataBaseStore>(&s);
        match res {
            Err(e) => {
                log::error(&format!(
                    "In data.rs > DateBase::load() > serde_json::from_str() | {} |\n{}",
                    e, s
                ));
                std::process::exit(0);
            }
            Ok(x) => Some(x),
        }
    }
    fn default() -> Self {
        Self::load(None).unwrap_or(Self::empty())
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
    pub fn insert(&mut self, mut g: Game) -> u64 {
        self.update(&g);
        self.store.count += 1;
        g.id = self.store.count;
        self.store.games.insert(self.store.count, g);
        self.store.save();
        self.store.count
    }
    pub fn modify(&mut self, id: u64, new_game: Game) {
        self.update(&new_game);
        let _ = self.store.games.insert(id, new_game);
        self.store.save();
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
    pub fn new_list(&mut self, name: Option<&str>, l: Vec<u64>) -> usize {
        let l = l
            .iter()
            .filter(|&&x| self.get_game(x).is_some())
            .copied()
            .collect::<HashSet<u64>>();
        let cnt = self.store.lists.len();
        self.store.lists.push(GameList {
            games: l,
            name: name
                .map(|x| x.to_owned())
                .unwrap_or(format!("List {}", cnt + 1)),
        });
        self.store.save();
        cnt
    }
    pub fn del_list(&mut self, lid: usize) {
        let _ = self.store.lists.remove(lid);
        self.store.save();
    }
    pub fn all_list(&self) -> String {
        let res = self.store.lists.iter().map(|x| &x.name).collect::<Vec<_>>();
        serde_json::to_string(&res).unwrap_or_else(|e| {
            log::error(&format!(
                "In data.rs > DataBase::all_list() > serde_json::to_string | {}\n{:?}",
                e, res,
            ));
            "[]".to_owned()
        })
    }
    pub fn get_list(&self, lid: usize) -> Option<&GameList> {
        self.store.lists.get(lid)
    }
    pub fn push_to_list(&mut self, gid: u64, lid: usize) {
        let l = match self.store.lists.get_mut(lid) {
            Some(x) => x,
            None => return,
        };
        l.add_game(gid);
        self.store.save();
    }
    pub fn del_in_list(&mut self, gid: u64, lid: usize) {
        let l = match self.store.lists.get_mut(lid) {
            Some(x) => x,
            None => return,
        };
        l.del_game(gid);
        self.store.save();
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
