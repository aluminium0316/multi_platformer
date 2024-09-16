use std::time::Instant;

use::macroquad::prelude::*;

use crate::{input::Input, Scene};

pub struct UI;

impl UI {
    pub fn ui(scene: &mut Scene, t: &mut i32, client: &bool) {
        draw_text(&format!("Time: {:.2}", t.clone() as f64 / 240.0), -120.0, -80.0, 16.0, BLACK);
        draw_text(if *client { "s=0" } else { "s=1" }, 100.0, -80.0, 16.0, BLACK);
        let mut newscene = scene.clone();
        match scene {
            Scene::End{ winner } => {
                draw_text(&winner, -120.0, -64.0, 16.0, BLACK);
                draw_text("Press R to Restart", -120.0, -48.0, 16.0, BLUE);
                if is_key_pressed(KeyCode::R) {
                    *t = 0;
                    newscene = Scene::Restart { level: 1 };
                }
            },
            Scene::Start => {
                draw_text(if *client { "Server Name: 127.0.0.1:3400 " } else { "Server start" }, -120.0, -64.0, 16.0, BLACK);
                draw_text("Press Enter to start", -120.0, -48.0, 16.0, BLUE);
                if is_key_pressed(KeyCode::Enter) {
                    *t = 0;
                    newscene = Scene::Restart { level: 1 };
                }
            },
            _ => {}
        }
        *scene = newscene;
    }
}