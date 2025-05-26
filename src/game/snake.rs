use std::{collections::LinkedList};
use std::time::{Duration, Instant};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect};
use ggez::Context;
use ggez::GameResult;
use crate::game::{Map, Segment, Direction};



pub struct Snake {
    body: LinkedList<Segment>,
	last_tail: Segment,
    dir: Direction,
	next_dir: [Option<Direction>; 2],
	n: u32,
	last_step_time: Instant,
	speed: f32,
	step_time: Duration
}

impl Snake {
    pub fn new(n: u32, speed: f32, step_time: Duration) -> Self {
		Self {
            body: [(n / 2, n - 4).into(), (n / 2, n - 3).into()].iter().cloned().collect(),
			last_tail: (n / 2, n - 2).into(),
            dir: Direction::Up,
			next_dir: [None, None],
			n,
			last_step_time: Instant::now(),
			speed,
			step_time
        }
    }

	fn last_dir(&self) -> Direction {
		if self.next_dir[1] != None {
			self.next_dir[1].unwrap()
		}else if self.next_dir[0] != None {
			self.next_dir[0].unwrap()
		}else {
			self.dir
		}
	}

    pub fn trun(&mut self, dir: Direction) {
		if self.next_dir[1] != None || dir == self.last_dir() || dir == self.last_dir().inverse() {
			return;
		}
		if self.next_dir[0] == None {
			self.next_dir[0] = Some(dir);
		}else {
			self.next_dir[1] = Some(dir);
		}
	}

    pub fn next(&mut self, map: &mut Map) -> bool {
		if self.last_step_time.elapsed() < self.step_time {
			return true;
		}
		if self.next_dir[0] != None {
			self.dir = self.next_dir[0].unwrap();
			self.next_dir[0] = self.next_dir[1];
			self.next_dir[1] = None;
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