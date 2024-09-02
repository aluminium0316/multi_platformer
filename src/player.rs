use macroquad::prelude::*;

use crate::{hold::obj::Obj, input::Input, key, platform::Platform};

pub struct Player<'a> {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    ground: i32,
    nx: f64,
    ny: f64,
    hold: Option<&'a mut dyn Obj>,
    pub id: u64,
}

impl<'a> Player<'a> {
    pub fn new(id: u64) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            ground: 0,
            nx: 0.0,
            ny: -1.0,
            hold: None,
            id,
        }
    }

    pub fn update(&mut self, input: &mut Input, platforms: &mut Vec<Platform>) {

        self.vy += 1.0 / 128.0;
        if self.vy > 8.0 {
            self.vy = 8.0;
        }

        let mut vx = 0.0;
        if input.key[key!(D)] {
            vx += 1.0;
        }
        if input.key[key!(A)] {
            vx -= 1.0;
        }

        if input.down[key!(K)] < 16 && self.ground < 16 {
            input.down[key!(K)] = 16;
            self.ground = 16;
            let mut nx = self.nx;
            let mut ny = self.ny;

            let _12 = 0.7071067811865476;

            if input.key[key!(S)] {
                if !input.key[key!(A)] && !input.key[key!(D)] {
                    ny = self.ny * _12;
                }
                if input.key[key!(A)] {
                    nx = self.nx * _12 + self.ny * _12;
                    ny = self.ny * _12 - self.nx * _12;
                }
                if input.key[key!(D)] {
                    nx = self.nx * _12 - self.ny * _12;
                    ny = self.ny * _12 + self.nx * _12;
                }
                if input.key[key!(A)] && input.key[key!(D)] {
                    nx = self.nx;
                    ny = self.ny * _12;
                }
            }

            self.vx += 1.0 * nx;
            self.vy += 1.0 * ny;
        }

        if input.down[key!(J)] < 16 {
            input.down[key!(J)] = 16;
            if self.hold.is_some() {
                self.hold = None;
            }
        }

        // if input.down[key!(L)] < 16 {
        //     input.down[key!(L)] = 16;

        //     let mut vx = 0.0_f64;
        //     let mut vy = 0.0;

        //     if input.key[key!(W)] {
        //         vy -= 1.0;
        //     }
        //     if input.key[key!(A)] {
        //         vx -= 1.0;
        //     }
        //     if input.key[key!(S)] {
        //         vy += 1.0;
        //     }
        //     if input.key[key!(D)] {
        //         vx += 1.0;
        //     }

        //     let r = (vx * vx + vy * vy).sqrt();
        //     if r < 0.25 {
        //         vx = 1.0;
        //     }
        //     else {
        //         vx /= r;
        //         vy /= r;
        //     }

        //     self.vx = vx * 4.0;
        //     self.vy = vy * 2.0;
        // }

        if self.ground < 16 {
            self.vx = (self.vx + vx * self.ny) * 63.0 / 64.0 - vx * self.ny;
            self.vy = (self.vy - vx * self.nx) * 63.0 / 64.0 + vx * self.nx;
        }
        else {
            self.vx = (self.vx - vx) * 63.0 / 64.0 + vx;
        }

        self.x += self.vx;
        self.y += self.vy;
        self.ground += 1;

        for platform in platforms {
            let (x, y, vx, vy, collided, nx, ny) = platform.collide(self.x, self.y, 8.0, self.vx, self.vy);
            if collided {
                self.x = x;
                self.y = y;
                self.vx = vx;
                self.vy = vy;
                self.nx = nx;
                self.ny = ny;
                self.ground = 0;
            }
        }

        if let Some(hold) = &mut self.hold {
            hold.hold_location(self.x, self.y - 8.0);
        }
    }

    pub fn render(&self, assets: &Vec<Texture2D>) {
        draw_rectangle(self.x as f32 - 8.0, self.y as f32 - 8.0, 16.0, 16.0, BLUE)
    }

    pub fn camera(&self, target: &RenderTarget) {
        set_camera(&Camera2D {
            target: vec2(self.x.clamp(-256.0, 256.0) as f32, self.y as f32),
            render_target: Some(target.clone()),
            zoom: vec2( 1.0 / 256.0 , 1.0 / 192.0),
            ..Default::default()
        })
    }

    pub fn hold(&mut self, other: &'a mut dyn Obj) where{
        self.hold = Some(other);
    }
}

impl Obj for Player<'_> {
    fn hold_location(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y - 8.0;
        self.vx = 0.0;
        self.vy = 0.0;
    }
}