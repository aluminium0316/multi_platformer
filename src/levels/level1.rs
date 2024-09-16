use crate::{hold::stone::Stone, new_id, platform::{LineType, Platform}, projectiles::path::{Curve, Path, Spline}, spawners::cannon::Cannon};

use super::H;

pub fn level1(platforms: H<Platform>, stones: H<Stone>, cannons: H<Cannon>, paths: H<Path>) -> (f64, f64) {
    let mut platform = Platform::new();
    platform.new_ln(-128.0, -32.0, 128.0, 0.0, LineType::Normal);
    // platform.new_ln(-128.0, -32.0, 128.0, 0.0, LineType::Normal);
    platform.new_ln(-128.0, -32.0, -96.0, -32.0, LineType::End);
    platform.new_ln(-128.0, -1024.0, -128.0, 0.0, LineType::Inv);
    platform.new_ln(128.0, -0.0, 128.0, -1024.0, LineType::Inv);
    platforms.insert(new_id(), platform);
    stones.insert(new_id(), Stone::new(0.0, -128.0));
    let cannon_id = new_id();
    cannons.insert(cannon_id, Cannon::new(0.0, -128.0, 0.25, 60, 3.0));
    let mut spline = Spline::new();
    spline.add_curve(Curve::new_c(-128.0, -32.0, 128.0, 96.0, -128.0, -32.0, 0.0, -160.0), 1024.0);
    spline.position();
    paths.insert(new_id(), Path::new(spline, cannon_id));
    (0.0, -32.0)
}