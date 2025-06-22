use std::{net::SocketAddr, time::Duration};
use rusnet::server::{room_state::{RoomState}, MainState};

const CELL_SIZE: f32 = 35.0; // 每个格子大小
const MAP_SIZE: u32 = 35;    // 地图大小（30x30 格子）
const STEP_TIME: Duration = Duration::from_millis(180);

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let host: SocketAddr = args[1].parse().expect("invalid addr");
	let password = args[2].clone();

    let (mut ctx, event_loop) = ggez::ContextBuilder::new("snake_game", "Your Name")
        .window_mode(ggez::conf::WindowMode::default().visible(false))
        .build()
        .expect("无法创建上下文");

	let room_state = RoomState::new(host, password, MAP_SIZE, CELL_SIZE, STEP_TIME).expect("Failed to create room");
    let main_state = MainState::new(&mut ctx, Box::new(room_state), MAP_SIZE, CELL_SIZE, STEP_TIME).expect("Failed to create room");
    ggez::event::run(ctx, event_loop, main_state);
}