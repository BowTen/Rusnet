use std::{
    cell::RefCell,
    future::Ready,
    io,
    mem::swap,
    net::{SocketAddr, UdpSocket},
    rc::Rc,
    sync::mpsc::{self, Receiver, Sender, TryRecvError, channel},
    thread::{self, JoinHandle},
    time::Duration,
};

use bincode::ErrorKind;
use ggez::{
    Context, GameError, GameResult,
    graphics::{Canvas, Color, Text},
    input::keyboard::{self, KeyCode},
};

use crate::{
    client::{MenuState, OnlineGame},
    common::{Button, GameStateHandler, InputObject, StateResult, StrOption, input_object},
    net::{
        message::{self, Message, MessageSocket},
        udp_net_thread,
    },
};

pub struct RoomState {
    server: SocketAddr,
    buttons: Vec<Rc<RefCell<dyn InputObject>>>,
    selected: usize,
    ready_swp: Rc<RefCell<dyn InputObject>>,
    players: Vec<(SocketAddr, bool)>,
    id: usize,
    map_size: u32,
    cell_size: f32,
    step_time: Duration,
    sender: Sender<(Message, SocketAddr)>,
    receiver: Receiver<(Message, SocketAddr)>,
    net_thread: JoinHandle<()>,
    game: Option<OnlineGame>,
}

impl RoomState {
    pub fn create_room(
        server: SocketAddr,
        socket: UdpSocket,
        host: SocketAddr,
        map_size: u32,
        cell_size: f32,
        step_time: Duration,
    ) -> Result<Self, String> {
        let (net_thread, out_sender, in_receiver) = udp_net_thread::run_with_socket(socket)?;

        let set_ready = Rc::new(RefCell::new(StrOption::new("Ready".to_string())));
        {
            let server = server.clone();
            let sender = out_sender.clone();
            set_ready.borrow_mut().set_key_down_cb(Box::new(
                move |ctx: &mut Context,
                      input: keyboard::KeyInput,
                      repeated: bool|
                      -> Result<StateResult, ggez::GameError> {
                    if let Some(key_code) = input.keycode {
                        if key_code == KeyCode::Return {
                            let msg = Message::SetReady(true);
                            println!("send true");
                            sender.send((msg, server));
                        }
                    }
                    Ok(StateResult::Ok)
                },
            ));
        }
        let set_unready = Rc::new(RefCell::new(StrOption::new("UnReady".to_string())));
        {
            let server = server.clone();
            let sender = out_sender.clone();
            set_unready.borrow_mut().set_key_down_cb(Box::new(
                move |ctx: &mut Context,
                      input: keyboard::KeyInput,
                      repeated: bool|
                      -> Result<StateResult, ggez::GameError> {
                    if let Some(key_code) = input.keycode {
                        if key_code == KeyCode::Return {
                            let msg = Message::SetReady(false);
                            println!("send false");
                            sender.send((msg, server));
                        }
                    }
                    Ok(StateResult::Ok)
                },
            ));
        }
        let start = Rc::new(RefCell::new(StrOption::new("Start".to_string())));
        {
            let server = server.clone();
            let sender = out_sender.clone();
            start.borrow_mut().set_key_down_cb(Box::new(
                move |ctx: &mut Context,
                      input: keyboard::KeyInput,
                      repeated: bool|
                      -> Result<StateResult, ggez::GameError> {
                    if let Some(key_code) = input.keycode {
                        if key_code == KeyCode::Return {
                            let msg = Message::Start;
                            sender.send((msg, server));
                        }
                    }
                    Ok(StateResult::Ok)
                },
            ));
        }
        let exit = Rc::new(RefCell::new(StrOption::new("Exit".to_string())));
        {
            let server = server.clone();
            let sender = out_sender.clone();
            exit.borrow_mut().set_key_down_cb(Box::new(
                move |ctx: &mut Context,
                      input: keyboard::KeyInput,
                      repeated: bool|
                      -> Result<StateResult, ggez::GameError> {
                    if let Some(key_code) = input.keycode {
                        if key_code == KeyCode::Return {
                            let msg = Message::ExitRoom;
                            sender.send((msg, server));
                            return Ok(StateResult::NextState(Box::new(MenuState::new(
                                map_size, cell_size, step_time,
                            ))));
                        }
                    }
                    Ok(StateResult::Ok)
                },
            ));
        }
        let buttons: Vec<Rc<RefCell<dyn InputObject>>> = vec![start, set_ready, exit];
        buttons[0].borrow_mut().focus();

        Ok(RoomState {
            server,
            buttons,
            selected: 0,
            ready_swp: set_unready,
            players: vec![(host, false)],
            id: 0,
            map_size,
            cell_size,
            step_time,
            sender: out_sender,
            receiver: in_receiver,
            net_thread,
            game: None,
        })
    }

