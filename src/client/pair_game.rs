use ggez::graphics::Canvas;
use ggez::input::keyboard::{self, KeyCode};
use ggez::Context;
use rand::{rngs::ThreadRng, Rng};
use std::time::{Duration, Instant};
use crate::common::GameStateHandler;
use crate::game::{Snake, Map, Direction};
use crate::common::StateResult;

use super::MenuState;


pub struct PairGame {
    snake1: Snake,
    snake2: Snake,
    map: Map,
    rng: ThreadRng,
    last_update_time: Instant,
    update_interval: Duration,
    game_over: bool,
	map_size: u32,
	cell_size: f32,
	step_time: Duration
}

// TODO: check for collisions between snakes
impl PairGame {
    pub fn new(map_size: u32, cell_size: f32, step_time: Duration) -> Self {
        Self {
            snake1: Snake::new_by_position(map_size, 
				cell_size/(step_time.as_millis() as f32), 
				step_time, 
				map_size/2 + 1,
				map_size-2,
				Direction::Up),
            snake2: Snake::new_by_position(map_size, 
				cell_size/(step_time.as_millis() as f32), 
				step_time, 
				map_size/2 + 1,
				3,
				Direction::Down),
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
		self.snake1 = Snake::new_by_position(self.map_size, 
			self.cell_size/(self.step_time.as_millis() as f32), 
			self.step_time, 
			self.map_size/2 + 1,
			self.map_size-2,
			Direction::Up);
        self.snake2 = Snake::new_by_position(self.map_size, 
			self.cell_size/(self.step_time.as_millis() as f32), 
			self.step_time, 
			self.map_size/2 + 1,
			3,
			Direction::Down);
		self.map = Map::new(self.map_size, self.cell_size);
		self.game_over = false;
	}
}

impl GameStateHandler for PairGame {
	fn update(&mut self, ctx: &mut Context) -> Result<StateResult, ggez::GameError> {
        if !self.snake1.next(&mut self.map) || !self.snake2.next(&mut self.map) {
			self.game_over = true;
			return Ok(StateResult::NextState(Box::new(MenuState::new(self.map_size, self.cell_size, self.step_time))));
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

    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> Result<StateResult, ggez::GameError> {
        // 绘制地图
		self.map.draw(ctx, canvas)?;

        // 绘制蛇
		self.snake1.draw(ctx, canvas, self.cell_size)?;
		self.snake2.draw(ctx, canvas, self.cell_size)?;

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
				},
				KeyCode::R => {
					self.restart();
				},
				KeyCode::W => self.snake1.trun(Direction::Up),
				KeyCode::S => self.snake1.trun(Direction::Down),
				KeyCode::A => self.snake1.trun(Direction::Left),
				KeyCode::D => self.snake1.trun(Direction::Right),
				KeyCode::Up => self.snake2.trun(Direction::Up),
				KeyCode::Down => self.snake2.trun(Direction::Down),
				KeyCode::Left => self.snake2.trun(Direction::Left),
				KeyCode::Right => self.snake2.trun(Direction::Right),
				_ => (),
			}
		}

		Ok(StateResult::Ok)
	}
}