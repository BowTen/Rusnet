use std::{cell, net::SocketAddr, time::Duration};

use crate::game::{Direction, Snake};

pub struct Player {
    pub id: String,
    pub addr: SocketAddr,
    pub alive: bool,
    pub snake: Snake,
}

impl Player {
    const PRESET_POSITION: [(f32, f32, u32); 9] = [
        (0.25, 0.25, 1), // 1
        (0.75, 0.75, 0), // 9
        (0.75, 0.25, 1), // 3
        (0.25, 0.75, 0), // 7
        (0.25, 0.50, 3), // 4
        (0.75, 0.50, 2), // 6
        (0.50, 0.25, 1), // 2
        (0.50, 0.75, 0), // 8
        (0.50, 0.50, 0), // 5
    ];

    pub fn new(
        id: String,
        addr: SocketAddr,
        map_size: u32,
        cell_size: f32,
        speed: f32,
        step_time: Duration,
        x: u32,
        y: u32,
        dir: Direction,
    ) -> Self {
        Player {
            id,
            addr,
            alive: true,
            snake: Snake::new_by_position(map_size, cell_size, speed, step_time, x, y, dir),
        }
    }
    pub fn new_by_position(
        id: String,
        addr: SocketAddr,
        map_size: u32,
        cell_size: f32,
        speed: f32,
        step_time: Duration,
        position: usize,
    ) -> Option<Self> {
        if position >= 9 {
            return None;
        }
        let (x, y, dir) = Player::PRESET_POSITION[position];
        let dir = Direction::from_u32(dir);
        let x = ((map_size as f32) * x) as u32;
        let y = ((map_size as f32) * y) as u32;
        Some(Player::new(
            id, addr, map_size, cell_size, speed, step_time, x, y, dir,
        ))
    }
}
