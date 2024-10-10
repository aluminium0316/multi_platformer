use std::{collections::HashMap, net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket}};

use crate::{hold::stone::Stone, platform::Platform, player::Player, projectiles::{damage::Damage, path::Path}, spawners::cannon::Cannon, NetData};

mod level1;

pub struct LevelLoader;
type H<'a, T> = &'a mut HashMap<u64, T>;

pub type E<'a> = (H<'a, Platform>, H<'a, Stone>, H<'a, Cannon>);

impl LevelLoader {
    pub fn load(id: u32, client: bool, username: String, socket: Option<&UdpSocket>, server_ip: &str, platforms: H<Platform>, stones: H<Stone>, cannons: H<Cannon>, damages: H<Damage>, paths: H<Path>) -> (f64, f64) {
        if id == 0 {
            platforms.clear();
            stones.clear();
            cannons.clear();
            damages.clear();
            paths.clear();
            return (0.0, 0.0);
        }
        if !client {
            match id {
                1 => level1::level1(platforms, stones, cannons, paths),
                _ => (0.0, 0.0)
            }
        }
        else {
            if let Some(socket) = socket {
                let id = NetData::Login { id: username };
                socket.send_to(&bincode::serialize(&id).unwrap(), server_ip).unwrap();
            }
            (0.0, 0.0)
        }
    }
}
