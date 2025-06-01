use std::net::UdpSocket;
use rusnet::net::message::Message;

const PASSWORD: &str = "666666";

fn main() {
	//bind socket
	let socket = UdpSocket::bind("127.0.0.1:8888").expect("Server Socket Bind Err");
	
	//listen for request to create room
	let mut buffer = [0; 1024];
	loop {
		let (size, client) = socket.recv_from(&mut buffer).expect("recv err");
		if let Ok(msg) = bincode::deserialize(&buffer[..size]) {
			match msg {
				Message::NewRoom { password } => {
					//check password
					if password == PASSWORD {
						//create room
						println!("create a new room for client[{}]", client);
					}else {
						println!("Password err");
					}
				},
				Message::JoinRoom { password } => {
					println!("receive join message, skip");
				}
			}
		}else {
			println!("Packet cannot be deserialize");
		}
	}	
}