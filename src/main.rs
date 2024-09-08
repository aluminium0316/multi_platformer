mod input;
mod player;
mod platform;
mod vector;
mod projectiles;
mod hold;
mod entity;
mod spawners;
mod scene;
mod levels;
mod networking;

use std::env::args;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Ipv4Addr, TcpListener, TcpStream, UdpSocket};
use std::time::{Instant, SystemTime};
use std::{io, thread};
use std::{collections::HashMap, sync::atomic::AtomicU64};

use entity::Entities;
use hold::stone::Stone;
use input::Input;
use levels::LevelLoader;
use macroquad::{prelude::*, window};
use platform::Platform;
use player::Player;
use scene::end::UI;
use spawners::cannon::Cannon;

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

#[derive(Clone)]
pub enum Scene {
    Start,
    Gameplay,
    End { winner: String },
    Restart { level: u32, },
}

// pub fn open_window() {
//     thread::spawn(move || {
//         macroquad::Window::from_config(window_conf(), macroquad_main());
//     });
// }

enum Client {
    Listener(TcpListener),
    Stream(TcpStream)
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut client = true;
    for arg in args() {
        if arg == "s" {
            client = false;
        }
    }

    let mut input = Input::new();
    let mut scene = Scene::Start;

    let mut players = HashMap::new();
    let mut platforms = HashMap::new();
    let mut stones = HashMap::new();
    let mut damages = HashMap::new();
    let mut cannons = HashMap::new();
    let mut player_id = new_id();
    players.insert(player_id, Player::new());
    LevelLoader::load(0, &mut platforms, &mut stones, &mut cannons);
    
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

    let socket = if client { 
        UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))
    } 
    else {
        UdpSocket::bind("127.0.0.1:3400")
    }.expect("12");
    socket.set_nonblocking(true).unwrap();

    let mut clients = HashMap::new();

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

        if let Scene::Gameplay = scene {
            while dt > 1000000000/fps {
                dt -= 1000000000/fps;

                entities.0.clear();
                entities.append(&mut stones);
                entities.append(&mut players);
                entities.append(&mut cannons);
                entities.append(&mut damages);

                update(&mut players, |id, player| {
                    player.update(id.clone(), &mut input, &mut platforms, &mut entities.as_mut(), &mut entities.as_mut(), &mut scene);
                    false
                });
                update(&mut stones, |_id, stone| {
                    stone.update(&mut platforms);
                    false
                });
                update(&mut cannons, |_id, cannon| {
                    cannon.update(&mut players, &mut damages);
                    false
                });
                update(&mut damages, |_id, damage| {
                    damage.update()
                });
                input.update();

                i += 1;
                if i > 8 {
                    break;
                }

                ticks += 1;

                if !client {
                    let mut buf = [0; 256];
                    loop {
                        if let Some((amt, src)) = match socket.recv_from(&mut buf) {
                            Ok(n) => Some(n),
                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                            Err(e) => panic!("encountered IO error: {e}"),
                        } {
                            let buf = &mut buf[..amt];
                            let out = vec![0];
                            if buf[0] == 0 {
                                let x = f64::from_be_bytes(buf[2..10].try_into().unwrap());
                                let y = f64::from_be_bytes(buf[10..18].try_into().unwrap());
                                // draw_circle(x as f32, y as f32, 4.0, MAGENTA);
                                if !clients.contains_key(&buf[1]) {
                                    clients.insert(buf[1], (0.0, 0.0));
                                }
                                *clients.get_mut(&buf[1]).unwrap() = (x, y);
                            }
                            // socket.send_to(buf, &src).unwrap();
                        }
                    }
                }
                else {
                    let buf1 = players.get(&player_id).unwrap().pos();
                    let mut buf = vec![0, 1];
                    buf.extend(buf1.0.to_be_bytes().iter());
                    buf.extend(buf1.1.to_be_bytes().iter());
                    socket.send_to(&buf, "127.0.0.1:3400").unwrap();
                }
            }
        }
        else {
            dt = 0;
        }

        if let Some(player) = players.get(&player_id) {
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

        for (_id, client) in &mut clients {
            draw_circle(client.0 as f32, client.1 as f32, 4.0, MAGENTA);
        }

        set_camera(&Camera2D {
            target: vec2(0.0, 0.0),
            render_target: Some(target.clone()),
            zoom: vec2( 1.0 / 128.0 , 1.0 / 96.0),
            ..Default::default()
        });

        UI::ui(&mut scene, &mut ticks);
        if let Scene::Restart{ level } = scene {
            scene = Scene::Gameplay;
            players.clear();
            player_id = new_id();
            players.insert(player_id, Player::new());
            LevelLoader::load(0, &mut platforms, &mut stones, &mut cannons);
            // if !client {
                LevelLoader::load(level, &mut platforms, &mut stones, &mut cannons);
            // }
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

fn update<T>(vec: &mut HashMap<u64, T>, mut f: impl FnMut(&u64, &mut T) -> bool) {
    let mut removals = Vec::new();
    for (id, t) in vec.iter_mut() {
        if f(id, t) {
            removals.push(id.clone());
        }
    }
    for id in removals {
        vec.remove(&id);
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