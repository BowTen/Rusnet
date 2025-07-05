use crate::common::StateResult;
use crate::common::{GameStateHandler, Player};
use crate::game::snake::MoveResult;
use crate::game::{Direction, Map, Segment, Snake, map};
use crate::net::message::Message;
use ggez::Context;
use ggez::graphics::Canvas;
use ggez::input::keyboard::{self, KeyCode};
use rand::{Rng, rngs::ThreadRng};
use std::net::SocketAddr;
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};

pub struct OnlineGame {
    players: Vec<Player>,
    map: Map,
    cover_cnt: Vec<Vec<u32>>,
    rng: ThreadRng,
    last_update_time: Instant,
    update_duration: Duration,
    map_size: u32,
    cell_size: f32,
    step_time: Duration,
    sender: Sender<(Message, SocketAddr)>,
}

impl OnlineGame {
    pub fn new(
        players: Vec<(String, SocketAddr)>,
        map_size: u32,
        cell_size: f32,
        step_time: Duration,
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
            cover_cnt: vec![vec![0; (map_size + 2) as usize]; (map_size + 2) as usize],
            rng: rand::thread_rng(),
            last_update_time: Instant::now(),
            update_duration: Duration::from_millis(150),
            map_size,
            cell_size,
            step_time,
            sender,
        }
    }

    pub fn on_message(
        &mut self,
        msg: Message,
        addr: SocketAddr,
    ) -> Result<crate::common::StateResult, ggez::GameError> {
        match msg {
            Message::Trun { dir } => self.trun(addr, dir),
            Message::RequestSnake { snake } => self.resynchronize(addr, snake),
            _ => Ok(StateResult::Ok),
        }
    }

    fn resynchronize(
        &self,
        addr: SocketAddr,
        snake: SocketAddr,
    ) -> Result<crate::common::StateResult, ggez::GameError> {
        println!("resynchronize!!!");
        if let Some(player) = self.players.iter().find(|e| e.addr == snake) {
            let Snake {
                body,
                last_tail,
                dir,
                next_dir,
                ..
            } = player.snake.clone();
            let msg = Message::ResynchronizeSnake {
                addr: snake,
                body,
                last_tail,
                dir,
                next_dir,
            };
            self.sender.send((msg, addr));
        }
        let msg = Message::ResynchronizeFruits {
            fruits: self.map.fruits.clone(),
        };
        self.sender.send((msg, addr));
        Ok(StateResult::Ok)
    }

    pub fn trun(
        &mut self,
        addr: SocketAddr,
        dir: Direction,
    ) -> Result<crate::common::StateResult, ggez::GameError> {
        if let Some(player) = self.players.iter_mut().find(|e| e.addr == addr) {
            if player.snake.trun(dir) {
                let msg = Message::UpdateTrun {
                    addr: player.addr,
                    head: player.snake.get_head(),
                    dir: dir,
                };
                self.broadcast(msg);
            }
            Ok(StateResult::Ok)
        } else {
            Ok(StateResult::Ok)
        }
    }

    pub fn broadcast(&mut self, msg: Message) {
        for player in &self.players {
            self.sender.send((msg.clone(), player.addr));
        }
    }
}

// 服务端：移动、增长、死亡（广播）、移除果实（广播），生成果实（广播）、接收转向请求、转向（广播）
// 客户端：渲染、移动、处理按键（上传）、接收转向、接收增长、接收果实、接收死亡
impl GameStateHandler for OnlineGame {
    fn update(&mut self, ctx: &mut Context) -> Result<StateResult, ggez::GameError> {
        // 移动
        // let mut rfs = Vec::new();
        for player in &mut self.players {
            if player.alive {
                let Segment { x, y } = player.snake.next_head();
                let got = self.map.eat(x as usize, y as usize);
                if player.snake.next(got) == MoveResult::Grow && got {
                    println!("eat!!");
                    self.map.fruits[x as usize][y as usize] = false;
                    // let msg = Message::RemoveFruit { x, y };
                    // rfs.push(msg);
                }
            }
        }
        // for msg in rfs {
        //     self.broadcast(msg);
        // }

        // 死亡
        for row in &mut self.cover_cnt {
            for cnt in row {
                *cnt = 0;
            }
        }
        for player in &self.players {
            if player.alive {
                let body = player.snake.get_body();
                for seg in body {
                    self.cover_cnt[seg.x as usize][seg.y as usize] += 1;
                }
            }
        }
        let mut dies = Vec::new();
        for player in &mut self.players {
            let Segment { x, y } = player.snake.get_head();
            if x < 1
                || x > self.map_size
                || y < 1
                || y > self.map_size
                || self.cover_cnt[x as usize][y as usize] > 1
            {
                println!("{} die", player.addr);
                player.alive = false;
                dies.push(Message::Die {
                    player: player.addr,
                });
            }
        }
        for msg in dies {
            self.broadcast(msg);
        }

        // 生成果实
        if self.last_update_time.elapsed() >= self.update_duration {
            if self.rng.gen_range(0..100) < 5 {
                let (x, y) = self.map.gen_fruit(&mut self.rng);
                self.broadcast(Message::NewFruit { x, y });
            }
            self.last_update_time = Instant::now();
        }

        let live = self
            .players
            .iter()
            .fold(false, |live, player| live || player.alive);
        if !live {
            println!("Game Over!");
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
}
