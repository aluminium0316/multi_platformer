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
mod startarea;

use std::env::args;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::os::unix::net;
use std::time::{Instant, SystemTime};
use std::{io, thread};
use std::{collections::HashMap, sync::atomic::AtomicU64};

use entity::Entities;
use hold::stone::Stone;
use input::Input;
use levels::{id_1, LevelLoader};
use macroquad::{prelude::*, window};
use miniquad::window::set_window_position;
use platform::Platform;
use player::Player;
use projectiles::damage::Damage;
use projectiles::path::{self, Path};
use scene::end::UI;
use spawners::cannon::Cannon;
use startarea::Startarea;

fn window_conf() -> Conf {
    Conf {
        window_title: "43".to_owned(),
        window_width: 256 * 2,
        window_height: 192 * 2,
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

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum NetData {
    Login { id: String },
    Input { id: String, input: Input },
    World { data: (HashMap<u64, Player>, HashMap<u64, Platform>, HashMap<u64, Stone>, HashMap<u64, Damage>, HashMap<u64, Cannon>, HashMap<u64, Path>, bool, Startarea) },
    Id { id: u64 },
    Denial,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut client = true;
    let mut port = 0;
    let mut username = String::new();
    for arg in args() {
        if arg == "s" {
            client = false;
        }
        if let Ok(id) = arg.parse::<u16>()  {
            port = id;
        }
        if arg.chars().nth(0) == Some('u') {
            let (_, uname) = arg.split_at(1);
            username = uname.to_owned();
        }
    }

    let mut input = Input::new();
    let mut scene = Scene::Start;
    let mut start = false;
    let mut startarea = Startarea::new(0.0, 0.0, 0.0, 0.0);
    let mut stpos = (0.0, 0.0);

    let mut players = HashMap::new();
    let mut platforms = HashMap::new();
    let mut stones = HashMap::new();
    let mut damages = HashMap::new();
    let mut cannons = HashMap::new();
    let mut paths = HashMap::new();
    let mut player_id = new_id();
    players.insert(player_id, Player::new(username.clone(), 0.0, 0.0));
    LevelLoader::load(0, client, username.clone(), None, &mut platforms, &mut stones, &mut cannons, &mut damages, &mut paths);
    
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
    
    // let mut fullscreen = true;

    let socket = if client { 
        UdpSocket::bind(format!("127.0.0.1:{}", port))
    } 
    else {
        UdpSocket::bind("127.0.0.1:3400")
    }.expect("12");
    socket.set_nonblocking(true).unwrap();

    let mut clients = HashMap::new();

    loop {
        input.input();

        // if input.down[key!(F11)] == 0 {
        //     fullscreen ^= true;
        //     set_fullscreen(fullscreen);
        // }

        let ns = t.elapsed().as_nanos();
        dt += ns - prev_ns;
        prev_ns = ns;
        let mut i = 0;

        while dt > 1000000000/fps {
            dt -= 1000000000/fps;
            
            if let Scene::Gameplay = scene {
                entities.0.clear();
                entities.append(&mut stones);
                entities.append(&mut players);
                entities.append(&mut cannons);
                entities.append(&mut damages);

                if let Some(player) = players.get_mut(&player_id) {
                    player.input.set(&input);
                }
                update(&mut players, |id, player| {
                    player.update(id.clone(), &mut platforms, &mut players, &mut entities.as_mut(), &mut entities.as_mut(), &mut scene, &mut start);
                    false
                });
                startarea.update(&mut start, &mut players);
                if !start {
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
                    update(&mut paths, |_id, path| {
                        path.update(&mut entities.as_mut());
                        false
                    });
                }
            }

            if let Scene::End { .. } = scene {
                clients.clear();
            }

            if !client {
                let mut buf = [0; 65536];
                let world = NetData::World { data: (players.clone(), platforms.clone(), stones.clone(), damages.clone(), cannons.clone(), paths.clone(), start, startarea.clone()) };
                loop {
                    if let Some((amt, src)) = match socket.recv_from(&mut buf) {
                        Ok(n) => Some(n),
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                        Err(e) => panic!("encountered IO error: {e}"),
                    } {
                        let buf = &mut buf[..amt];
                        let netdata: NetData = bincode::deserialize(&buf).unwrap();
                        match netdata {
                            NetData::Login { id } => {
                                if start {
                                    if !clients.contains_key(&id) {
                                        let client_id = new_id();
                                        players.insert(client_id, Player::new(id.clone(), stpos.0, stpos.1));
                                        clients.insert(id.clone(), client_id);
                                        socket.send_to(&bincode::serialize(&NetData::Id { id: client_id }).unwrap(), src).unwrap();
                                    }
                                    // let client_id = clients.get(&client).unwrap();
                                    socket.send_to(&bincode::serialize(&world).unwrap(), src).unwrap();
                                }
                                else {
                                    socket.send_to(&bincode::serialize(&NetData::Denial).unwrap(), src).unwrap();
                                }
                            },
                            NetData::Input { id, input } => {
                                if let Some(client_id) = clients.get(&id) {
                                    let client = players.get_mut(client_id).unwrap();
                                    client.input.set(&input);
                                    socket.send_to(&bincode::serialize(&world).unwrap(), src).unwrap();
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            else {
                let mut buf = [0; 65536];
                loop {
                    if let Some((amt, src)) = match socket.recv_from(&mut buf) {
                        Ok(n) => Some(n),
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
                        Err(e) => panic!("encountered IO error: {e}"),
                    } {
                        let buf = &mut buf[..amt];
                        let netdata: NetData = bincode::deserialize(&buf).unwrap();
                        match netdata {
                            NetData::World { data } => {
                                (players, platforms, stones, damages, cannons, paths, start, startarea) = data;
                            },
                            NetData::Id { id } => {
                                player_id = id;
                            },
                            NetData::Denial => {
                                scene = Scene::Start;
                            },
                            _ => {}
                        }
                    }
                }
                
                if let Scene::Gameplay = scene {
                    let id = NetData::Input { id: username.clone(), input: input.clone() };
                    socket.send_to(&bincode::serialize(&id).unwrap(), "127.0.0.1:3400").unwrap(); 
                }
            }
            input.update();

            i += 1;
            if i > 8 {
                break;
            }

            if let Scene::End { .. } = scene { } else {
                if !start {
                    ticks += 1;
                }
            }
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
        for (_id, path) in &mut paths {
            path.render();
        }
        startarea.render();

        // for (_id, client) in &mut clients {
        //     let (x, y) = players.get(&client).unwrap().pos();
        //     draw_circle(x as f32, y as f32, 4.0, MAGENTA);
        // }

        set_camera(&Camera2D {
            target: vec2(0.0, 0.0),
            render_target: Some(target.clone()),
            zoom: vec2( 1.0 / 128.0 , 1.0 / 96.0),
            ..Default::default()
        });

        if let Scene::Start = scene { } else {
            startarea.ui(&start);
        }
        UI::ui(&mut scene, &mut ticks, &client);
        if let Scene::Restart{ level } = scene {
            scene = Scene::Gameplay;
            players.clear();
            player_id = new_id();
            LevelLoader::load(0, client, username.clone(), Some(&socket), &mut platforms, &mut stones, &mut cannons, &mut damages, &mut paths);
            let (x, y) = LevelLoader::load(level, client, username.clone(), Some(&socket), &mut platforms, &mut stones, &mut cannons, &mut damages, &mut paths);
            stpos = (x, y);
            players.insert(player_id, Player::new(username.clone(), x, y));
            start = true;
            startarea = Startarea::new(x - 32.0, y - 12.0, 64.0, 24.0);
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

fn update<T>(vec: *mut HashMap<u64, T>, mut f: impl FnMut(&u64, &mut T) -> bool) {
    unsafe {
        let mut removals = Vec::new();
        for (id, t) in vec.as_mut().unwrap().iter_mut() {
            if f(id, t) {
                removals.push(id.clone());
            }
        }
        for id in removals {
            vec.as_mut().unwrap().remove(&id);
        }
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