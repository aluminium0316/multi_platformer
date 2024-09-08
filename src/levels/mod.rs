use std::collections::HashMap;

use crate::{hold::stone::Stone, platform::Platform, player::Player, spawners::cannon::Cannon};

mod level1;

pub struct LevelLoader;
type H<'a, T> = &'a mut HashMap<u64, T>;

impl LevelLoader {
    pub fn load(id: u32, platforms: H<Platform>, stones: H<Stone>, cannons: H<Cannon>) {
        match id {
            0 => {
                platforms.clear();
                stones.clear();
                cannons.clear();
            }
            1 => level1::level1(platforms, stones, cannons),
            _ => ()
        }
    }
}