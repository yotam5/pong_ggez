use serde::{Deserialize};
use toml;
use std::fs;

#[derive(Deserialize)]
pub(super) struct Config
{
    pub(super) display: MonitorConf
}

impl Config
{
    pub(super) fn new(filename: &str) -> Self
    {
       let content = fs::read_to_string(filename).unwrap();
       toml::from_str(&content).unwrap()       
    } 
}

#[derive(Deserialize)]
pub(super) struct MonitorConf
{
    pub(super) height: u16,
    pub(super) width: u16,
    pub(super) fps: u8,
}