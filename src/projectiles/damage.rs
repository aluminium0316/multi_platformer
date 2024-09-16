use macroquad::prelude::*;

use crate::{entity::Entity, vector::dist2};

use super::Projectile;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Damage {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    force: f64,
    t: i32,
}

impl Damage {
    pub fn new(x: f64, y: f64, vx: f64, vy: f64, force: f64) -> Self {
        Self { x, y, vx, vy, force, t: 4096 }
    }

    pub fn update(&mut self) -> bool {
        self.x += self.vx;
        self.y += self.vy;

        self.t -= 1;
        if self.t < 0 {
            return true;
        }
        false
    }

    pub fn render(&self, assets: &Vec<Texture2D>) {
        draw_texture_ex(&assets[3], self.x as f32 - 4.0, self.y as f32 - 4.0, WHITE, DrawTextureParams {
            source: Some(Rect { x: 0.0, y: 0.0, w: 16.0, h: 16.0 }),
            dest_size: Some(vec2(8.0, 8.0)),
            ..Default::default()
        });
    }
}

impl Entity<'_> for Damage {
    fn as_proj(&mut self) -> Option<&mut dyn super::Projectile> {
        Some(self)
    }
}

impl Projectile for Damage {
    fn collision(&mut self, player: &mut crate::player::Player) {
        let (x, y) = player.pos();
        let d = dist2(x, y, self.x, self.y);
        if d < 16.0 {
            player.throw(self.vx * self.force, self.vy * self.force / 2.0);
            self.t = 0;
        }
    }
}