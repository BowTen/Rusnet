use super::{ClassicGame, MainState, MenuState};
use crate::client::room_state::RoomState;
use crate::common::{GameStateHandler, InputBox, InputObject, StateResult, StrOption, str_option};
use crate::net::message::{Message, MessageSocket, ResponseBody, StatusCode};
use ggez::Context;
use ggez::event::EventHandler;
use ggez::graphics::{self, Canvas, Color, Text};
use ggez::input::keyboard::{self, KeyCode};
use std::cell::RefCell;
use std::net::{SocketAddr, UdpSocket};
use std::rc::Rc;
use std::str::FromStr;
use std::time::Duration;

pub struct RoomMenu {
    selected_mode: usize,
    selected_box: usize,
    mode_options: Vec<StrOption>,
    input_boxes: [Vec<Rc<RefCell<dyn InputObject>>>; 2],
    // input_boxes2: Vec<Rc<RefCell<dyn InputObject>>>,
    map_size: u32,
    cell_size: f32,
    step_time: Duration,
}

impl RoomMenu {
    pub fn new(ctx: &Context, map_size: u32, cell_size: f32, step_time: Duration) -> Self {
        let mut room_menu = RoomMenu {
            selected_mode: 0,
            selected_box: 0,
            mode_options: vec![
                StrOption::new("Create a room".to_string()),
                StrOption::new("Join a room".to_string()),
            ],
            input_boxes: [
                vec![
                    Rc::new(RefCell::new(InputBox::new(
                        ctx,
                        "Server Address".to_string(),
                    ))),
                    Rc::new(RefCell::new(InputBox::new(
                        ctx,
                        "Server PassWord".to_string(),
                    ))),
                    Rc::new(RefCell::new(InputBox::new(
                        ctx,
                        "Set Room PassWord".to_string(),
                    ))),
                ],
                vec![
                    Rc::new(RefCell::new(InputBox::new(
                        ctx,
                        "Server Address".to_string(),
                    ))),
                    Rc::new(RefCell::new(InputBox::new(
                        ctx,
                        "Room PassWord".to_string(),
                    ))),
                ],
            ],
            map_size,
            cell_size,
            step_time,
        };
        for str_option in &mut room_menu.mode_options {
            str_option.set_scale(25.0);
        }
        room_menu.mode_options[room_menu.selected_mode].focus();
        room_menu.input_boxes[0][room_menu.selected_box]
            .borrow_mut()
            .focus();
        room_menu.input_boxes[1][room_menu.selected_box]
            .borrow_mut()
            .focus();

        let button = Rc::new(RefCell::new(StrOption::new("OK".to_string())));
        button.borrow_mut().set_scale(35.0);
        {
            let address = room_menu.input_boxes[0][0].clone();
            let server_password = room_menu.input_boxes[0][1].clone();
            let room_password = room_menu.input_boxes[0][2].clone();
            button.borrow_mut().set_key_down_cb(Box::new(
                move |ctx: &mut Context,
                      input: keyboard::KeyInput,
                      repeated: bool|
                      -> Result<StateResult, ggez::GameError> {
                    if let Some(key_code) = input.keycode {
                        if key_code == KeyCode::Return {
                            let mut address = address.borrow_mut();
                            let mut server_password = server_password.borrow_mut();
                            let mut room_password = room_password.borrow_mut();
                            let abox = address.as_any_mut().downcast_mut::<InputBox>().unwrap();
                            let spbox = server_password
                                .as_any_mut()
                                .downcast_mut::<InputBox>()
                                .unwrap();
                            let rpbox = room_password
                                .as_any_mut()
                                .downcast_mut::<InputBox>()
                                .unwrap();
                            let addr = abox.content();
                            let spswd = spbox.content();
                            let rpswd = rpbox.content();
                            println!("{addr}\n{spswd}\n{rpswd}");

                            match UdpSocket::bind("127.0.0.1:0") {
                                Ok(socket) => {
                                    let msg = Message::NewRoom {
                                        server_password: spswd,
                                        room_password: rpswd,
                                    };
                                    if let Err(_) = socket.send_msg_to(&msg, addr) {
                                        abox.set_content("request err".to_string());
                                        return Ok(StateResult::Ok);
                                    }
                                    let mut buf = [0; 1024];
                                    socket.set_read_timeout(Some(Duration::from_secs(5)));
                                    match socket.recv_msg_from(&mut buf) {
                                        Ok((rsp, server)) => {
                                            match rsp {
                                                Message::Response { status, .. } => {
                                                    match status {
                                                        StatusCode::OK => {
                                                            //TODO: return room
                                                            let host = socket.local_addr().unwrap();
                                                            let room = RoomState::create_room(
                                                                server, socket, host, map_size,
                                                                cell_size, step_time,
                                                            )
                                                            .unwrap();
                                                            return Ok(StateResult::NextState(
                                                                Box::new(room),
                                                            ));
                                                            spbox.set_content("OK".to_string());
                                                        }
                                                        _ => spbox.set_content(
                                                            "password error".to_string(),
                                                        ),
                                                    }
                                                }
                                                _ => abox
                                                    .set_content("unexpected message".to_string()),
                                            }
                                        }
                                        Err(e) => {
                                            println!("{:?}", e);
                                            abox.set_content(e.to_string());
                                        }
                                    }
                                }
                                Err(_) => abox.set_content("socket bind err".to_string()),
                            }
                        }
                    }

                    Ok(StateResult::Ok)
                },
            ));
        }
        room_menu.input_boxes[0].push(button);

        let button = Rc::new(RefCell::new(StrOption::new("OK".to_string())));
        button.borrow_mut().set_scale(35.0);
        {
            let address = room_menu.input_boxes[1][0].clone();
            let room_password = room_menu.input_boxes[1][1].clone();
            button.borrow_mut().set_key_down_cb(Box::new(
                move |ctx: &mut Context,
                      input: keyboard::KeyInput,
                      repeated: bool|
                      -> Result<StateResult, ggez::GameError> {
                    if let Some(key_code) = input.keycode {
                        if key_code == KeyCode::Return {
                            let mut address = address.borrow_mut();
                            let mut room_password = room_password.borrow_mut();
                            let abox = address.as_any_mut().downcast_mut::<InputBox>().unwrap();
                            let rpbox = room_password
                                .as_any_mut()
                                .downcast_mut::<InputBox>()
                                .unwrap();
                            let addr = abox.content();
                            let rpswd = rpbox.content();
                            println!("{addr}\n{rpswd}");

                            match UdpSocket::bind("127.0.0.1:0") {
                                Ok(socket) => {
                                    let msg = Message::JoinRoom { password: rpswd };
                                    if let Err(_) = socket.send_msg_to(&msg, addr) {
                                        abox.set_content("request err".to_string());
                                        return Ok(StateResult::Ok);
                                    }
                                    let mut buf = [0; 1024];
                                    socket.set_read_timeout(Some(Duration::from_secs(5)));
                                    match socket.recv_msg_from(&mut buf) {
                                        Ok((rsp, server)) => {
                                            match rsp {
                                                Message::Response { status, content } => {
                                                    match status {
                                                        StatusCode::OK => {
                                                            //TODO: return room
                                                            let players =
                                                                if let ResponseBody::RoomInfo {
                                                                    players,
                                                                } = content
                                                                {
                                                                    players
                                                                } else {
                                                                    panic!("err rsponse type");
                                                                };
                                                            let room = RoomState::join_room(
                                                                server, socket, players, map_size,
                                                                cell_size, step_time,
                                                            )
                                                            .unwrap();
                                                            return Ok(StateResult::NextState(
                                                                Box::new(room),
                                                            ));
                                                            rpbox.set_content("OK".to_string());
                                                        }
                                                        _ => rpbox.set_content(
                                                            "password error".to_string(),
                                                        ),
                                                    }
                                                }
                                                _ => abox
                                                    .set_content("unexpected message".to_string()),
                                            }
                                        }
                                        Err(e) => {
                                            println!("{:?}", e);
                                            abox.set_content(e.to_string());
                                        }
                                    }
                                }
                                Err(_) => abox.set_content("socket bind err".to_string()),
                            }
                        }
                    }

                    Ok(StateResult::Ok)
                },
            ));
        }
        room_menu.input_boxes[1].push(button);

        room_menu
    }
}

