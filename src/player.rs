use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{entity::Entity, hold::obj::Obj, input::Input, key, platform::{LineType, Platform}, projectiles::Projectile, vector::{dist2, proj}, Scene};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Player {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    dir: f64,
    ground: i32,
    linetype: LineType,
    nx: f64,
    ny: f64,
    hold: Option<u64>,
    cam: f64,
    pub input: Input,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            dir: 0.0,
            ground: 0,
            linetype: LineType::Ice,
            nx: 0.0,
            ny: -1.0,
            hold: None,
            cam: 0.0,
            input: Input::new()
        }
    }

    pub fn update(&mut self, id: u64, platforms: &mut HashMap<u64, Platform>, objs: &mut HashMap<u64, &mut dyn Obj>, projectiles: &mut HashMap<u64, &mut dyn Projectile>, scene: &mut Scene, start: &mut bool) {
        let input = &mut self.input;
        self.vy += 1.0 / 256.0;
        if self.vy > 8.0 {
            self.vy = 8.0;
        }

        let mut vx = 0.0;
        if input.key[key!(D)] {
            vx += 0.5;
        }
        if input.key[key!(A)] {
            vx -= 0.5;
        }
        if vx != 0.0 {
            self.dir = vx;
        }

        if input.down[key!(K)] < 16 && self.ground < 16 {
            input.down[key!(K)] = 16;
            self.ground = 16;
            let mut nx = self.nx;
            let mut ny = self.ny;

            let _12 = 0.7071067811865476;

            if input.key[key!(S)] {
                if !input.key[key!(A)] && !input.key[key!(D)] {
                    nx = self.nx * _12;
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
                    nx = self.nx * _12;
                    ny = self.ny * _12;
                }
            }

            self.vx += 0.5 * nx;
            self.vy += 0.5 * ny;
        }

        if matches!(self.linetype, LineType::Normal) && self.ground < 16 {
            self.vx = (self.vx + vx * self.ny) * 63.0 / 64.0 - vx * self.ny;
            self.vy = (self.vy - vx * self.nx) * 63.0 / 64.0 + vx * self.nx;
        }
        else {
            self.vx = (self.vx - vx) * 63.0 / 64.0 + vx;
        }

        if input.down[key!(J)] < 16 && self.hold.is_some() {
            input.down[key!(J)] = 16;
            if let Some(id_) = &mut self.hold {
                if let Some(hold) = objs.get_mut(id_) {
                    if input.key[key!(S)] {
                        hold.throw(self.vx / 2.0, self.vy / 2.0 + 0.5);
                        self.vx /= 2.0;
                        self.vy /= 2.0;
                        self.vy -= 0.5;
                    }
                    else if input.key[key!(W)] {
                        hold.throw(self.vx / 2.0, self.vy / 2.0 - 0.5);
                        self.vx /= 2.0;
                        self.vy /= 2.0;
                        self.vy += 0.5;
                    }
                    else {
                        hold.throw(self.vx / 2.0 + self.dir, self.vy / 2.0);
                        self.vx /= 2.0;
                        self.vy /= 2.0;
                        self.vx -= self.dir;
                    }
                }
            }
            self.hold = None;
        }
        if input.down[key!(J)] < 16 {
            let nearest = <dyn Obj>::nearest(objs, self.x, self.y, 256.0, id);
            self.hold = nearest;
            if let Some(id) = nearest {
                input.down[key!(J)] = 16;
                let near = objs.get_mut(&id).unwrap();
                let (vx, vy) = near.hold();
                self.vx = (self.vx + vx) / 2.0;
                self.vy = (self.vy + vy) / 2.0;
            }
        }

        self.x += self.vx;
        self.y += self.vy;
        self.ground += 1;

        self.linetype = LineType::Inv;
        for (_id, platform) in platforms {
            let (x, y, vx, vy, collided, nx, ny, linetypes) = platform.collide(self.x, self.y, 4.0, self.vx, self.vy);
            if collided {
                self.x = x;
                self.y = y;
                self.vx = vx;
                self.vy = vy;
                self.nx = nx;
                self.ny = ny;
                for linetype in linetypes {
                    if let LineType::Inv = linetype { } else {
                        self.ground = 0;
                    }
                    if let LineType::End = linetype {
                        *scene = Scene::End {
                            winner: format!("Winner: {}", id)
                        };
                    }
                    if let LineType::Normal = linetype {
                        self.linetype = LineType::Normal;
                    }
                }
            }
        }

        // if self.ground < 16 {
        //     self.cam = self.cam * 15.0 / 16.0 + (self.y - 64.0) / 16.0;
        // }
        // else if self.vy > -0.5 {
        //     self.cam = self.cam * 15.0 / 16.0 + (self.y - 32.0) / 16.0;
        // }
        // self.cam = self.cam * 15.0 / 16.0  + (self.y + self.vy.abs().sqrt() * 48.0 * self.vy.signum() - 16.0) / 16.0;
        self.cam = self.cam * 15.0 / 16.0  + (self.y - 32.0) / 16.0 + self.vy;
        if let Some(id_) = &mut self.hold {
            if let Some(hold) = objs.get_mut(id_) {
                hold.hold_location(self.x, self.y - 4.0);
            }
        }

        if input.down[key!(RightBracket)] == 0 {
            *start = false;
        }

        for (_id, projectile) in projectiles.iter_mut() {
            projectile.collision(self);
        }

        self.input.update();
    }

    pub fn render(&self, assets: &Vec<Texture2D>) {
        // draw_rectangle(self.x as f32 - 8.0, self.y as f32 - 8.0, 16.0, 16.0, BLUE)
        draw_texture_ex(&assets[0], self.x as f32 - 4.0, self.y as f32 - 4.0, WHITE, DrawTextureParams {
            source: Some(Rect { x: 0.0, y: 0.0, w: 8.0, h: 8.0 }),
            dest_size: Some(vec2(8.0, 8.0)),
            // rotation: self.ny.atan2(self.nx) as f32,
            ..Default::default()
        });
    }

    pub fn camera(&self, target: &RenderTarget) {
        set_camera(&Camera2D {
            target: vec2(0.0, self.cam as f32),
            render_target: Some(target.clone()),
            zoom: vec2( 1.0 / 128.0 , 1.0 / 96.0),
            ..Default::default()
        })
    }

    // pub fn hold(&mut self, other: u64) {
    //     self.hold = Some(other);
    // }

    pub fn pos(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn throw(&mut self, vx: f64, vy: f64) {
        self.vx += vx;
        self.vy += vy;
    }
    pub fn collide(&mut self, vx: f64, vy: f64) {
        self.x += vx;
        self.y += vy;
        (self.vx, self.vy) = proj(self.vx, self.vy, -vy, vx);
    }
}

impl Obj for Player {
    fn hold_location(&mut self, x: f64, y: f64) {
        self.hold = None;
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

impl Entity<'_> for Player {
    fn as_obj(&mut self) -> Option<&mut dyn Obj> {
        Some(self)
    }
}