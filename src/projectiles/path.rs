use std::collections::HashMap;

use macroquad::prelude::*;

use crate::hold::obj::Obj;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum Curve {
    Linear {
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
    },
    Quadratic {
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    },
    Cubic {
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        x3: f64,
        y3: f64,
    },
}

impl Curve {
    pub fn new_l(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Self::Linear { x0, y0, x1, y1 }
    }
    pub fn new_q(x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self::Quadratic { x0, y0, x1, y1, x2, y2 }
    }
    pub fn new_c(x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) -> Self {
        Self::Cubic { x0, y0, x1, y1, x2, y2, x3, y3 }
    }
    pub fn b(&self, t: f64) -> (f64, f64) {
        let t1 = 1.0 - t;
        match self {
            Curve::Linear { x0, y0, x1, y1 } => {
                let x = t1 * x0 + t * x1;
                let y = t1 * y0 + t * y1;
                (x, y)
            },
            Curve::Quadratic { x0, y0, x1, y1, x2, y2 } => {
                let x = t1 * t1 * x0 + 2.0 * t1 * t * x1 + t * t * x2;
                let y = t1 * t1 * y0 + 2.0 * t1 * t * y1 + t * t * y2;
                (x, y)
            },
            Curve::Cubic { x0, y0, x1, y1, x2, y2, x3, y3 } => {
                let x = t1 * t1 * t1 * x0 + 3.0 * t1 * t1 * t * x1 + 3.0 * t1 * t * t * x2 + t * t * t * x3;
                let y = t1 * t1 * t1 * y0 + 3.0 * t1 * t1 * t * y1 + 3.0 * t1 * t * t * y2 + t * t * t * y3;
                (x, y)
            },
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Spline {
    curves: Vec<Curve>,
    speed: Vec<f64>,
    pos: Vec<f64>,
}

impl Spline {
    pub fn new() -> Self {
        Self {
            curves: vec![],
            speed: vec![],
            pos: vec![],
        }
    }
    pub fn add_curve(&mut self, curve: Curve, speed: f64) {
        self.curves.push(curve);
        self.speed.push(speed);
    }
    pub fn position(&mut self) {
        let mut prev = 0.0;
        let mut j = 0;
        self.pos = vec![0.0];
        for i in self.speed.iter() {
            self.pos.push(prev + i);
            prev = self.pos.last().unwrap().clone();
            j += 1;
        }
    }
    pub fn b(&self, mut t: f64) -> (f64, f64) {
        let i = self.pos.partition_point(|&x| x <= t) - 1;
        t -= self.pos[i];
        if i == self.pos.len() - 1 {
            self.curves[i-1].b(1.0)
        }
        else {
            self.curves[i].b(t / self.speed[i])
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Path {
    spline: Spline,
    id: u64,
    t: f64,
}

impl Path {
    pub fn new(spline: Spline, id: u64) -> Self {
        Self {
            spline,
            id,
            t: 0.0,
        }
    }
    pub fn update(&mut self, objs: &mut HashMap<u64, &mut dyn Obj>) {
        if let Some(obj) = objs.get_mut(&self.id) {
            let (x, y) = self.spline.b(self.t);
            obj.hold_location(x, y);
        }
        self.t += 1.0;
        let last = self.spline.pos.last().unwrap().clone();
        if self.t > last {
            self.t = 0.0;
        }
    }
    pub fn render(&self) {
        let last = self.spline.pos.last().unwrap().clone();
        for i in 0..32 {
            let t = i as f64 * last / 32.0;
            let (x, y) = self.spline.b(t);
            draw_rectangle(x as f32, y as f32, 1.0, 1.0, MAGENTA);
        }
    }
}