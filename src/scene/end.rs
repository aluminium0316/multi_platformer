use std::time::Instant;

use::macroquad::prelude::*;

use crate::{input::Input, Scene};

pub struct UI;

impl UI {
    pub fn ui(scene: &mut Scene, t: &mut i32) {
        draw_text(&format!("Time: {:.2}", t.clone() as f64 / 240.0), -120.0, -80.0, 16.0, BLACK);
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
                draw_text("Server Name: ", -120.0, -64.0, 16.0, BLACK);
                draw_text("Press Enter to Restart", -120.0, -48.0, 16.0, BLUE);
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