use macroquad::prelude::*;
use rand::rand;

use crate::vector::{dist2, dot, proj};

struct Line {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

impl Line {
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self { x1, y1, x2, y2 }
    }
}

pub struct Platform {
    x: f64,
    y: f64,
    lines: Vec<Line>,
}

impl Platform {
    pub fn new() -> Self {
        let mut lines = Vec::new();

        for i in 0..16 {
            lines.push(Line::new(rand() as f64 / 8388608.0 - 256.0, rand() as f64 / 8388608.0 - 256.0, rand() as f64 / 8388608.0 - 256.0, rand() as f64 / 8388608.0 - 256.0));
            if lines[i].x2 < lines[i].x1 {
                (lines[i].x1, lines[i].x2) = (lines[i].x2, lines[i].x1);
            }
        }

        Self {
            x: 0.0,
            y: 0.0,
            lines,
        }
    }
    pub fn render(&self) {
        for line in self.lines.iter() {
            draw_line(line.x1 as f32, line.y1 as f32, line.x2 as f32, line.y2 as f32, 1.0, RED);
        }
    }

    pub fn collide(&self, mut x: f64, mut y: f64, r: f64, mut vx: f64, mut vy: f64) -> (f64, f64, f64, f64, bool, f64, f64) {
        let mut collision = false;
        let mut nx1 = 0.0;
        let mut ny1 = 0.0;
        for line in self.lines.iter() {
            let mut nx = line.y2 - line.y1;
            let mut ny = line.x1 - line.x2;

            let r1 = (nx * nx + ny * ny).sqrt();

            nx = nx * r / r1;
            ny = ny * r / r1;

            let (x1, y1) = proj(x - line.x1, y - line.y1, line.x2 - line.x1, line.y2 - line.y1);

            if ny < x1 && x1 < line.x2 - line.x1 - ny || line.x2 - line.x1 - ny < x1 && x1 < ny || -nx < y1 && y1 < line.y2 - line.y1 + nx || line.y2 - line.y1 + nx < y1 && y1 < -nx {
                let d = dist2(x1, y1, x - line.x1, y - line.y1);
                if d < r*r && dot(vx, vy, nx, ny) < 0.0 {
                    x = x1 + line.x1 + nx;
                    y = y1 + line.y1 + ny;

                    nx1 += nx / r;
                    ny1 += ny / r;

                    (vx, vy) = proj(vx, vy, line.x2 - line.x1, line.y2 - line.y1);
                    collision = true;
                }
            }
        }

        let r = (nx1 * nx1 + ny1 * ny1).sqrt();

        (x, y, vx, vy, collision, nx1 / r, ny1 / r)
    }
}