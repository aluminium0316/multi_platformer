mod input;
mod player;
mod platform;
mod vector;
mod projectiles;
mod hold;

use input::Input;
use macroquad::prelude::*;
use platform::Platform;
use player::Player;
use rand::rand;

fn window_conf() -> Conf {
    Conf {
        window_title: "43".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut input = Input::new();
    let player_id = ((rand() as u64) << 32) | rand() as u64;
    let mut players = vec![Player::new(player_id), Player::new(12)];

    let mut platforms = vec![Platform::new()];
    let target = render_target(256, 192);
    target.texture.set_filter(FilterMode::Nearest);

    // client.as_ref();
    let t = std::time::Instant::now();
    let mut prev_ns = 0;
    let mut dt = 0;
    let fps = 240;
    let mut ticks = 0;
    let assets: Vec<Texture2D> = vec![
        // load_texture("assets/player.png").await.unwrap(),
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

            for player in players.iter_mut() {
                player.update(&mut input, &mut platforms);
            }
            input.update();

            i += 1;
            if i > 8 {
                break;
            }

            ticks += 1;
        }

        if let Some(player) = players.iter().filter(|x| x.id == player_id).next() {
            player.camera(&target);
        }

        clear_background(LIGHTGRAY);

        for player in &mut players {
            player.render(&assets);
        }
        for platform in &mut platforms {
            platform.render();
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