use std::{collections::HashMap, net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket}};

use crate::{hold::stone::Stone, platform::Platform, player::Player, spawners::cannon::Cannon, NetData};

mod level1;

pub struct LevelLoader;
type H<'a, T> = &'a mut HashMap<u64, T>;

pub type E<'a> = (H<'a, Platform>, H<'a, Stone>, H<'a, Cannon>);

impl LevelLoader {
    pub fn load(id: u32, client: bool, socket: Option<&UdpSocket>, platforms: H<Platform>, stones: H<Stone>, cannons: H<Cannon>) -> (f64, f64) {
        if id == 0 {
            platforms.clear();
            stones.clear();
            cannons.clear();
            return (0.0, 0.0);
        }
        if !client {
            match id {
                1 => level1::level1(platforms, stones, cannons),
                _ => (0.0, 0.0)
            }
        }
        else {
            if let Some(socket) = socket {
                let id = NetData::Login { id: socket.local_addr().unwrap() };
                socket.send_to(&bincode::serialize(&id).unwrap(), "127.0.0.1:3400").unwrap();
            }
            (0.0, 0.0)
        }
    }
}

pub fn id1(socket: &UdpSocket, buf: &mut Vec<u8>) {
    let id = socket.local_addr().unwrap();
    if let SocketAddr::V4(id) = id {
        buf.extend(id.ip().octets().iter());
        buf.push((id.port() & 0xFF) as u8);
        buf.push((id.port() >> 8) as u8);
        buf.push(0);
        buf.push(0);
    }
}

pub fn id_1(buf: &[u8]) -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(buf[0], buf[1], buf[2], buf[3])), ((buf[4] as u16) << 8) + buf[5] as u16)
}