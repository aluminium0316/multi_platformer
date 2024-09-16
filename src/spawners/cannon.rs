use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{entity::Entity, hold::obj::Obj, new_id, player::Player, projectiles::damage::Damage, vector::dist2};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Cannon {
    x: f64,
    y: f64,
    px: f64,
    py: f64,
    sp: f64,
    freq: i32, 
    progress: i32,
    force: f64,
}

impl Cannon {
    pub fn new(x: f64, y: f64, sp: f64, freq: i32, force: f64,) -> Self {
        Self {
            x,
            y,
            px: x,
            py: y,
            sp, freq, force,
            progress: 0,
        }
    }

    pub fn update(&mut self, players: &mut HashMap<u64, Player>, damages: &mut HashMap<u64, Damage>) {
        self.progress -= 1;
        if self.progress < 0 {
            self.progress = self.freq;
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

                let r = (vx * vx + vy * vy).sqrt() / self.sp;

                damages.insert(new_id(), Damage::new(self.x, self.y, vx / r, vy / r, self.force));
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

impl Obj for Cannon {
    fn hold_location(&mut self, x: f64, y: f64) {
        self.px = self.x;
        self.py = self.y;
        self.x = x;
        self.y = y - 4.0;
    }
    fn throw(&mut self, vx: f64, vy: f64) {
        
    }
    fn grab(&self, x: f64, y: f64) -> f64 {
        std::f64::INFINITY
    }
    fn hold(&mut self) -> (f64, f64) {
        (0.0, 0.0)
    }
}

impl Entity<'_> for Cannon {
    fn as_obj(&mut self) -> Option<&mut dyn Obj> {
        Some(self)
    }
}
