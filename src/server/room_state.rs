use std::{
    future::Ready,
    io,
    net::{SocketAddr, UdpSocket},
    sync::mpsc::{self, Receiver, Sender, TryRecvError, channel},
    thread::{self, JoinHandle},
    time::Duration,
};

use bincode::ErrorKind;
use ggez::{GameError, GameResult};

use crate::{
    common::{GameStateHandler, StateResult},
    net::{
        message::{self, Message, MessageSocket},
        udp_net_thread,
    },
    server::OnlineGame,
};

pub struct RoomState {
    players: Vec<(SocketAddr, bool)>,
    host: SocketAddr,
    password: String,
    map_size: u32,
    cell_size: f32,
    step_time: Duration,
    sender: Sender<(Message, SocketAddr)>,
    receiver: Receiver<(Message, SocketAddr)>,
    net_thread: JoinHandle<()>,
    game: Option<OnlineGame>,
}

impl RoomState {
    pub fn new(
        host: SocketAddr,
        password: String,
        map_size: u32,
        cell_size: f32,
        step_time: Duration,
    ) -> Result<Self, String> {
        let (net_thread, port, out_sender, in_receiver) = udp_net_thread::run()?;

        let msg = Message::Response {
            status: message::StatusCode::OK,
            content: message::ResponseBody::RoomPort(port),
        };
        out_sender
            .send((msg, host))
            .expect("init response send err");

        Ok(RoomState {
            players: vec![(host, false)],
            host,
            password,
            map_size,
            cell_size,
            step_time,
            sender: out_sender,
            receiver: in_receiver,
            net_thread,
            game: None,
        })
    }

    //join,ready,start,exit
    fn on_message(
        &mut self,
        msg: Message,
        addr: SocketAddr,
    ) -> Result<crate::common::StateResult, ggez::GameError> {
        println!("receive msg");
        if let Some(game) = &mut self.game {
            let res = game.on_message(msg, addr);
            return match res {
                Ok(StateResult::GameOver) => {
                    self.game = None;
                    Ok(StateResult::Ok)
                }
                _ => res,
            };
        }
        match msg {
            Message::JoinRoom { password } => self.join(addr, password),
            Message::SetReady(is_ready) => self.set_ready(addr, is_ready),
            Message::Start => self.start(addr),
            Message::ExitRoom => self.exit_room(addr),
            _ => Ok(StateResult::Ok),
        }
    }

    fn add_player(&mut self, addr: SocketAddr) {
        let msg = Message::AddPlayer(addr);
        for player in &self.players {
            self.sender.send((msg.clone(), player.0.clone()));
        }
        self.players.push((addr, false));
    }
    fn join(&mut self, addr: SocketAddr, password: String) -> Result<StateResult, GameError> {
        //check the number of player
        if self.players.len() >= 4 {
            let rsp = Message::Response {
                status: message::StatusCode::FAIL,
                content: message::ResponseBody::Str("room is full".to_string()),
            };
            self.sender.send((rsp, addr));
            return Ok(StateResult::Ok);
        }
        //check password
        if self.password != password {
            let rsp = Message::Response {
                status: message::StatusCode::FAIL,
                content: message::ResponseBody::Str("wrong password".to_string()),
            };
            self.sender.send((rsp, addr));
            return Ok(StateResult::Ok);
        }
        //check repeat
        if self.players.iter().filter(|e| e.0 == addr).count() > 0 {
            let rsp = Message::Response {
                status: message::StatusCode::FAIL,
                content: message::ResponseBody::Str("alread in the room".to_string()),
            };
            self.sender.send((rsp, addr));
            return Ok(StateResult::Ok);
        }
        //join
        self.add_player(addr);
        let rsp = Message::Response {
            status: message::StatusCode::OK,
            content: message::ResponseBody::RoomInfo {
                players: self.players.clone(),
            },
        };
        self.sender.send((rsp, addr));

        Ok(StateResult::Ok)
    }

    fn set_ready(&mut self, addr: SocketAddr, is_ready: bool) -> Result<StateResult, GameError> {
        println!("setready {}", is_ready);
        if let Some((_, value)) = self.players.iter_mut().find(|e| e.0 == addr) {
            *value = is_ready;

            let msg = Message::UpdateReady {
                addr: addr,
                is_ready: is_ready,
            };
            for player in &self.players {
                self.sender.send((msg.clone(), player.0.clone()));
            }
        }
        Ok(StateResult::Ok)
    }

    fn start(&mut self, addr: SocketAddr) -> Result<StateResult, GameError> {
        if addr != self.host {
            return Ok(StateResult::Ok);
        }
        let all_ready = self
            .players
            .iter()
            .fold(true, |all_ready, (_, is_ready)| all_ready && (*is_ready));
        if !all_ready {
            return Ok(StateResult::Ok);
        }
        let msg = Message::Start;
        for (player, _) in &self.players {
            self.sender.send((msg.clone(), player.clone()));
        }
        self.game = Some(OnlineGame::new(
            self.players
                .iter()
                .map(|(addr, _)| (addr.to_string(), addr.clone()))
                .collect(),
            self.map_size,
            self.cell_size,
            self.step_time,
            self.sender.clone(),
        ));
        return Ok(StateResult::Ok);
    }

    fn exit_room(&mut self, addr: SocketAddr) -> Result<StateResult, GameError> {
        if let Some(i) = self.players.iter().position(|e| e.0 == addr) {
            self.players.remove(i);

            let msg = Message::RemovePlayer(addr);
            for (player, _) in &self.players {
                self.sender.send((msg.clone(), player.clone()));
            }
        } else {
            let rsp = Message::Response {
                status: message::StatusCode::FAIL,
                content: message::ResponseBody::None,
            };
            self.sender.send((rsp, addr));
        }
        Ok(StateResult::Ok)
    }
}

impl GameStateHandler for RoomState {
    fn update(
        &mut self,
        ctx: &mut ggez::Context,
    ) -> Result<crate::common::StateResult, ggez::GameError> {
        if self.players.is_empty() {
            return Ok(StateResult::ShutDown);
        }

        loop {
            match self.receiver.try_recv() {
                Ok((msg, addr)) => {
                    let res = self.on_message(msg, addr);
                    match res {
                        Ok(StateResult::Ok) => (),
                        _ => return res,
                    };
                }
                Err(ref e) if *e == TryRecvError::Empty => break,
                Err(e) => return Ok(StateResult::ShutDown),
            };
        }
        // TODO: send keep alive msg

        if let Some(game) = &mut self.game {
            let res = game.update(ctx);
            return match res {
                Ok(StateResult::GameOver) => {
                    self.game = None;
                    Ok(StateResult::Ok)
                }
                _ => res,
            };
        }

        Ok(StateResult::Ok)
    }

    fn draw(
        &mut self,
        ctx: &mut ggez::Context,
        canvas: &mut ggez::graphics::Canvas,
    ) -> Result<StateResult, ggez::GameError> {
        if let Some(game) = &mut self.game {
            let res = game.draw(ctx, canvas);
            return match res {
                Ok(StateResult::GameOver) => {
                    self.game = None;
                    Ok(StateResult::Ok)
                }
                _ => res,
            };
        }

        Ok(StateResult::Ok)
    }
}
