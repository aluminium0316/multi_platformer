use macroquad::prelude::*;

pub struct Input {
    pub down: [i64; 512],
    pub up: [i64; 512],
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

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut v = vec![];
        v.extend(self.down.iter().flat_map(|x| x.to_be_bytes()));
        v.extend(self.up.iter().flat_map(|x| x.to_be_bytes()));
        v.extend(self.key.iter().flat_map(|x| [*x as u8]));
        v
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut s = Self::new();
        let mut i = 0;
        for byte8 in bytes.chunks_exact(8) {
            if i < 512 {
                s.down[i] = i64::from_be_bytes(byte8.try_into().unwrap());
            }
            else if i < 1024 {
                s.up[i - 512] = i64::from_be_bytes(byte8.try_into().unwrap());
            }
            else {
                s.key[(i - 1024) * 8 + 0] = byte8[0] != 0;
                s.key[(i - 1024) * 8 + 1] = byte8[1] != 0;
                s.key[(i - 1024) * 8 + 2] = byte8[2] != 0;
                s.key[(i - 1024) * 8 + 3] = byte8[3] != 0;
                s.key[(i - 1024) * 8 + 4] = byte8[4] != 0;
                s.key[(i - 1024) * 8 + 5] = byte8[5] != 0;
                s.key[(i - 1024) * 8 + 6] = byte8[6] != 0;
                s.key[(i - 1024) * 8 + 7] = byte8[7] != 0;
            }
            i += 1;
        }
        s
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