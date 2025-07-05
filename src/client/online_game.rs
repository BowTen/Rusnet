use crate::common::StateResult;
use crate::common::{GameStateHandler, Player};
use crate::game::snake::MoveResult;
use crate::game::{Direction, Map, Segment, Snake};
use crate::net::message::Message;
use ggez::Context;
use ggez::graphics::Canvas;
use ggez::input::keyboard::{self, KeyCode};
use rand::{Rng, rngs::ThreadRng};
use std::collections::LinkedList;
use std::net::SocketAddr;
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};

use super::MenuState;

pub struct OnlineGame {
    players: Vec<Player>,
    map: Map,
    alive: bool,
    last_update_time: Instant,
    update_duration: Duration,
    map_size: u32,
    cell_size: f32,
    step_time: Duration,
    server: SocketAddr,
    sender: Sender<(Message, SocketAddr)>,
}

impl OnlineGame {
    pub fn new(
        players: Vec<(String, SocketAddr)>,
        map_size: u32,
        cell_size: f32,
        step_time: Duration,
        server: SocketAddr,
        sender: Sender<(Message, SocketAddr)>,
    ) -> Self {
        assert!(players.len() <= 9);

        let speed = cell_size / (step_time.as_millis() as f32);
        let players: Vec<Player> = players
            .into_iter()
            .enumerate()
            .map(|(position, (id, addr))| {
                Player::new_by_position(id, addr, map_size, cell_size, speed, step_time, position)
                    .unwrap()
            })
            .collect();

        Self {
            players,
            map: Map::new(map_size, cell_size),
            alive: true,
            last_update_time: Instant::now(),
            update_duration: Duration::from_millis(150),
            map_size,
            cell_size,
            step_time,
            server,
            sender,
        }
    }

    pub fn on_message(
        &mut self,
        msg: Message,
        addr: SocketAddr,
    ) -> Result<crate::common::StateResult, ggez::GameError> {
        if addr != self.server {
            return Ok(StateResult::Ok);
        }
        match msg {
            Message::UpdateTrun { addr, head, dir } => self.trun_hadnler(addr, head, dir),
            Message::ResynchronizeSnake {
                addr,
                body,
                last_tail,
                dir,
                next_dir,
            } => self.resynchronize_snake(addr, body, last_tail, dir, next_dir),
            Message::ResynchronizeFruits { fruits } => self.resynchronize_fruits(fruits),
            Message::NewFruit { x, y } => self.new_fruit(x, y),
            Message::RemoveFruit { x, y } => self.remove_fruit(x, y),
            Message::Die { player } => self.die_handler(player),
            _ => Ok(StateResult::Ok),
        }
    }

    fn die_handler(
        &mut self,
        player: SocketAddr,
    ) -> Result<crate::common::StateResult, ggez::GameError> {
        println!("get die msg");
        if let Some(player) = self.players.iter_mut().find(|e| e.addr == player) {
            println!("{} die", player.addr);
            player.alive = false;
        }
        Ok(StateResult::Ok)
    }
    fn new_fruit(&mut self, x: u32, y: u32) -> Result<crate::common::StateResult, ggez::GameError> {
        self.map.fruits[x as usize][y as usize] = true;
        Ok(StateResult::Ok)
    }
    fn remove_fruit(
        &mut self,
        x: u32,
        y: u32,
    ) -> Result<crate::common::StateResult, ggez::GameError> {
        self.map.fruits[x as usize][y as usize] = false;
        Ok(StateResult::Ok)
    }

    fn trun_hadnler(
        &mut self,
        addr: SocketAddr,
        head: Segment,
        dir: Direction,
    ) -> Result<crate::common::StateResult, ggez::GameError> {
        let snake = &mut self
            .players
            .iter_mut()
            .find(|e| e.addr == addr)
            .unwrap()
            .snake;
        if snake.get_head() == head {
            snake.trun(dir);
        } else if snake.last_head() == head {
            if snake.just_truned() {
                snake.trun(dir);
            } else {
                snake.back_then_trun(dir);
            }
        } else {
            self.request_resynchronize_snake(addr)?;
        }
        Ok(StateResult::Ok)
    }

    fn request_resynchronize_snake(
        &mut self,
        snake: SocketAddr,
    ) -> Result<crate::common::StateResult, ggez::GameError> {
        let msg = Message::RequestSnake { snake };
        self.sender.send((msg, self.server)).unwrap();
        Ok(StateResult::Ok)
    }

    fn resynchronize_snake(
        &mut self,
        addr: SocketAddr,
        body: LinkedList<Segment>,
        last_tail: Segment,
        dir: Direction,
        next_dir: [Option<Direction>; 2],
    ) -> Result<crate::common::StateResult, ggez::GameError> {
        if let Some(Player { snake, .. }) = self.players.iter_mut().find(|e| e.addr == addr) {
            *snake = Snake {
                body: body,
                last_tail,
                dir,
                next_dir,
                ..*snake
            };
        }
        Ok(StateResult::Ok)
    }
    fn resynchronize_fruits(
        &mut self,
        fruits: Vec<Vec<bool>>,
    ) -> Result<crate::common::StateResult, ggez::GameError> {
        self.map.fruits = fruits;
        Ok(StateResult::Ok)
    }
}

// 服务端：移动、增长、死亡（广播）、生成果实（广播）、接收转向请求、转向（广播）
// 客户端：渲染、移动、处理按键（上传）、接收转向、接收果实、接收死亡
impl GameStateHandler for OnlineGame {
    fn update(&mut self, ctx: &mut Context) -> Result<StateResult, ggez::GameError> {
        // 移动
        for player in &mut self.players {
            if player.alive {
                let Segment { x, y } = player.snake.next_head();
                let got = self.map.eat(x as usize, y as usize);
                let res = player.snake.next(got);
                if res == MoveResult::Grow {
                    println!("Grow");
                }
                if res == MoveResult::Grow && got {
                    self.map.fruits[x as usize][y as usize] = false;
                }
            }
        }

        let live = self
            .players
            .iter()
            .fold(false, |live, player| live || player.alive);
        if !live {
            Ok(StateResult::GameOver)
        } else {
            Ok(StateResult::Ok)
        }
    }

    fn draw(
        &mut self,
        ctx: &mut Context,
        canvas: &mut Canvas,
    ) -> Result<StateResult, ggez::GameError> {
        self.map.draw(ctx, canvas)?;
        for player in &self.players {
            if player.alive {
                player.snake.draw(ctx, canvas)?
            }
        }
        Ok(StateResult::Ok)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: keyboard::KeyInput,
        repeated: bool,
    ) -> Result<StateResult, ggez::GameError> {
        if let Some(key_code) = input.keycode {
            if let Some(dir) = Direction::from_keycode(key_code) {
                let msg = Message::Trun { dir: dir };
                self.sender.send((msg, self.server));
            }
        }

        Ok(StateResult::Ok)
    }
}
