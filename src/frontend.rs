use std::cmp::Ordering;

use eframe::egui;
use egui::ScrollArea;

use crate::{
    data::DataBase,
    game::{DataProviver, GameList},
    log,
};

pub fn run(db: Box<DataBase>) -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Vnfind ver0.1 name",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(FrontEnd::new(db)))
        }),
    )
}

struct FrontEnd {
    db: Box<DataBase>,
    current_game: GameList,
    current_colomn: Vec<String>,
    font_id: egui::FontId,
    sort_by: String,
    rev: bool,
}

impl FrontEnd {
    const HEIGHT: f32 = 32.0;
    fn new(db: Box<DataBase>) -> Self {
        let mut list = GameList::new();
        list.set_all(&db);
        Self {
            db,
            current_game: list,
            current_colomn: vec!["story".to_string(), "draw".to_string()],
            font_id: egui::FontId::monospace(22.0),
            sort_by: String::new(),
            rev: false,
        }
    }
    fn scores_ui(&mut self, ui: &mut egui::Ui) {
        use egui::RichText;
        use egui_extras::{Column, TableBuilder};
        let mut table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto().at_least(100.0))
            .columns(Column::auto(), 2)
            .sense(egui::Sense::click());

        table
            .header(Self::HEIGHT, |mut header| {
                header.col(|ui| {
                    ui.strong(RichText::new("Rank").font(self.font_id.clone()));
                });
                header.col(|ui| {
                    ui.strong(RichText::new("name").font(self.font_id.clone()));
                });
                for title in self.current_colomn.clone() {
                    header.col(|ui| {
                        let show_title = title.clone();
                        let show_title = if title == self.sort_by {
                            if self.rev {
                                show_title + " ⬇"
                                //show_title + " ↓"
                            } else {
                                show_title + " ⬆"
                                //show_title + " ↑"
                            }
                        } else {
                            show_title
                        };
                        if ui
                            .button(
                                RichText::new(show_title)
                                    .font(self.font_id.clone())
                                    .strong(),
                            )
                            .clicked()
                        {
                            if title == self.sort_by {
                                if self.rev {
                                    self.rev = false;
                                } else {
                                    self.sort_by = String::new();
                                }
                            } else {
                                self.rev = true;
                                self.sort_by = title;
                            }
                        }
                    });
                }
            })
            .body(|mut body| {
                let mut list: Vec<u64> = self.current_game.games.iter().copied().collect();
                if self.sort_by == *"" {
                    list.sort();
                } else {
                    list.sort_by(|&a, &b| {
                        let a_game = match self.db.get_game(a) {
                            None => {
                                super::log::error(&format!("no game id {}", a));
                                return Ordering::Less;
                            }
                            Some(x) => x,
                        };
                        let b_game = match self.db.get_game(b) {
                            None => {
                                crate::log::error(&format!("no game id {}", b));
                                return Ordering::Less;
                            }
                            Some(x) => x,
                        };
                        let a_value = a_game.get_any(&self.sort_by);
                        let b_value = b_game.get_any(&self.sort_by);
                        let a_num = a_value.parse::<f64>();
                        let b_num = b_value.parse::<f64>();
                        match (a_num, b_num) {
                            (Ok(a), Ok(b)) => a.total_cmp(&b),
                            (Ok(_), Err(_)) => Ordering::Greater,
                            (Err(_), Ok(_)) => Ordering::Less,
                            (Err(_), Err(_)) => a.cmp(&b),
                        }
                    });
                    if self.rev {
                        list.reverse();
                    }
                }
                let total = list.len();
                body.rows(Self::HEIGHT, total, |mut row| {
                    let row_index = row.index();
                    let game_id = list[row_index];
                    let game = match self.db.get_game(game_id) {
                        None => {
                            log::error(&format!("no game id {}", game_id));
                            panic!()
                        }
                        Some(x) => x,
                    };
                    row.col(|ui| {
                        ui.label(
                            RichText::new((row_index + 1).to_string()).font(self.font_id.clone()),
                        );
                    });
                    row.col(|ui| {
                        ui.label(
                            RichText::new(game.get_property("name").unwrap_or_default())
                                .font(self.font_id.clone()),
                        );
                    });
                    for title in &self.current_colomn {
                        row.col(|ui| {
                            ui.label(RichText::new(game.get_any(title)).font(self.font_id.clone()));
                        });
                    }
                });
            });
    }
}

impl eframe::App for FrontEnd {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(egui::RichText::new("Game Scores").font(egui::FontId::proportional(32.0)));
            /*ui.image(egui::include_image!(
                "/home/user/data/image/2/EdXCgAYVcAERdE7.png"
            ));*/
            ui.vertical_centered(|ui| {
                ScrollArea::vertical()
                    .auto_shrink(false)
                    .scroll_bar_visibility(
                        egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded,
                    )
                    .show(ui, |ui| {
                        self.scores_ui(ui);
                    });
            });
        });
    }
}