    pub fn join_room(
        server: SocketAddr,
        socket: UdpSocket,
        players: Vec<(SocketAddr, bool)>,
        map_size: u32,
        cell_size: f32,
        step_time: Duration,
    ) -> Result<Self, String> {
        let (net_thread, out_sender, in_receiver) = udp_net_thread::run_with_socket(socket)?;

        let set_ready = Rc::new(RefCell::new(StrOption::new("Ready".to_string())));
        {
            let server = server.clone();
            let sender = out_sender.clone();
            set_ready.borrow_mut().set_key_down_cb(Box::new(
                move |ctx: &mut Context,
                      input: keyboard::KeyInput,
                      repeated: bool|
                      -> Result<StateResult, ggez::GameError> {
                    if let Some(key_code) = input.keycode {
                        if key_code == KeyCode::Return {
                            let msg = Message::SetReady(true);
                            println!("send true");
                            sender.send((msg, server));
                        }
                    }
                    Ok(StateResult::Ok)
                },
            ));
        }
        let set_unready = Rc::new(RefCell::new(StrOption::new("UnReady".to_string())));
        {
            let server = server.clone();
            let sender = out_sender.clone();
            set_unready.borrow_mut().set_key_down_cb(Box::new(
                move |ctx: &mut Context,
                      input: keyboard::KeyInput,
                      repeated: bool|
                      -> Result<StateResult, ggez::GameError> {
                    if let Some(key_code) = input.keycode {
                        if key_code == KeyCode::Return {
                            let msg = Message::SetReady(false);
                            println!("send false");
                            sender.send((msg, server));
                        }
                    }
                    Ok(StateResult::Ok)
                },
            ));
        }
        let exit = Rc::new(RefCell::new(StrOption::new("Exit".to_string())));
        {
            let server = server.clone();
            let sender = out_sender.clone();
            exit.borrow_mut().set_key_down_cb(Box::new(
                move |ctx: &mut Context,
                      input: keyboard::KeyInput,
                      repeated: bool|
                      -> Result<StateResult, ggez::GameError> {
                    if let Some(key_code) = input.keycode {
                        if key_code == KeyCode::Return {
                            let msg = Message::ExitRoom;
                            sender.send((msg, server));
                            return Ok(StateResult::NextState(Box::new(MenuState::new(
                                map_size, cell_size, step_time,
                            ))));
                        }
                    }
                    Ok(StateResult::Ok)
                },
            ));
        }
        let buttons: Vec<Rc<RefCell<dyn InputObject>>> = vec![set_ready, exit];
        buttons[0].borrow_mut().focus();

        let id = players.len() - 1;
        Ok(RoomState {
            server,
            buttons,
            selected: 0,
            ready_swp: set_unready,
            players,
            id,
            map_size,
            cell_size,
            step_time,
            sender: out_sender,
            receiver: in_receiver,
            net_thread,
            game: None,
        })
    }

