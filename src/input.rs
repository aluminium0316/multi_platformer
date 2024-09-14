use macroquad::prelude::*;
use serde_big_array::BigArray;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Input {
    #[serde(with = "BigArray")]
    pub down: [i64; 512],
    #[serde(with = "BigArray")]
    pub up: [i64; 512],
    #[serde(with = "BigArray")]
    pub key: [bool; 512],
    // pub x: f32,
    // pub y: f32,
    // pub dx: f32,
    // pub dy: f32,
}

#[macro_export]
macro_rules! key {
    ($key:tt) => {{
        KeyCode::$key as usize & 0x1ff
    }};
}

impl Input {
    pub fn new() -> Self {
        Self {
            down: [256; 512],
            up: [256; 512], 
            key: [false; 512],
            // x: 0.0,
            // y: 0.0,
            // dx: 0.0,
            // dy: 0.0,
        }
    }

    pub fn input(&mut self) {
        for keys in get_keys_down() {
            self.key[keys as usize & 0x1ff] = true;
        }
        for keys in get_keys_released() {
            self.up[keys as usize & 0x1ff] = 0;
            self.key[keys as usize & 0x1ff] = false;
        }
        for keys in get_keys_pressed() {
            self.down[keys as usize & 0x1ff] = 0;
            self.key[keys as usize & 0x1ff] = true;
        }
        // Vec2 { x: self.x, y: self.y } = mouse_position_local();
        // Vec2 { x: self.dx, y: self.dy } = mouse_delta_position();

        mouse_key(MouseButton::Left, 0, self);
        mouse_key(MouseButton::Middle, 1, self);
        mouse_key(MouseButton::Right, 2, self);
        mouse_key(MouseButton::Unknown, 3, self);

        // self.x *= screen_width() / screen_height();
    }

    pub fn update(&mut self) {
        for i in 0..512 {
            self.up[i] += 1;
            self.down[i] += 1;
        }
    }

    pub fn set(&mut self, other: &Input) {
        for i in 0..512 {
            if other.down[i] == 0 {
                self.down[i] = 0;
            }
            if other.up[i] == 0 {
                self.up[i] = 0;
            }
            self.key[i] = other.key[i];
        }
    }
}

fn mouse_key(button: MouseButton, i: usize, input: &mut Input) {
    if is_mouse_button_pressed(button) {
        input.down[i] = 0;
        input.key[i] = true;
    }
    if is_mouse_button_down(button) {
        input.key[i] = true;
    }
    if is_mouse_button_released(button) {
        input.up[i] = 0;
        input.key[i] = false;
    }
}