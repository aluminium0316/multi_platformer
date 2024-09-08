use crate::player::Player;

pub mod damage;

pub trait Projectile {
    fn collision(&mut self, player: &mut Player);
}