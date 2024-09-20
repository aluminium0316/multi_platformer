use std::collections::HashMap;

use macroquad::prelude::*;

use crate::{entity::Entity, hold::obj::Obj, input::Input, key, platform::{LineType, Platform}, projectiles::Projectile, vector::{dist2, norm, proj}, Scene};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Player {
    username: String,
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    dx: f64,
    dy: f64,
    dir: f64,
    ground: i32,
    linetype: LineType,
    nx: f64,
    ny: f64,
    hold: Option<u64>,
    pub cam: f64,
    escape: i32,
    pub input: Input,
}

impl Player {
    pub fn new(username: String, x: f64, y: f64) -> Self {
        Self {
            username,
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            dx: 0.0,
            dy: 0.0,
            dir: 0.0,
            ground: 0,
            linetype: LineType::Ice,
            nx: 0.0,
            ny: -1.0,
            hold: None,
            cam: 0.0,
            escape: 0,
            input: Input::new()
        }
    }

    pub fn update(&mut self, id: u64, platforms: &mut HashMap<u64, Platform>, players: &mut HashMap<u64, Player>, objs: &mut HashMap<u64, &mut dyn Obj>, projectiles: &mut HashMap<u64, &mut dyn Projectile>, scene: &mut Scene, start: &mut bool) {
        let input = &mut self.input;

        let px = self.x;
        let py = self.y;

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

        self.escape -= 1;
        // input.down[key!(K)] = 0;
        if input.down[key!(K)] < 16 && self.ground < 16 {
            input.down[key!(K)] = 16;
            self.escape = 16;
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

        // vx /= 2.0;
        if matches!(self.linetype, LineType::Normal) && self.ground < 16 {
            self.vx = (self.vx + vx * self.ny) * 63.0 / 64.0 - vx * self.ny;
            self.vy = (self.vy - vx * self.nx) * 63.0 / 64.0 + vx * self.nx;
        }
        else {
            if matches!(self.linetype, LineType::Ice) {
                vx *= 2.0;
                self.vx = (self.vx - vx) * 511.0 / 512.0 + vx;
            }
            else {
                self.vx = (self.vx - vx) * 63.0 / 64.0 + vx;
            }
        }

        if self.hold.is_some() {
            if let Some(hold) = objs.get_mut(&self.hold.unwrap()) {
                if input.down[key!(J)] < 16 || hold.escape() {
                    input.down[key!(J)] = 16;
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
                    self.hold = None;
                }
            }
        }
        if input.down[key!(J)] < 16 {
            let mut id1s = None;
            let mut id2s = None;
            let mut id3s = None;
            for (id1, player) in players.iter() {
                if player.hold == Some(id) {
                    id1s = Some(id1);
                }
            }
            for (id2, player) in players.iter() {
                if player.hold == id1s.copied() && player.hold.is_some() {
                    id2s = Some(id2);
                }
            }
            for (id3, player) in players.iter() {
                if player.hold == id2s.copied() && player.hold.is_some() {
                    id3s = Some(id3);
                }
            }
            let mut ids = vec![id];
            if let Some(id1) = id1s {
                ids.push(id1.clone());
            }
            if let Some(id1) = id2s {
                ids.push(id1.clone());
            }
            if let Some(id1) = id3s {
                ids.push(id1.clone());
            }
            for (_id, player) in players.iter() {
                if let Some(id0) = player.hold {
                    ids.push(id0)
                }
            }
            let nearest = <dyn Obj>::nearest(objs, self.x, self.y, 256.0, &ids);
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

        if self.ground > 16 {
            self.vx += self.dx;
            self.vy += self.dy;
            self.dx = 0.0;
            self.dy = 0.0;
            self.nx = self.nx * 15.0 / 16.0;
            self.ny = -1.0 / 16.0 + self.ny * 15.0 / 16.0;
        }

        self.x += self.dx;
        self.y += self.dy;

        self.linetype = LineType::Inv;
        for (_id, platform) in platforms {
            let (x, y, vx, vy, collided, nx, ny, dx, dy, linetypes) = platform.collide(self.x, self.y, 4.0, self.vx, self.vy, px, py);
            if collided {
                self.x = x;
                self.y = y;
                self.vx = vx + self.dx - dx;
                self.vy = vy + self.dy - dy;
                // self.nx = nx;
                // self.ny = ny;
                self.nx = nx / 16.0 + self.nx * 15.0 / 16.0;
                self.ny = ny / 16.0 + self.ny * 15.0 / 16.0;
                self.dx = dx;
                self.dy = dy;
                for linetype in linetypes {
                    if let LineType::Inv = linetype { } else {
                        self.ground = 0;
                    }
                    if let LineType::End = linetype {
                        *scene = Scene::End {
                            winner: format!("Winner: {}", self.username.clone()),
                        };
                    }
                    if let LineType::Normal = linetype {
                        self.linetype = LineType::Normal;
                    }
                    if let LineType::Ice = linetype {
                        self.linetype = LineType::Ice;
                    }
                }
            }
        }

        let r = self.nx * self.nx + self.ny * self.ny;
        self.nx /= r;
        self.ny /= r;

        // self.rn = (self.nx / 16.0 + self.rn.0 * 15.0 / 16.0, self.ny / 16.0 + self.rn.1 * 15.0 / 16.0);

        // if self.ground < 16 {
        //     self.cam = self.cam * 15.0 / 16.0 + (self.y - 64.0) / 16.0;
        // }
        // else if self.vy > -0.5 {
        //     self.cam = self.cam * 15.0 / 16.0 + (self.y - 32.0) / 16.0;
        // }
        // self.cam = self.cam * 15.0 / 16.0  + (self.y + self.vy.abs().sqrt() * 48.0 * self.vy.signum() - 16.0) / 16.0;
        self.cam = (
            self.cam * 15.0 / 16.0  + (self.y - 32.0) / 16.0 + self.vy
        ).clamp(-416.0, -96.0);
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
        draw_text(&self.username, self.x as f32 - 4.0, self.y as f32 - 12.0, 8.0, BLUE);
        if self.hold.is_some() {
            draw_texture_ex(&assets[0], self.x as f32 - 8.0, self.y as f32 - 8.0, WHITE, DrawTextureParams {
                source: Some(Rect { x: 0.0, y: 0.0, w: 16.0, h: 16.0 }),
                ..Default::default()
            });
        }
        else {
            draw_texture_ex(&assets[0], self.x as f32 - 8.0, self.y as f32 - 8.0, WHITE, DrawTextureParams {
                source: Some(Rect { x: 0.0, y: 16.0, w: 16.0, h: 16.0 }),
                ..Default::default()
            });
        }
        draw_texture_ex(&assets[0], self.x as f32 - 8.0, self.y as f32 - 8.0, WHITE, DrawTextureParams {
            source: Some(Rect { x: 16.0, y: 0.0, w: 16.0, h: 16.0 }),
            rotation: self.ny.atan2(self.nx) as f32,
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
        self.x = x;
        self.y = y - 4.0;
        self.vx = 0.0;
        self.vy = 0.0;
        self.ground = 0;
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
    fn escape(&mut self) -> bool {
        self.escape > 0
    }
}

impl Projectile for Player {
    fn collision(&mut self, player: &mut crate::player::Player) {
        if std::ptr::eq(player.as_proj().unwrap(), self) {
            return;
        }
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

impl Entity<'_> for Player {
    fn as_obj(&mut self) -> Option<&mut dyn Obj> {
        Some(self)
    }
    fn as_proj(&mut self) -> Option<&mut dyn Projectile> {
        Some(self)
    }
}