use ggez::event::EventHandler;
use ggez::graphics::{self, Color};
use ggez::input::keyboard::{self, KeyCode};
use ggez::Context;
use ggez::GameResult;
use rand::{rngs::ThreadRng, Rng};
use std::time::{Duration, Instant};
use crate::game::{Snake, Map, Direction};

pub struct GameState {
    snake: Snake,
    map: Map,
    rng: ThreadRng,
    last_update_time: Instant,
    update_interval: Duration,
    game_over: bool,
	map_size: u32,
	cell_size: f32,
	step_time: Duration,
	turn_time: Duration
}

impl GameState {
    pub fn new(ctx: &mut Context, map_size: u32, cell_size: f32, step_time: Duration, turn_time: Duration) -> Self {
        Self {
            snake: Snake::new(map_size, cell_size/(step_time.as_millis() as f32), step_time, turn_time),
			map: Map::new(map_size),
			rng: rand::thread_rng(),
            last_update_time: Instant::now(),
            update_interval: Duration::from_millis(150),
            game_over: false,
			map_size,
			cell_size,
			step_time,
			turn_time
        }
    }

	fn restart(&mut self) {
		self.snake = Snake::new(self.map_size, self.cell_size/(self.step_time.as_millis() as f32), self.step_time, self.turn_time);
		self.map = Map::new(self.map_size);
		self.game_over = false;
	}
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.game_over {
            return Ok(());
        }
        if !self.snake.step(&mut self.map) {
            self.game_over = true;
        }

        // 每隔一段时间更新一次
        if self.last_update_time.elapsed() >= self.update_interval {

            // 生成新水果
            if self.rng.gen_range(0..100) < 5 {
                self.map.gen_fruit(&mut self.rng);
            }

            self.last_update_time = Instant::now();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        // 绘制边框
		self.map.draw_map(ctx, &mut canvas, self.cell_size, self.map_size)?;

        // 绘制水果
		self.map.draw_fruits(ctx, &mut canvas, self.cell_size)?;

        // 绘制蛇
		self.snake.draw(ctx, &mut canvas, self.cell_size)?;

        canvas.finish(ctx)?;
        Ok(())
    }

	fn key_down_event(
			&mut self,
			ctx: &mut Context,
			input: keyboard::KeyInput,
			_repeated: bool,
		) -> Result<(), ggez::GameError> {
		
		if let Some(key_code) = input.keycode {
			match key_code {
				KeyCode::Escape => {
					self.game_over = true;
				},
				KeyCode::R => {
					self.restart();
				},
				_ => {
					if let Some(dir) = Direction::from_keycode(key_code) {
						self.snake.trun(&mut self.map, dir);
					}
				}
			}
		}

		Ok(())
	}
}