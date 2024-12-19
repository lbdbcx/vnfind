use std::{
    net::{IpAddr, Ipv4Addr},
    path::{Path, PathBuf},
    sync::{Arc, LazyLock},
};

use crate::log;
use config::Config;

fn config() -> &'static Config {
    static CONFIG: LazyLock<Arc<Config>> = LazyLock::new(|| {
        Arc::new(
            Config::builder()
                .add_source(config::File::from(exe_dir().join("config.toml")))
                .add_source(config::Environment::default())
                .build()
                .unwrap_or_default(),
        )
    });
    &CONFIG
}

pub fn exe_dir() -> PathBuf {
    let path = std::env::current_exe();
    if path.is_err() {
        log::warn("Cannot get executable's path > using relative path!");
        return ".".into();
    }
    let path = path.unwrap();
    let path = path.parent();
    match path {
        None => {
            log::warn("Cannot get executable's path > using relative path!");
            ".".into()
        }
        Some(p) => p.into(),
    }
}

pub fn address() -> IpAddr {
    match config().get_string("addr") {
        Ok(s) => s.parse().unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
        Err(_) => IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
    }
}
pub fn port() -> u16 {
    config().get_int("port").map(|x| x as u16).unwrap_or(8000)
}

pub fn data_path() -> PathBuf {
    match config().get_string("data_path") {
        Ok(s) => s.into(),
        Err(_) => exe_dir().join("data"),
    }
}

pub fn web_path() -> PathBuf {
    match config().get_string("web_path") {
        Ok(s) => s.into(),
        Err(_) => exe_dir().join("www"),
    }
}

pub fn default_column() -> Vec<String> {
    const DEFAULT_COLUMN: [&str; 11] = [
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
    ];
    match config().get_array("default_column") {
        Ok(v) => v.iter().map(|x| x.to_string()).collect(),
        Err(_) => Vec::from(DEFAULT_COLUMN)
            .iter()
            .map(|&x| x.to_owned())
            .collect(),
    }
}
