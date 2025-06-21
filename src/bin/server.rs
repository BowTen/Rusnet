use std::{net::UdpSocket, process, thread, time::Duration};
use ggez::{conf::WindowMode, winit::{event_loop::EventLoopBuilder, platform::windows::EventLoopBuilderExtWindows}, Context, ContextBuilder};
use rusnet::{net::message::{Message, MessageSocket, ResponseBody, StatusCode}, server};

const CELL_SIZE: f32 = 35.0; // 每个格子大小
const MAP_SIZE: u32 = 35;    // 地图大小（30x30 格子）
const STEP_TIME: Duration = Duration::from_millis(180);

const PASSWORD: &str = "666666";

fn main() {
	//bind socket
	let socket = UdpSocket::bind("127.0.0.1:8888").expect("Server Socket Bind Err");
	
	//listen for request to create room
	let mut buffer = [0; 1024];
	loop {
		let (msg, client) = socket.recv_msg_from(&mut buffer).expect("recv err");
			match msg {
				Message::NewRoom { server_password, room_password } => {
					//check password
					if server_password == PASSWORD {
						//create room
						if let Err(_) = process::Command::new(r"target\debug\server_room.exe")
							.arg(client.to_string())
							.arg(room_password.clone())
							.spawn() {
								let msg = Message::Response { status: StatusCode::ERR, content: ResponseBody::None };
								socket.send_msg_to(&msg, client);
							}
						println!("create a new room for client[{}], room password: {}", client, room_password);
					}else {
						println!("Password err");
						socket.send_msg_to(
							&Message::Response { 
								status: rusnet::net::message::StatusCode::FAIL, 
								content: rusnet::net::message::ResponseBody::None 
							}, 
							client
						);
					}
				},
				Message::JoinRoom { password } => {
					println!("receive join message, skip");
				},
				_ => ()
			}
	}	
}