use std::collections::{HashMap, HashSet, VecDeque};

/* may totally change to a json object ? */
pub struct Game {
    id: u64, // local id, use VNDB
    name: String,
    release_date: u32, /* may change to a stronger date type */
    tag: HashSet<String>,
    numtag: HashMap<String, f32>, /* f32 or i32 ? */
    score_user: f32,
    score_bgm: f32,
    score_vndb: f32,
    /* friends' score ? */
}

impl Game {
    pub fn empty() -> Self {
        Self {
            id: 0, // ?????
            name: String::new(),
            release_date: 0,
            tag: HashSet::new(),
            numtag: HashMap::new(),
            score_bgm: 0.,
            score_user: 0.,
            score_vndb: 0.,
        }
    }
    pub fn from_name(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ..Self::empty()
        }
    }
    pub fn add_tag(&mut self, tag: &str) {
        let _ = self.tag.insert(tag.to_owned());
    }
    pub fn add_numtag(&mut self, tag: &str, num: f32) {
        let _ = self.numtag.insert(tag.to_owned(), num);
    }
}

pub struct List {
    // must sorted ! storage game id
    games: VecDeque<u64>,
    name: String,
}

impl List {
    fn apply(&mut self, filt: Filter) {}
    fn apply_many(&mut self, filts: Vec<Filter>) {
        for f in filts {
            self.apply(f);
        }
    }
}

pub struct Filter {}
