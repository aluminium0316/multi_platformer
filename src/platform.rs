use macroquad::prelude::*;
use rand::rand;

use crate::{vector::{dist2, dot, proj}, Scene};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum LineType {
    Normal,
    End, 
    Ice,
    Inv,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct Line {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    linetype: LineType,
}

impl Line {
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64, linetype: LineType) -> Self {
        Self { x1, y1, x2, y2, linetype }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Platform {
    x: f64,
    y: f64,
    lines: Vec<Line>,
}

impl Platform {
    pub fn new() -> Self {
        let lines = Vec::new();

        Self {
            x: 0.0,
            y: 0.0,
            lines,
        }
    }
    pub fn render(&self) {
        for line in self.lines.iter() {
            draw_line(line.x1 as f32, line.y1 as f32, line.x2 as f32, line.y2 as f32, 1.0, match line.linetype {
                LineType::Normal => RED,
                LineType::End => GREEN,
                LineType::Ice => SKYBLUE,
                LineType::Inv => BLANK,
            });
        }
    }

    pub fn new_ln(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, line: LineType) {
        self.lines.push(Line::new(x1, y1, x2, y2, line));
    }
    pub fn new_poly(&mut self, mut lines: Vec<(f64, f64)>, linetype: LineType) {
        lines.push(lines[0]);
        let mut prev: Option<(f64, f64)> = None;
        for line in lines.iter() {
            if let Some(pline) = prev {
                self.lines.push(Line::new(pline.0, pline.1, line.0, line.1, linetype.clone()));
            }
            prev = Some(line.clone())
        }
    }

    pub fn collide(&self, mut x: f64, mut y: f64, r: f64, mut vx: f64, mut vy: f64) -> (f64, f64, f64, f64, bool, f64, f64, Vec<LineType>) {
        let mut collision = false;
        let mut linetypes = Vec::new();
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
                    linetypes.push(line.linetype.clone());
                }
            }
        }

        let r = (nx1 * nx1 + ny1 * ny1).sqrt();

        (x, y, vx, vy, collision, nx1 / r, ny1 / r, linetypes)
    }
}