impl GameStateHandler for RoomMenu {
    fn draw(
        &mut self,
        _ctx: &mut Context,
        canvas: &mut Canvas,
    ) -> Result<StateResult, ggez::GameError> {
        // draw mode options
        for (i, mode) in self.mode_options.iter().enumerate() {
            let color = if i == self.selected_mode as usize {
                Color::YELLOW
            } else {
                Color::WHITE
            };
            canvas.draw(
                mode,
                ggez::graphics::DrawParam::default()
                    .dest([100.0 + i as f32 * 250.0, 100.0])
                    .color(color),
            );
        }
        // draw input options
        for (i, input) in self.input_boxes[self.selected_mode].iter().enumerate() {
            let color = if i == self.selected_box as usize {
                Color::YELLOW
            } else {
                Color::WHITE
            };
            input.borrow_mut().draw(
                canvas,
                ggez::graphics::DrawParam::default()
                    .dest([100.0, 100.0 + (i + 1) as f32 * 100.0])
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
        if let Some(key_code) = &input.keycode {
            self.input_boxes[self.selected_mode][self.selected_box]
                .borrow_mut()
                .unfocus();
            self.mode_options[self.selected_mode].unfocus();
            match key_code {
                KeyCode::Up => {
                    self.selected_box += self.input_boxes[self.selected_mode].len() - 1;
                    self.selected_box %= self.input_boxes[self.selected_mode].len();
                }
                KeyCode::Down => {
                    self.selected_box += 1;
                    self.selected_box %= self.input_boxes[self.selected_mode].len();
                }
                KeyCode::Left => {
                    self.selected_mode += self.mode_options.len() - 1;
                    self.selected_mode %= self.mode_options.len();
                    self.selected_box = 0;
                }
                KeyCode::Right => {
                    self.selected_mode += 1;
                    self.selected_mode %= self.mode_options.len();
                    self.selected_box = 0;
                }
                KeyCode::Escape => {
                    return Ok(StateResult::NextState(Box::new(MenuState::new(
                        self.map_size,
                        self.cell_size,
                        self.step_time,
                    ))));
                }
                _ => {}
            }
            self.input_boxes[self.selected_mode][self.selected_box]
                .borrow_mut()
                .focus();
            self.mode_options[self.selected_mode].focus();
        }
        match self.input_boxes[self.selected_mode][self.selected_box]
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
