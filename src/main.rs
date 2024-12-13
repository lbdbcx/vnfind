#![allow(unused)]
#[allow(unused_imports)]
#[allow(unused_variables)]
mod config;
mod data;
mod frontend;
mod game;
mod log;
mod output;

use std::{
    cell::{Cell, RefCell},
    cmp::Ordering,
    fs::read_to_string,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock},
};

#[macro_use]
extern crate rocket;

use data::DataBase;
use game::{DataProviver, Filter, Game, GameList};
use log::error;
use output::Table;
use rocket::{
    data::{Data, ToByteUnit},
    fs::NamedFile,
    response::{content::RawHtml, Flash, Redirect},
    serde::json::Json,
};

static DB: OnceLock<Mutex<DataBase>> = OnceLock::new();
macro_rules! db {
    () => {
        DB.get().unwrap().lock().unwrap()
    };
}
pub(crate) use db;

#[launch]
fn launch() -> _ {
    DB.set(Mutex::new(DataBase::default())).unwrap_or_else(|_| {
        error("In main.rs > launch() > DB.set() | DB is set before.");
    });
    rocket::build().mount(
        "/",
        routes![index, files, sort, add_game, del_game, edit_game, get_game],
    )
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("www/").join(file)).await.ok()
}

#[get("/")]
async fn index() -> Redirect {
    Redirect::to(uri!("/index.html"))
    // NamedFile::open("index.html").await.unwrap()
}

#[get("/sort?<key>&<rev>&<num>&<page>")]
async fn sort(
    key: Option<&str>,
    rev: Option<bool>,
    num: Option<usize>,
    page: Option<usize>,
) -> Json<output::Table> {
    let key = key.unwrap_or("结束时间");
    let num = num.unwrap_or(500);
    let rev = rev.unwrap_or(false);
    let page = page.unwrap_or(1) - 1;

    let db = db!();
    let games = db.get_all_id();
    let mut games = games.iter().copied().collect::<Vec<_>>();
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
                if a_value.cmp(&b_value) != Ordering::Equal {
                    a_value.cmp(&b_value)
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

    let start = (games.len() - 1).min(num * page);
    let end = games.len().min(num * (page + 1));
    Json(Table::from(
        &games[start..end],
        vec![
            "id",
            "标题",
            "剧情",
            "画面",
            "角色",
            "感情",
            "玩法",
            "日常",
            "色情",
            "声音",
            "结束时间",
        ],
    ))
}

#[get("/get_game?<id>")]
async fn get_game(id: u64) -> Option<Json<Game>> {
    db!().get_game(id).map(|e| Json(e.clone()))
}

#[post("/add_game", data = "<game>")]
async fn add_game(game: Json<Game>) {
    db!().insert(game.0);
}

#[delete("/del_game")]
async fn del_game() {}

#[post("/edit_game?<id>", data = "<game>")]
async fn edit_game(game: Json<Game>, id: u64) {
    assert_eq!(game.id, id);
    db!().modify(id, game.0);
}
