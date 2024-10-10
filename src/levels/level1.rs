use crate::{hold::stone::Stone, new_id, platform::{LineType, Platform}, projectiles::path::{Curve, Path, Spline}, spawners::cannon::Cannon};

use super::H;

pub fn level1(platforms: H<Platform>, stones: H<Stone>, cannons: H<Cannon>, paths: H<Path>) -> (f64, f64) {
    let mut platform = Platform::new(Some(4));
    platform.new_ln(-128.0, -70.0, -96.0, -48.0, LineType::Normal);
    platform.new_ln(-96.0, -48.0, 16.0, -60.0, LineType::Normal);
    platform.new_ln(16.0, -60.0, 78.0, -54.0, LineType::Normal);
    platform.new_ln(78.0, -54.0, 86.0, -78.0, LineType::Normal);
    platform.new_ln(86.0, -79.0, 78.0, -115.0, LineType::Normal);
    platform.new_ln(78.0, -115.0, 128.0, -122.0, LineType::Normal);
    platform.new_ln(4.0, -87.0, 43.0, -78.0, LineType::Normal);
    platform.new_ln(43.0, -78.0, 54.0, -90.0, LineType::Normal);
    // platform.new_ln(-72.0, -180.0, -38.0, -90.0, LineType::Normal);
    platform.new_poly(vec![
        (-72.0, -180.0),
        (-38.0, -173.0),
        (-48.0, -161.0),
        (-69.0, -168.0),
    ], LineType::Normal);
    platform.new_poly(vec![
        (-56.0, -280.0),
        (76.0, -276.0),
        (45.0, -261.0),
        (-11.0, -254.0),
        (-39.0, -208.0),
        (-70.0, -198.0),
    ], LineType::Normal);
    platform.new_poly(vec![
        (-128.0, -295.0),
        (-104.0, -284.0),
        (-96.0, -258.0),
        (-128.0, -240.0),
    ], LineType::Normal);
    platform.new_poly(vec![
        (129.0, -276.0),
        (129.0, -220.0),
        (108.0, -225.0),
    ], LineType::Normal);
    platform.new_poly(vec![
        (16.0, -436.0),
        (39.0, -321.0),
        (-47.0, -412.0),
    ], LineType::Normal);
    platform.new_ln(39.0, -321.0, 55.0, -301.0 , LineType::Normal);
    platform.new_poly(vec![
        (59.0, -440.0),
        (128.0, -448.0),
        (128.0, -330.0),
        (87.0, -323.0),
    ], LineType::Normal);
    platform.new_poly(vec![
        (-92.0, -423.0),
        (-96.0, -409.0),
        (-126.0, -400.0),
    ], LineType::Normal);
    // platform.new_ln(-128.0, -32.0, 128.0, 0.0, LineType::Normal);
    platform.new_ln(-128.0, -440.0, -93.0, -425.0, LineType::End);
    platform.new_ln(-128.0, -1024.0, -128.0, 0.0, LineType::Inv);
    platform.new_ln(128.0, -0.0, 128.0, -1024.0, LineType::Inv);
    // platform.new_ln(128.0, -0.0, 128.0, -1024.0, LineType::Inv);
    platforms.insert(new_id(), platform);

    let mut platform1 = Platform::new(None);
    let id1 = new_id();
    platform1.new_ln(-32.0, 8.0, 30.0, 7.0, LineType::Normal);
    platforms.insert(id1, platform1);
    let mut spline1 = Spline::new();
    spline1.add_curve(Curve::new_c(47.0, -113.0, 1.0, -128.0, 1.0, -128.0, -14.0, -158.0), 512.0);
    spline1.add_curve(Curve::new_c(-14.0, -158.0, 1.0, -128.0, 1.0, -128.0, 47.0, -113.0), 512.0);
    spline1.position();
    let path = Path::new(spline1, id1);
    paths.insert(new_id(), path);

    stones.insert(new_id(), Stone::new(0.0, -128.0));
    cannons.insert(new_id(), Cannon::new(66.0, -279.0, 0.25, 256, 3.0));
    cannons.insert(new_id(), Cannon::new(114.0, -209.0, 0.25, 256, 3.0));
    cannons.insert(new_id(), Cannon::new(-65.0, -420.0, 0.5, 128, 3.0));
    (0.0, -72.0)
}