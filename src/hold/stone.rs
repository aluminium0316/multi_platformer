use std::collections::HashMap;

use crate::{entity::Entity, platform::Platform, projectiles::Projectile, vector::{dist2, norm}};

use super::obj::Obj;
use macroquad::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Stone {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    ground: i32,
}

impl Stone {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y, 
            vx: 0.0, 
            vy: 0.0,
            ground: 0,
        }
    }
    pub fn update(&mut self, platforms: &mut HashMap<u64, Platform>) {
        self.vy += 1.0 / 256.0;

        if self.ground < 16 {
            self.vx *= 15.0 / 16.0;
            self.vy *= 15.0 / 16.0;
            self.ground = 16;
        }

        self.x += self.vx;
        self.y += self.vy;

        self.ground += 1;

        for (_id, platform) in platforms {
            let (x, y, vx, vy, collided, _nx, _ny, dx, dy, _) = platform.collide(self.x, self.y, 4.0, self.vx, self.vy, self.x - self.vx, self.y - self.vy);
            if collided {
                self.x = x + dx;
                self.y = y + dy;
                self.vx = vx * 1.5 - self.vx * 0.5;
                self.vy = vy * 1.5 - self.vy * 0.5;
                self.ground = 0;
            }
        }

    }
    pub fn render(&self, assets: &Vec<Texture2D>) {
        draw_texture_ex(&assets[1], self.x as f32 - 4.0, self.y as f32 - 4.0, WHITE, DrawTextureParams {
            source: Some(Rect { x: 0.0, y: 0.0, w: 8.0, h: 8.0 }),
            dest_size: Some(vec2(8.0, 8.0)),
            ..Default::default()
        });
    }
}

impl Obj for Stone {
    fn hold_location(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y - 4.0;
        self.vx = 0.0;
        self.vy = 0.0;
    }
    fn throw(&mut self, vx: f64, vy: f64) {
        self.vx = vx;
        self.vy = vy;
    }
    fn grab(&self, x: f64, y: f64) -> f64 {
        dist2(self.x, self.y, x, y)
    }
    fn hold(&mut self) -> (f64, f64) {
        (self.vx, self.vy)
    }
}

impl Entity<'_> for Stone {
    fn as_obj(&mut self) -> Option<&mut dyn Obj> {
        Some(self)
    }
    fn as_proj(&mut self) -> Option<&mut dyn Projectile> {
        Some(self)
    }
}

impl Projectile for Stone {
    fn collision(&mut self, player: &mut crate::player::Player) {
        let (x, y) = player.pos();
        let d = dist2(x, y, self.x, self.y);
        let (vx, vy) = norm(x - self.x, y - self.y, 0.25);
        if d < 16.0 {
            // player.throw(vx, vy);
            player.collide(vx, vy);
            // self.vx -= vx;
            // self.vy -= vy;
        }
    }
}