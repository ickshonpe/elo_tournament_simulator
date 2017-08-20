extern crate serde_yaml;

use player::Player;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;

pub fn load_players() -> Vec<Player> {
    let mut file = File::open("players.yaml").unwrap();
    let mut data: String = String::new();
    file.read_to_string(&mut data).unwrap();
    let players: Vec<Player> = serde_yaml::from_str(&data).unwrap();
    players
}