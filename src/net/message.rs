use serde::{Deserialize, Serialize};
use std::collections::LinkedList;
use std::io::{self, Error};
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

use crate::game::{Direction, Segment};

#[derive(Serialize, Deserialize, Clone)]
pub enum Message {
    NewRoom {
        server_password: String,
        room_password: String,
    },
    JoinRoom {
        password: String,
    },
    SetReady(bool),
    Start,
    ExitRoom,
    AddPlayer(SocketAddr),
    RemovePlayer(SocketAddr),
    UpdateReady {
        addr: SocketAddr,
        is_ready: bool,
    },

    Die {
        player: SocketAddr,
    },
    NewFruit {
        x: u32,
        y: u32,
    },
    RemoveFruit {
        x: u32,
        y: u32,
    },
    Trun {
        dir: Direction,
    },
    UpdateTrun {
        addr: SocketAddr,
        head: Segment,
        dir: Direction,
    },

    Response {
        status: StatusCode,
        content: ResponseBody,
    },

    RequestSnake {
        snake: SocketAddr,
    },

    ResynchronizeSnake {
        addr: SocketAddr,
        body: LinkedList<Segment>,
        last_tail: Segment,
        dir: Direction,
        next_dir: [Option<Direction>; 2],
    },
    ResynchronizeFruits {
        fruits: Vec<Vec<bool>>,
    },
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ResponseBody {
    None,
    Str(String),
    RoomPort(u16),
    RoomInfo { players: Vec<(SocketAddr, bool)> },
}

#[derive(Serialize, Deserialize, Clone)]
pub enum StatusCode {
    OK,
    FAIL,
    ERR,
    NOT_FOUND,
}

pub trait MessageSocket {
    fn send_msg_to<T>(&self, req: &Message, addr: T) -> io::Result<usize>
    where
        T: ToSocketAddrs;

    fn recv_msg_from(&self, buf: &mut [u8]) -> io::Result<(Message, SocketAddr)>;
}

impl MessageSocket for UdpSocket {
    fn send_msg_to<T>(&self, req: &Message, addr: T) -> io::Result<usize>
    where
        T: ToSocketAddrs,
    {
        let buf = bincode::serialize(req).map_err(|e| {
            Error::new(
                io::ErrorKind::InvalidData,
                format!("Serialization failed: {}", e),
            )
        })?;

        self.send_to(&buf, addr)
    }

    fn recv_msg_from(&self, buf: &mut [u8]) -> io::Result<(Message, SocketAddr)> {
        let (size, addr) = self.recv_from(buf)?;
        let msg = bincode::deserialize(&buf[..size]).map_err(|e| {
            Error::new(
                io::ErrorKind::InvalidData,
                format!("Serialization failed: {}", e),
            )
        })?;

        Ok((msg, addr))
    }
}
