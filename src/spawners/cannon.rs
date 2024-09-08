use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{entity::Entity, new_id, player::Player, projectiles::damage::Damage, vector::dist2};

pub struct Cannon {
    x: f64,
    y: f64,
    progress: i32,
}

impl Cannon {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            progress: 0,
        }
    }

    pub fn update(&mut self, players: &mut HashMap<u64, Player>, damages: &mut HashMap<u64, Damage>) {
        self.progress -= 1;
        if self.progress < 0 {
            self.progress = 64;
            let mut dist = 65536.0;
            let mut x = 0.0;
            let mut y = 0.0;
            for (_id, player) in players.iter_mut() {
                let (x1, y1) = player.pos();

                let d = dist2(x1, y1, self.x, self.y);
                if d < dist {
                    dist = d;
                    x = x1;
                    y = y1;
                }
            }

            if dist < 65535.0 {
                let vx = x - self.x;
                let vy = y - self.y;

                let r = (vx * vx + vy * vy).sqrt() * 2.0;

                damages.insert(new_id(), Damage::new(self.x, self.y, vx / r, vy / r));
            }
        }
    }

    pub fn render(&self, assets: &Vec<Texture2D>) {
        draw_texture_ex(&assets[2], self.x as f32 - 4.0, self.y as f32 - 4.0, WHITE, DrawTextureParams {
            source: Some(Rect { x: 0.0, y: 0.0, w: 8.0, h: 8.0 }),
            dest_size: Some(vec2(8.0, 8.0)),
            ..Default::default()
        });
    }
}

impl Entity<'_> for Cannon {}