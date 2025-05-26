use std::{collections::LinkedList};
use std::thread;
use std::time::{Duration, Instant};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::Context;
use ggez::GameResult;
use crate::game::{Map, Segment, Direction};



pub struct Snake {
    body: LinkedList<Segment>,
	last_tail: Segment,
    dir: Direction,
	n: u32,
	last_step_time: Instant,
	speed: f32,
	step_time: Duration,
	turn_time: Duration
}

impl Snake {
    pub fn new(n: u32, speed: f32, step_time: Duration, turn_time: Duration) -> Self {
        Self {
            body: [(n / 2, n - 4).into(), (n / 2, n - 3).into()].iter().cloned().collect(),
			last_tail: (n / 2, n - 2).into(),
            dir: Direction::Up,
			n,
			last_step_time: Instant::now(),
			speed,
			step_time,
			turn_time
        }
    }

    pub fn step(&mut self, map: &mut Map) -> bool {
		self.next(map, self.step_time)
	}

    pub fn trun(&mut self, map: &mut Map, dir: Direction) {
		if dir == self.dir || dir == self.dir.inverse() {
			return;
		}
		self.dir = dir;
		if let Some(sleep_time) = self.turn_time.checked_sub(self.last_step_time.elapsed()) {
			thread::sleep(sleep_time);
		}
		self.next(map, self.turn_time);
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

	pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas, cell_size: f32) -> GameResult {
        for &Segment{x, y} in self.body.iter().skip(1) {
            let rect = Rect::new(x as f32 * cell_size, y as f32 * cell_size, cell_size, cell_size);
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
		let rect = Rect::new(x as f32 * cell_size + dx, y as f32 * cell_size + dy, cell_size, cell_size);
		canvas.draw(
			&Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::GREEN)?,
			DrawParam::default()
		);
		//画移动的尾部
		let &Segment{x, y} = &self.last_tail;
		let u = &self.last_tail;
		let v = self.body.back().unwrap();
		let (dx, dy) = Direction::new(u, v).shift(self.speed, self.last_step_time.elapsed());
		let rect = Rect::new(x as f32 * cell_size + dx, y as f32 * cell_size + dy, cell_size, cell_size);
		canvas.draw(
			&Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::GREEN)?,
			DrawParam::default()
		);

		Ok(())
	}
}