use serde::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize)]
pub(super) struct Config {
    pub display: MonitorConf,
    pub paddle: PaddleConf,
}

impl Config {
    pub fn new(filename: &str) -> Self {
        let content = fs::read_to_string(filename).unwrap();
        toml::from_str(&content).unwrap()
    }
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
