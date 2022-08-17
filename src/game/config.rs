use std::fs;

use serde::Deserialize;
use toml;

/// contain the game configurations from the config.toml file
#[derive(Deserialize)]
pub(super) struct Config {
    pub display: MonitorConf,
    pub paddle: PaddleConf,
    pub player_location: PlayerConf,
    pub bot_location: BotConf,
    pub ball: BallConf,
}

impl Config {
    pub fn new(filename: &str) -> Self {
        let content = fs::read_to_string(filename).unwrap();
        toml::from_str(&content).unwrap()
    }
}

#[derive(Deserialize)]
pub(super) struct BallConf {
    pub radius: u8,
    pub speed_x: u8,
    pub speed_y: u8,
    pub x: u16,
    pub y: u16,
}

#[derive(Deserialize)]
pub(super) struct BotConf {
    pub x: u16,
    pub y: u16,
}

#[derive(Deserialize)]
pub(super) struct PlayerConf {
    pub x: u16,
    pub y: u16,
}

#[derive(Deserialize)]
pub(super) struct PaddleConf {
    pub height: u16,
    pub width: u16,
    pub speed_x: u16,
    pub speed_y: u16,
}

#[derive(Deserialize)]
pub(super) struct MonitorConf {
    pub height: u16,
    pub width: u16,
    pub fps: u8,
}
