use crate::common::GameStateHandler;
use crate::common::StateResult;
use crate::game::snake::MoveResult;
use crate::game::{Direction, Map, Segment, Snake};
use ggez::Context;
use ggez::graphics::Canvas;
use ggez::input::keyboard::{self, KeyCode};
use rand::{Rng, rngs::ThreadRng};
use std::time::{Duration, Instant};

use super::MenuState;

pub struct ClassicGame {
    snake: Snake,
    map: Map,
    rng: ThreadRng,
    last_update_time: Instant,
    update_interval: Duration,
    game_over: bool,
    map_size: u32,
    cell_size: f32,
    step_time: Duration,
}

impl ClassicGame {
    pub fn new(map_size: u32, cell_size: f32, step_time: Duration) -> Self {
        Self {
            snake: Snake::new(
                map_size,
                cell_size,
                cell_size / (step_time.as_millis() as f32),
                step_time,
            ),
            map: Map::new(map_size, cell_size),
            rng: rand::thread_rng(),
            last_update_time: Instant::now(),
            update_interval: Duration::from_millis(150),
            game_over: false,
            map_size,
            cell_size,
            step_time,
        }
    }

    fn restart(&mut self) {
        self.snake = Snake::new(
            self.map_size,
            self.cell_size,
            self.cell_size / (self.step_time.as_millis() as f32),
            self.step_time,
        );
        self.map = Map::new(self.map_size, self.cell_size);
        self.game_over = false;
    }
}

impl GameStateHandler for ClassicGame {
    fn update(&mut self, ctx: &mut Context) -> Result<StateResult, ggez::GameError> {
        let Segment { x, y } = self.snake.next_head();
        if self.snake.next(self.map.eat(x as usize, y as usize)) == MoveResult::Die {
            self.game_over = true;
            return Ok(StateResult::NextState(Box::new(MenuState::new(
                self.map_size,
                self.cell_size,
                self.step_time,
            ))));
        }

        // 每隔一段时间更新一次
        if self.last_update_time.elapsed() >= self.update_interval {
            // 生成新水果
            if self.rng.gen_range(0..100) < 5 {
                self.map.gen_fruit(&mut self.rng);
            }

            self.last_update_time = Instant::now();
        }

        Ok(StateResult::Ok)
    }

    fn draw(
        &mut self,
        ctx: &mut Context,
        canvas: &mut Canvas,
    ) -> Result<StateResult, ggez::GameError> {
        // 绘制地图
        self.map.draw(ctx, canvas)?;

        // 绘制蛇
        self.snake.draw(ctx, canvas)?;

        Ok(StateResult::Ok)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<StateResult, ggez::GameError> {
        if let Some(key_code) = input.keycode {
            match key_code {
                KeyCode::Escape => {
                    self.game_over = true;
                }
                KeyCode::R => {
                    self.restart();
                }
                _ => {
                    if let Some(dir) = Direction::from_keycode(key_code) {
                        self.snake.trun(dir);
                    }
                }
            }
        }

        Ok(StateResult::Ok)
    }
}
