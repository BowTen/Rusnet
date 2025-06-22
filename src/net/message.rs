use serde::{Serialize, Deserialize};
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::io::{self, Error};

#[derive(Serialize, Deserialize, Clone)]
pub enum Message {
	NewRoom{
		server_password: String,
		room_password: String,
	},
	JoinRoom{
		password: String,
	},
	SetReady(bool),
	Start,
	//TODO: add game info
	StartState,
	ExitRoom,
	AddPlayer(SocketAddr),
	RemovePlayer(SocketAddr),
	UpdateReady{
		addr: SocketAddr,
		is_ready: bool,
	},
	Response{
		status: StatusCode,
		content: ResponseBody, 
	},
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ResponseBody{
	None,
	Str(String),
	RoomPort(u16),
	RoomInfo{
		players: Vec<(SocketAddr, bool)>,
	},
}

#[derive(Serialize, Deserialize, Clone)]
pub enum StatusCode{
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
			T: ToSocketAddrs {
		let buf = bincode::serialize(req)
			.map_err(|e| Error::new(io::ErrorKind::InvalidData, format!("Serialization failed: {}", e)))?;

		self.send_to(&buf, addr)
	}

	fn recv_msg_from(&self, buf: &mut [u8]) -> io::Result<(Message, SocketAddr)> {
		let (size, addr) = self.recv_from(buf)?;
		let msg = bincode::deserialize(&buf[..size])
			.map_err(|e| Error::new(io::ErrorKind::InvalidData, format!("Serialization failed: {}", e)))?;
		
		Ok((msg, addr))
	}
}