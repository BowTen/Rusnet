use std::{io, net::{SocketAddr, UdpSocket}, sync::mpsc::{self, Receiver, Sender, TryRecvError}, thread::{self, JoinHandle}};
use crate::net::message::{Message, MessageSocket};

pub fn run() -> Result<(JoinHandle<()>, u16, Sender<(Message, SocketAddr)>, Receiver<(Message, SocketAddr)>), String> {
	let (out_sender, out_receiver) = mpsc::channel::<(Message, SocketAddr)>();
	let (in_sender, in_receiver) = mpsc::channel::<(Message, SocketAddr)>();
	let socket = UdpSocket::bind("127.0.0.1:0").map_err(|e| "socket bind err".to_string())?;
	socket.set_nonblocking(true).map_err(|e| "set socket nonblocking err".to_string())?;
	let port = socket.local_addr().map_err(|e| "get port err".to_string())?.port();
	let net_thread = thread::spawn(move ||{
		let mut buf = [0; 4096];
		loop {
			loop {
				match socket.recv_msg_from(&mut buf) {
					Ok(pkg) => in_sender.send(pkg),
					Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
					Err(e) => panic!("receive msg err"),
				};
			}
			loop {
				match out_receiver.try_recv() {
					Ok((msg, addr)) => socket.send_msg_to(&msg, addr),
					Err(ref e) if *e == TryRecvError::Empty => break,
					Err(e) => return (),
				};
			}
		}
	});

	Ok((net_thread, port, out_sender, in_receiver))
}
pub fn run_with_socket(socket: UdpSocket) -> Result<(JoinHandle<()>, Sender<(Message, SocketAddr)>, Receiver<(Message, SocketAddr)>), String> {
	let (out_sender, out_receiver) = mpsc::channel::<(Message, SocketAddr)>();
	let (in_sender, in_receiver) = mpsc::channel::<(Message, SocketAddr)>();
	socket.set_nonblocking(true).map_err(|e| "set socket nonblocking err".to_string())?;
	let net_thread = thread::spawn(move ||{
		let mut buf = [0; 4096];
		loop {
			loop {
				match socket.recv_msg_from(&mut buf) {
					Ok(pkg) => in_sender.send(pkg),
					Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => break,
					Err(e) => panic!("receive msg err"),
				};
			}
			loop {
				match out_receiver.try_recv() {
					Ok((msg, addr)) => socket.send_msg_to(&msg, addr),
					Err(ref e) if *e == TryRecvError::Empty => break,
					Err(e) => return (),
				};
			}
		}
	});

	Ok((net_thread, out_sender, in_receiver))
}