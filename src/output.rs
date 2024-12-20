use std::sync::Arc;

use serde::Serialize;

use crate::list::DataProviver;

#[macro_use]
use super::DB;
use super::db;

#[derive(Serialize)]
pub struct Table {
    column: Vec<String>,
    row: Vec<Vec<String>>,
}

impl Table {
    pub fn from(games: &[u64], columns: Vec<String>) -> Self {
        let mut row = vec![];
        let db = db!();
        for &game_id in games {
            let game = db.get_game(game_id).unwrap();
            let mut this_row = vec![];
            for col in &columns {
                this_row.push(if col == "id" {
                    game.id.to_string()
                } else {
                    game.get_any(col).to_owned()
                });
            }
            row.push(this_row);
        }
        Self {
            column: columns,
            row,
        }
    }
}
