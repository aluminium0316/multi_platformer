mod input;
mod player;
mod platform;
mod vector;
mod projectiles;
mod hold;
mod entity;
mod spawners;

use std::{collections::HashMap, sync::atomic::AtomicU64};

use entity::Entities;
use hold::stone::Stone;
use input::Input;
use macroquad::prelude::*;
use platform::Platform;
use player::Player;
use projectiles::damage;
use spawners::cannon::{self, Cannon};

fn window_conf() -> Conf {
    Conf {
        window_title: "43".to_owned(),
        window_width: 768,
        window_height: 576,
        ..Default::default()
    }
}

static MAX_ID: AtomicU64 = AtomicU64::new(0);
pub fn new_id() -> u64 {
    // println!("{:?}", MAX_ID);
    MAX_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut input = Input::new();
    let mut players = HashMap::new();
    let mut platforms = HashMap::new();
    let mut stones = HashMap::new();
    let mut damages = HashMap::new();
    let mut cannons = HashMap::new();
    players.insert(new_id(), Player::new());
    platforms.insert(new_id(), Platform::new());
    stones.insert(new_id(), Stone::new(0.0, 0.0));
    cannons.insert(new_id(), Cannon::new(0.0, -128.0));

    let mut entities: Entities = Entities(HashMap::new());

    let target = render_target(256, 192);
    target.texture.set_filter(FilterMode::Nearest);

    // client.as_ref();
    let t = std::time::Instant::now();
    let mut prev_ns = 0;
    let mut dt = 0;
    let fps = 240;
    let mut ticks = 0;
    let assets = vec![
        load_texture("assets/player.png").await.unwrap(),
        load_texture("assets/stone.png").await.unwrap(),
        load_texture("assets/cannon.png").await.unwrap(),
        load_texture("assets/orb.png").await.unwrap(),
    ];

    for asset in &assets {
        asset.set_filter(FilterMode::Nearest);
    }
    
    let mut fullscreen = true;

    loop {
        input.input();

        if input.down[key!(F11)] == 0 {
            fullscreen ^= true;
            set_fullscreen(fullscreen);
        }

        let ns = t.elapsed().as_nanos();
        dt += ns - prev_ns;
        prev_ns = ns;
        let mut i = 0;

        while dt > 1000000000/fps {
            dt -= 1000000000/fps;

            entities.0.clear();
            entities.append(&mut stones);
            entities.append(&mut players);
            entities.append(&mut cannons);
            entities.append(&mut damages);

            for (id, player) in players.iter_mut() {
                player.update(id.clone(), &mut input, &mut platforms, &mut entities.as_mut(), &mut entities.as_mut());
            }
            for (_id, stone) in stones.iter_mut() {
                stone.update(&mut platforms);
            }
            for (_id, cannon) in cannons.iter_mut() {
                cannon.update(&mut players, &mut damages);
            }
            let mut removals = Vec::new();
            for (id, damage) in damages.iter_mut() {
                if damage.update() {
                    removals.push(id.clone());
                }
            }
            for id in removals {
                damages.remove(&id);
            }
            input.update();

            i += 1;
            if i > 8 {
                break;
            }

            ticks += 1;
        }

        if let Some(player) = players.get(&0) {
            player.camera(&target);
        }

        clear_background(LIGHTGRAY);

        for (_id, player) in &mut players {
            player.render(&assets);
        }
        for (_id, stone) in &mut stones {
            stone.render(&assets);
        }
        for (_id, platform) in &mut platforms {
            platform.render();
        }
        for (_id, cannon) in &mut cannons {
            cannon.render(&assets);
        }
        for (_id, damage) in &mut damages {
            damage.render(&assets);
        }

        set_default_camera();

        clear_background(BLACK);
        draw_texture_ex(&target.texture, (screen_width() - screen_height() * 4.0 / 3.0).max(0.0) / 2.0, (screen_height() - screen_width() / 4.0 * 3.0).max(0.0) / 2.0, WHITE, DrawTextureParams {
            dest_size: Some(vec2(screen_width().min(screen_height() * 4.0 / 3.0), screen_height().min(screen_width() / 4.0 * 3.0))),
            ..Default::default()
        });
        
        next_frame().await
    }
}

// const FRAGMENT_SHADER: &'static str = "#version 100
// precision lowp float;

// varying vec2 uv;

// uniform sampler2D Texture;

// void main() {
//     vec4 color = texture2D(Texture, uv);
//     if (color.w < 0.5) {
//         discard;
//     }
//     gl_FragColor = color;
// }
// ";

// const VERTEX_SHADER: &'static str = "#version 100
// precision lowp float;

// attribute vec3 position;
// attribute vec2 texcoord;

// varying vec2 uv;

// uniform mat4 Model;
// uniform mat4 Projection;

// void main() {
//     gl_Position = Projection * Model * vec4(position, 1);
//     uv = texcoord;
// }
// ";