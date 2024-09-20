use crate::{hold::stone::Stone, new_id, platform::{LineType, Platform}, projectiles::path::{Curve, Path, Spline}, spawners::cannon::Cannon};

use super::H;

pub fn level1(platforms: H<Platform>, stones: H<Stone>, cannons: H<Cannon>, paths: H<Path>) -> (f64, f64) {
    let mut platform = Platform::new(Some(4));
    platform.new_ln(-128.0, -70.0, -96.0, -48.0, LineType::Normal);
    platform.new_ln(-96.0, -48.0, 16.0, -60.0, LineType::Normal);
    platform.new_ln(16.0, -60.0, 78.0, -54.0, LineType::Normal);
    platform.new_ln(78.0, -54.0, 86.0, -78.0, LineType::Normal);
    // platform.new_ln(-128.0, -32.0, 128.0, 0.0, LineType::Normal);
    platform.new_ln(-128.0, -440.0, -93.0, -425.0, LineType::End);
    platform.new_ln(-128.0, -1024.0, -128.0, 0.0, LineType::Inv);
    platform.new_ln(128.0, -0.0, 128.0, -1024.0, LineType::Inv);
    platforms.insert(new_id(), platform);
    stones.insert(new_id(), Stone::new(0.0, -128.0));
    // cannons.insert(new_id(), Cannon::new(0.0, -128.0, 0.25, 60, 3.0));
    // let platform_id1 = new_id();
    // let mut platform1 = Platform::new(None);
    // platform1.new_ln(-64.0, -32.0, 64.0, -32.0, LineType::Normal);
    // platforms.insert(platform_id1, platform1);
    // let mut spline = Spline::new();
    // // spline.add_curve(Curve::new_c(-128.0, -32.0, 128.0, 96.0, -128.0, -32.0, 0.0, -160.0), 1024.0);
    // spline.add_curve(Curve::new_c(0.0, 0.0, 0.0, -64.0, 0.0, -64.0, 0.0, 0.0), 512.0);
    // spline.position();
    // paths.insert(new_id(), Path::new(spline, platform_id1));
    (0.0, -64.0)
}