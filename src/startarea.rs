use std::collections::HashMap;

use macroquad::{color::{Color, BLUE}, shapes::draw_rectangle, text::draw_text};

use crate::player::Player;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Startarea {
    x: f64,
    y: f64,
    w: f64, 
    h: f64,
    st: bool,
}

impl Startarea {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self {
            x, y, w, h, st: true,
        }
    }
    pub fn update(&mut self, start: &bool, players: &mut HashMap<u64, Player>) {
        if !start { 
            self.st = false;
            return; 
        }
        for (_id, player) in players {
            let (x, y) = player.pos();
            if x < self.x {
                player.collide(self.x - x, 0.0);
            }
            if x > self.x + self.w {
                player.collide(self.x + self.w - x, 0.0);
            }
            if y < self.y {
                player.collide(0.0, self.y - y);
            }
            if y > self.y + self.h {
                player.collide(0.0, self.y + self.h - y);
            }
        }
    }
    pub fn render(&self) {
        if self.st {
            draw_rectangle(self.x as f32 - 4.0, self.y as f32 - 4.0, self.w as f32 + 8.0, self.h as f32 + 8.0, Color::new(0.0, 1.0, 0.0, 0.125));
        }
    }
    pub fn ui(&self, start: &bool) {
        if self.st && *start {
            draw_text("press ] to start", -120.0, -48.0, 16.0, BLUE);
        }
    }
}