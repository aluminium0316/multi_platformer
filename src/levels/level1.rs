use crate::{hold::stone::Stone, new_id, platform::{LineType, Platform}, spawners::cannon::Cannon};

use super::H;

pub fn level1(platforms: H<Platform>, stones: H<Stone>, cannons: H<Cannon>) -> (f64, f64) {
    let mut platform = Platform::new();
    platform.new_ln(-128.0, -32.0, 128.0, 0.0, LineType::Normal);
    platform.new_ln(-128.0, -1024.0, -128.0, 0.0, LineType::Inv);
    platform.new_ln(128.0, -0.0, 128.0, -1024.0, LineType::Inv);
    platforms.insert(new_id(), platform);
    stones.insert(new_id(), Stone::new(0.0, 0.0));
    // cannons.insert(new_id(), Cannon::new(0.0, -128.0));
    (0.0, -32.0)
}