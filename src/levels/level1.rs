use crate::{hold::stone::Stone, new_id, platform::Platform, spawners::cannon::Cannon};

use super::H;

pub fn level1(platforms: H<Platform>, stones: H<Stone>, cannons: H<Cannon>) {
    platforms.insert(new_id(), Platform::new());
    stones.insert(new_id(), Stone::new(0.0, 0.0));
    cannons.insert(new_id(), Cannon::new(0.0, -128.0));
}