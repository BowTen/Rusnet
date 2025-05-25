use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::input::keyboard::{self, KeyCode};
use ggez::Context;
use ggez::GameResult;
use rand::{rngs::ThreadRng, Rng};
use std::collections::LinkedList;
use std::time::{Duration, Instant};

const CELL_SIZE: f32 = 20.0; // 每个格子大小
const MAP_SIZE: u32 = 50;    // 地图大小（30x30 格子）
const STEP_TIME: Duration = Duration::from_millis(120);
const TRUN_TIME: Duration = Duration::from_millis(60);

struct Snake {
    body: LinkedList<(u32, u32)>,
    dir: Direction,
	n: u32,
	last_step_time: Instant
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
	fn inverse(&self) -> Self {
		match self {
			Direction::Up => Direction::Down,
			Direction::Down => Direction::Up,
			Direction::Left => Direction::Right,
			Direction::Right => Direction::Left
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
            body: [(n / 2, n - 3), (n / 2, n - 2)].iter().cloned().collect(),
            dir: Direction::Up,
			n,
			last_step_time: Instant::now()
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
		self.next(map, TRUN_TIME);
    }

    fn next(&mut self, map: &mut Map, duration: Duration) -> bool {
		if self.last_step_time.elapsed() < duration {
			return true;
		}
        let (mut x, mut y) = self.body.front().unwrap().clone();
        match self.dir {
            Direction::Up => y -= 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }

        if x <= 0 || x >= self.n-1 || y <= 0 || y >= self.n-1 || self.body.contains(&(x, y)) {
			self.last_step_time = Instant::now();
            return false;
        }
		
        let got = map.eat(x as usize, y as usize);
        if got {
			self.body.push_front((x, y));
        } else {
			self.body.pop_back().unwrap();
            self.body.push_front((x, y));
        }
		
		self.last_step_time = Instant::now();
        true
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

        // 绘制蛇
        for &(x, y) in &self.snake.body {
            let rect = Rect::new(x as f32 * CELL_SIZE, y as f32 * CELL_SIZE, CELL_SIZE, CELL_SIZE);
            canvas.draw(
                &Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::GREEN)?,
                DrawParam::default(),
            );
        }

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

        canvas.finish(ctx)?;
        Ok(())
    }

	fn key_down_event(
			&mut self,
			ctx: &mut Context,
			input: keyboard::KeyInput,
			_repeated: bool,
		) -> Result<(), ggez::GameError> {
		
		if let Some(dir) = input.keycode.and_then(Direction::from_keycode) {
			self.snake.trun(&mut self.map, dir);
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