    // start, update-ready, add, remove,
    fn on_message(
        &mut self,
        msg: Message,
        addr: SocketAddr,
    ) -> Result<crate::common::StateResult, ggez::GameError> {
        println!("receive msg");
        if addr != self.server {
            println!("!= server");
            return Ok(StateResult::Ok);
        }
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
            Message::Start => {
                self.game = Some(OnlineGame::new(
                    self.players
                        .iter()
                        .map(|(addr, _)| (addr.to_string(), addr.clone()))
                        .collect(),
                    self.map_size,
                    self.cell_size,
                    self.step_time,
                    self.server,
                    self.sender.clone(),
                ));
                Ok(StateResult::Ok)
            }
            Message::AddPlayer(player) => {
                self.players.push((player, false));
                Ok(StateResult::Ok)
            }
            Message::RemovePlayer(player) => self.remove_player(player),
            Message::UpdateReady { addr, is_ready } => self.update_ready(addr, is_ready),
            _ => Ok(StateResult::Ok),
        }
    }

    fn remove_player(
        &mut self,
        player: SocketAddr,
    ) -> Result<crate::common::StateResult, ggez::GameError> {
        if let Some(i) = self.players.iter().position(|e| e.0 == player) {
            self.players.remove(i);
            if i < self.id {
                self.id -= 1;
            }
        }
        Ok(StateResult::Ok)
    }

    fn update_ready(
        &mut self,
        addr: SocketAddr,
        is_ready: bool,
    ) -> Result<crate::common::StateResult, ggez::GameError> {
        println!("receive ready {}", is_ready);
        if addr == self.players[self.id].0 && is_ready != self.players[self.id].1 {
            if self.id == 0 {
                std::mem::swap(&mut self.buttons[1], &mut self.ready_swp);
            } else {
                std::mem::swap(&mut self.buttons[0], &mut self.ready_swp);
            }
        }
        if let Some((addr, value)) = self.players.iter_mut().find(|e| e.0 == addr) {
            *value = is_ready;
        }
        Ok(StateResult::Ok)
    }
}

impl GameStateHandler for RoomState {
    fn update(
        &mut self,
        ctx: &mut ggez::Context,
    ) -> Result<crate::common::StateResult, ggez::GameError> {
        // TODO: keep alive
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
        ctx: &mut Context,
        canvas: &mut Canvas,
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

        // draw room ip
        let room_ip = Text::new("Room IP: ".to_string() + &self.server.to_string());
        canvas.draw(
            &room_ip,
            ggez::graphics::DrawParam::default().dest([100.0, 50.0]),
        );
        // draw buttons
        for (i, button) in self.buttons.iter().enumerate() {
            button.borrow().draw(
                canvas,
                ggez::graphics::DrawParam::default().dest([100.0, 100.0 + i as f32 * 100.0]),
            );
        }
        // draw players
        for (i, (addr, is_ready)) in self.players.iter().enumerate() {
            let color = if i == 0 as usize {
                Color::YELLOW
            } else {
                Color::WHITE
            };
            let status_text = if *is_ready {
                Text::new("Ready".to_string())
            } else {
                Text::new("UnReady".to_string())
            };
            let addr_text = Text::new(addr.to_string());
            canvas.draw(
                &status_text,
                ggez::graphics::DrawParam::default()
                    .dest([300.0, 100.0 + i as f32 * 100.0])
                    .color(color),
            );
            canvas.draw(
                &addr_text,
                ggez::graphics::DrawParam::default()
                    .dest([400.0, 100.0 + i as f32 * 100.0])
                    .color(color),
            );
        }

        Ok(StateResult::Ok)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: keyboard::KeyInput,
        repeated: bool,
    ) -> Result<StateResult, ggez::GameError> {
        if let Some(game) = &mut self.game {
            let res = game.key_down_event(ctx, input, repeated);
            return match res {
                Ok(StateResult::GameOver) => {
                    self.game = None;
                    Ok(StateResult::Ok)
                }
                _ => res,
            };
        }

        if let Some(key_code) = &input.keycode {
            self.buttons[self.selected].borrow_mut().unfocus();
            match key_code {
                KeyCode::Up => {
                    self.selected += self.buttons.len() - 1;
                    self.selected %= self.buttons.len();
                }
                KeyCode::Down => {
                    self.selected += 1;
                    self.selected %= self.buttons.len();
                }
                _ => {}
            }
            self.buttons[self.selected].borrow_mut().focus();
        }
        match self.buttons[self.selected]
            .borrow_mut()
            .key_down_event(ctx, input, repeated)?
        {
            StateResult::NextState(next_state) => Ok(StateResult::NextState(next_state)),
            StateResult::Ok => Ok(StateResult::Ok),
            StateResult::ShutDown => Ok(StateResult::ShutDown),
            _ => Ok(StateResult::Ok),
        }
    }
}
