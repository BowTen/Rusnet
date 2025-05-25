use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Canvas, Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::input::keyboard::{self, KeyCode};
use ggez::Context;
use ggez::GameResult;
use rand::{rngs::ThreadRng, Rng};
use std::collections::LinkedList;
use std::thread;
use std::time::{Duration, Instant};

const CELL_SIZE: f32 = 35.0; // 每个格子大小
const MAP_SIZE: u32 = 35;    // 地图大小（30x30 格子）
const STEP_TIME: Duration = Duration::from_millis(180);
const TRUN_TIME: Duration = Duration::from_millis(90);

#[derive(Clone, Copy, PartialEq)]
struct Segment{
	x: u32,
	y: u32
}

impl From<(u32, u32)> for Segment {
	fn from(value: (u32, u32)) -> Self {
		Segment{
			x: value.0,
			y: value.1
		}
	}
}
impl From<Segment> for (u32, u32) {
	fn from(value: Segment) -> Self {
		(value.x, value.y)
	}
}

struct Snake {
    body: LinkedList<Segment>,
	last_tail: Segment,
    dir: Direction,
	n: u32,
	last_step_time: Instant,
	speed: f32
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
	Rest,
}

impl Direction {
	fn new(u: &Segment, v: &Segment) -> Self {
		if v.y < u.y {
			Direction::Up
		}else if v.y > u.y {
			Direction::Down
		}else if v.x < u.x {
			Direction::Left
		}else if v.x > u.x {
			Direction::Right
		}else {
			Direction::Rest
		}
	}

	fn inverse(&self) -> Self {
		match self {
			Direction::Up => Direction::Down,
			Direction::Down => Direction::Up,
			Direction::Left => Direction::Right,
			Direction::Right => Direction::Left,
			Direction::Rest => Direction::Rest
		}
	}
	
	fn from_keycode(key_code: KeyCode) -> Option<Self> {
		match key_code {
			KeyCode::Up => Some(Direction::Up),
			KeyCode::Down => Some(Direction::Down),
			KeyCode::Left => Some(Direction::Left),
			KeyCode::Right => Some(Direction::Right),
			_ => None
		}
	}

	fn shift(&self, speed: f32, duration: Duration) -> (f32, f32) {
		let mut offset = (0f32, 0f32);
		let delta = speed * (duration.as_millis() as f32);
		match self {
			Direction::Up => offset.1 -= delta,
			Direction::Down => offset.1 += delta,
			Direction::Left => offset.0 -= delta,
			Direction::Right => offset.0 += delta,
			Direction::Rest => ()
		}
		offset
	}
}

struct Map {
    fruits: Vec<Vec<bool>>,
    n: u32,
}

struct GameState {
    snake: Snake,
    map: Map,
    rng: ThreadRng,
    last_update_time: Instant,
    update_interval: Duration,
    game_over: bool,
}

impl Snake {
    fn new(n: u32) -> Self {
        Self {
            body: [(n / 2, n - 4).into(), (n / 2, n - 3).into()].iter().cloned().collect(),
			last_tail: (n / 2, n - 2).into(),
            dir: Direction::Up,
			n,
			last_step_time: Instant::now(),
			speed: CELL_SIZE / (STEP_TIME.as_millis() as f32)
        }
    }

    fn step(&mut self, map: &mut Map) -> bool {
		self.next(map, STEP_TIME)
	}

    fn trun(&mut self, map: &mut Map, dir: Direction) {
		if dir == self.dir || dir == self.dir.inverse() {
			return;
		}
		self.dir = dir;
		if let Some(sleep_time) = TRUN_TIME.checked_sub(self.last_step_time.elapsed()) {
			thread::sleep(sleep_time);
		}
		self.next(map, TRUN_TIME);
    }

    fn next(&mut self, map: &mut Map, duration: Duration) -> bool {
		if self.last_step_time.elapsed() < duration {
			return true;
		}
        let (mut x, mut y) = self.body.front().unwrap().clone().into();
        match self.dir {
            Direction::Up => y -= 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
			Direction::Rest => ()
        }
		self.last_tail = self.body.back().unwrap().clone();

        if x <= 0 || x >= self.n-1 || y <= 0 || y >= self.n-1 || self.body.contains(&(x, y).into()) {
			self.last_step_time = Instant::now();
            return false;
        }
		
        let got = map.eat(x as usize, y as usize);
        if got {
			self.body.push_front((x, y).into());
        } else {
			self.body.pop_back().unwrap();
            self.body.push_front((x, y).into());
        }
		
		self.last_step_time = Instant::now();
        true
    }

	fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        for &Segment{x, y} in self.body.iter().skip(1) {
            let rect = Rect::new(x as f32 * CELL_SIZE, y as f32 * CELL_SIZE, CELL_SIZE, CELL_SIZE);
            canvas.draw(
                &Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::GREEN)?,
                DrawParam::default(),
            );
        }
		//画移动的头部
		let &Segment{x, y} = self.body.iter().nth(1).unwrap();
		let u = self.body.iter().nth(1).unwrap();
		let v = self.body.iter().nth(0).unwrap();
		let (dx, dy) = Direction::new(u, v).shift(self.speed, self.last_step_time.elapsed());
		let rect = Rect::new(x as f32 * CELL_SIZE + dx, y as f32 * CELL_SIZE + dy, CELL_SIZE, CELL_SIZE);
		canvas.draw(
			&Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::GREEN)?,
			DrawParam::default()
		);
		//画移动的尾部
		let &Segment{x, y} = &self.last_tail;
		let u = &self.last_tail;
		let v = self.body.back().unwrap();
		let (dx, dy) = Direction::new(u, v).shift(self.speed, self.last_step_time.elapsed());
		let rect = Rect::new(x as f32 * CELL_SIZE + dx, y as f32 * CELL_SIZE + dy, CELL_SIZE, CELL_SIZE);
		canvas.draw(
			&Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::GREEN)?,
			DrawParam::default()
		);


		Ok(())
	}
}

impl Map {
    fn new(n: u32) -> Self {
        Self {
            fruits: vec![vec![false; n as usize]; n as usize],
            n,
        }
    }

    fn gen_fruit(&mut self, rng: &mut ThreadRng) {
        let x = rng.gen_range(1..=self.n - 2);
        let y = rng.gen_range(1..=self.n - 2);
        self.fruits[x as usize][y as usize] = true;
    }

    fn eat(&mut self, x: usize, y: usize) -> bool {
        let ret = self.fruits[x][y];
        self.fruits[x][y] = false;
        ret
    }
}

impl GameState {
    fn new(ctx: &mut Context) -> Self {
        Self {
            snake: Snake::new(MAP_SIZE),
            map: Map::new(MAP_SIZE),
            rng: rand::thread_rng(),
            last_update_time: Instant::now(),
            update_interval: Duration::from_millis(150),
            game_over: false,
        }
    }

	fn restart(&mut self) {
		self.snake = Snake::new(MAP_SIZE);
		self.map = Map::new(MAP_SIZE);
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
        let border_rect = Rect::new(0.0, 0.0, CELL_SIZE * MAP_SIZE as f32, CELL_SIZE);
        canvas.draw(
            &Mesh::new_rectangle(ctx, DrawMode::fill(), border_rect, Color::WHITE)?,
            DrawParam::default(),
        );
        let border_rect = Rect::new(0.0, CELL_SIZE * (MAP_SIZE-1) as f32, CELL_SIZE * MAP_SIZE as f32, CELL_SIZE);
        canvas.draw(
            &Mesh::new_rectangle(ctx, DrawMode::fill(), border_rect, Color::WHITE)?,
            DrawParam::default(),
        );
        let border_rect = Rect::new(0.0, 0.0, CELL_SIZE, CELL_SIZE * MAP_SIZE as f32);
        canvas.draw(
            &Mesh::new_rectangle(ctx, DrawMode::fill(), border_rect, Color::WHITE)?,
            DrawParam::default(),
        );
        let border_rect = Rect::new(CELL_SIZE * (MAP_SIZE-1) as f32, 0.0, CELL_SIZE, CELL_SIZE * MAP_SIZE as f32);
        canvas.draw(
            &Mesh::new_rectangle(ctx, DrawMode::fill(), border_rect, Color::WHITE)?,
            DrawParam::default(),
        );

        // 绘制水果
        for (i, row) in self.map.fruits.iter().enumerate() {
            for (j, &has_fruit) in row.iter().enumerate() {
                if has_fruit {
                    let rect = Rect::new(i as f32 * CELL_SIZE, j as f32 * CELL_SIZE, CELL_SIZE, CELL_SIZE);
                    canvas.draw(
                        &Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::RED)?,
                        DrawParam::default(),
                    );
                }
            }
        }

        // 绘制蛇
		self.snake.draw(ctx, &mut canvas)?;

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

fn main() {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("snake_game", "Your Name")
        .window_setup(ggez::conf::WindowSetup::default().title("贪吃蛇"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(CELL_SIZE * MAP_SIZE as f32, CELL_SIZE * MAP_SIZE as f32))
        .build()
        .expect("无法创建上下文");

    let game_state = GameState::new(&mut ctx);
    event::run(ctx, event_loop, game_state);
}