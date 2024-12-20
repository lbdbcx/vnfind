#![allow(unused)]
mod config;
mod data;
// mod frontend;
mod game;
mod list;
mod log;
mod output;

use std::{
    cell::{Cell, RefCell},
    cmp::Ordering,
    fs::read_to_string,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    sync::{Arc, LazyLock, Mutex, OnceLock},
};

#[macro_use]
extern crate rocket;

use data::DataBase;
use game::Game;
use list::{DataProviver, Filter, GameList};
use log::error;
use output::Table;
use rocket::{
    data::{Data, ToByteUnit},
    fs::NamedFile,
    response::{content::RawHtml, Flash, Redirect},
    serde::json::Json,
};

static DB: LazyLock<Mutex<DataBase>> = LazyLock::new(|| Mutex::new(DataBase::default()));
macro_rules! db {
    () => {
        DB.lock().unwrap()
    };
}
pub(crate) use db;

#[launch]
fn launch() -> _ {
    let cfg = rocket::Config::figment()
        .merge(("address", crate::config::address()))
        .merge(("port", crate::config::port()));

    rocket::custom(cfg).mount(
        "/",
        routes![
            index,
            files,
            search,
            add_game,
            del_game,
            edit_game,
            get_game,
            get_tag,
            get_property,
            get_comment,
            set_comment,
            new_list,
            del_list,
            all_list,
            get_list,
            push_to_list,
            del_in_list,
        ],
    )
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(crate::config::web_path().join(file))
        .await
        .ok()
}

#[get("/")]
async fn index() -> Redirect {
    Redirect::to(uri!("/index.html"))
    // NamedFile::open("index.html").await.unwrap()
}

#[get("/search?<query>&<key>&<rev>&<num>&<page>&<columns>")]
async fn search(
    query: Option<&str>,
    key: Option<&str>,
    rev: Option<bool>,
    num: Option<usize>,
    page: Option<usize>,
    columns: Option<&str>,
) -> Json<output::Table> {
    let query = query.unwrap_or_default();
    let key = key.unwrap_or("结束时间");
    let num = num.unwrap_or(500);
    let rev = rev.unwrap_or(false);
    let page = page.unwrap_or(1) - 1;
    let columns = columns
        .map(|v| {
            let mut res = v.split(['|', '｜']).collect::<Vec<_>>();
            if !res.contains(&"标题") {
                res.insert(0, "标题");
            }
            if !res.contains(&"id") {
                res.insert(0, "id");
            }
            res.iter().map(|x| x.to_string()).collect()
        })
        .unwrap_or(crate::config::default_column());

    let db = db!();
    let mut games = db.search(query);
    // sort
    games.sort_by(|&a, &b| {
        let a_game = match db.get_game(a) {
            None => {
                log::error(&format!("no game id {}", a));
                return Ordering::Less;
            }
            Some(x) => x,
        };
        let b_game = match db.get_game(b) {
            None => {
                log::error(&format!("no game id {}", b));
                return Ordering::Less;
            }
            Some(x) => x,
        };
        let a_value = a_game.get_any(key);
        let b_value = b_game.get_any(key);
        let a_num = a_value.parse::<f64>();
        let b_num = b_value.parse::<f64>();
        match (a_num, b_num) {
            (Ok(a), Ok(b)) => a.total_cmp(&b),
            (Ok(_), Err(_)) => Ordering::Greater,
            (Err(_), Ok(_)) => Ordering::Less,
            (Err(_), Err(_)) => {
                if a_value.cmp(b_value) != Ordering::Equal {
                    a_value.cmp(b_value)
                } else {
                    a.cmp(&b)
                }
            }
        }
    });
    if !rev {
        games.reverse();
    }
    drop(db);

    let start = games.len().min(num * page);
    let end = games.len().min(num * (page + 1));

    Json(Table::from(&games[start..end], columns))
}

#[get("/get_game?<id>")]
async fn get_game(id: u64) -> Option<Json<Game>> {
    db!().get_game(id).map(|e| Json(e.clone()))
}

#[get("/get_tag")]
async fn get_tag() -> Option<String> {
    serde_json::to_string(&db!().tag_set).ok()
}

#[get("/get_property")]
async fn get_property() -> Option<String> {
    serde_json::to_string(&db!().property_set).ok()
}

#[post("/add_game", data = "<game>")]
async fn add_game(game: Json<Game>) -> String {
    db!().insert(game.0).to_string()
}

#[delete("/del_game")]
async fn del_game() {}

#[post("/edit_game?<id>", data = "<game>")]
async fn edit_game(game: Json<Game>, id: u64) {
    assert_eq!(game.id, id);
    db!().modify(id, game.0);
}
#[get("/get_comment?<id>")]
async fn get_comment(id: u64) -> String {
    db!().get_game(id).unwrap().load_comment()
}

#[post("/set_comment?<id>", data = "<s>")]
async fn set_comment(id: u64, s: &str) {
    db!().get_game(id).unwrap().save_comment(s);
}

#[post("/new_list?<name>", data = "<l>")]
async fn new_list(name: Option<&str>, l: Json<Vec<u64>>) -> String {
    let l = l.0;
    println!("list : {:?}", l);
    db!().new_list(name, l).to_string()
}

#[delete("/del_list?<lid>")]
async fn del_list(lid: usize) {
    db!().del_list(lid);
}

#[get("/get_list?<id>")]
async fn get_list(id: usize) -> Option<String> {
    match db!().get_list(id) {
        Some(l) => match serde_json::to_string(l) {
            Ok(s) => Some(s),
            Err(e) => {
                log::error(&format!(
                    "In main.rs > get_list > serde_json::to_string() | {e}\n{l:?}"
                ));
                None
            }
        },
        None => None,
    }
}

#[get("/all_list")]
async fn all_list() -> String {
    let res = db!().all_list();
    println!("all_list : {}", res);
    res
}

#[get("/push_to_list?<gid>&<lid>")]
async fn push_to_list(gid: u64, lid: usize) {
    db!().push_to_list(gid, lid);
}

#[get("/del_in_list?<gid>&<lid>")]
async fn del_in_list(gid: u64, lid: usize) {
    db!().del_in_list(gid, lid);
}
