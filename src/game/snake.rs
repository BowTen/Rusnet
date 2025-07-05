use crate::game::{Direction, Map, Segment};
use ggez::Context;
use ggez::GameResult;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Mesh, Rect};
use std::collections::LinkedList;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Snake {
    pub body: LinkedList<Segment>,
    pub last_tail: Segment,
    pub dir: Direction,
    pub next_dir: [Option<Direction>; 2],
    pub map_size: u32,
    pub cell_size: f32,
    pub last_step_time: Instant,
    pub speed: f32,
    pub step_time: Duration,
}

#[derive(PartialEq)]
pub enum MoveResult {
    Ok,
    Grow,
    Die,
}

impl Snake {
    pub fn new(map_size: u32, cell_size: f32, speed: f32, step_time: Duration) -> Self {
        Self {
            body: [
                (map_size / 2 + 1, map_size - 3).into(),
                (map_size / 2 + 1, map_size - 2).into(),
            ]
            .iter()
            .cloned()
            .collect(),
            last_tail: (map_size / 2 + 1, map_size - 1).into(),
            dir: Direction::Up,
            next_dir: [None, None],
            map_size,
            cell_size,
            last_step_time: Instant::now(),
            speed,
            step_time,
        }
    }

    pub fn get_body(&self) -> &LinkedList<Segment> {
        &self.body
    }

    pub fn get_head(&self) -> Segment {
        self.body.front().unwrap().clone()
    }

    pub fn new_by_position(
        map_size: u32,
        cell_size: f32,
        speed: f32,
        step_time: Duration,
        x: u32,
        y: u32,
        dir: Direction,
    ) -> Self {
        let (dx, dy) = dir.inverse().coor_offset();
        let (x2, y2) = (((x as i32) + dx) as u32, ((y as i32) + dy) as u32);
        let (x3, y3) = (((x2 as i32) + dx) as u32, ((y2 as i32) + dy) as u32);
        Self {
            body: [(x, y).into(), (x2, y2).into()].iter().cloned().collect(),
            last_tail: (x3, y3).into(),
            dir,
            next_dir: [None, None],
            map_size,
            cell_size,
            last_step_time: Instant::now(),
            speed,
            step_time,
        }
    }

    fn last_dir(&self) -> Direction {
        if self.next_dir[1] != None {
            self.next_dir[1].unwrap()
        } else if self.next_dir[0] != None {
            self.next_dir[0].unwrap()
        } else {
            self.dir
        }
    }

    pub fn just_truned(&self) -> bool {
        let third = if self.body.len() >= 3 {
            self.body.iter().nth(2).unwrap().clone()
        } else {
            self.last_tail.clone()
        };
        let head = self.get_head();
        head.x != third.x && head.y != third.y
    }

    // 返回值：操作是否有效
    pub fn trun(&mut self, dir: Direction) -> bool {
        if self.next_dir[1] != None || dir == self.last_dir() || dir == self.last_dir().inverse() {
            return false;
        }
        if self.next_dir[0] == None {
            self.next_dir[0] = Some(dir);
        } else {
            self.next_dir[1] = Some(dir);
        }
        true
    }

    pub fn back_then_trun(&mut self, dir: Direction) {
        assert!(!self.just_truned());
        self.back();
        self.trun(dir);
        self.next(false);
    }

    pub fn next_head(&mut self) -> Segment {
        let dir = self.next_dir[0].or_else(|| Some(self.last_dir())).unwrap();
        let (dx, dy) = dir.coor_offset();
        let mut head = self.get_head();
        head.x = ((head.x as i32) + dx) as u32;
        head.y = ((head.y as i32) + dy) as u32;
        head
    }
    pub fn last_head(&self) -> Segment {
        self.body.iter().nth(1).unwrap().clone()
    }

    pub fn next(&mut self, got: bool) -> MoveResult {
        if self.last_step_time.elapsed() < self.step_time {
            return MoveResult::Ok;
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
            Direction::Rest => (),
        }
        self.last_tail = self.body.back().unwrap().clone();

        if got {
            self.body.push_front((x, y).into());
        } else {
            self.body.pop_back().unwrap();
            self.body.push_front((x, y).into());
        }
        self.last_step_time = Instant::now();

        if x < 1
        || x > self.map_size
        || y < 1
        || y > self.map_size
        || self.body.iter().filter(|e| e.x == x && e.y == y).count() > 1
        {
            MoveResult::Die
        } else if got {
            MoveResult::Grow
        } else {
            MoveResult::Ok
        }
    }

    fn back(&mut self) {
        self.body.pop_front();
        self.body.push_back(self.last_tail);
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        for &Segment { x, y } in self.body.iter().skip(1) {
            let rect = Rect::new(
                x as f32 * self.cell_size,
                y as f32 * self.cell_size,
                self.cell_size,
                self.cell_size,
            );
            canvas.draw(
                &Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::GREEN)?,
                DrawParam::default(),
            );
        }
        //画移动的头部
        let &Segment { x, y } = self.body.iter().nth(1).unwrap();
        let u = self.body.iter().nth(1).unwrap();
        let v = self.body.iter().nth(0).unwrap();
        let (dx, dy) = Direction::new(u, v).shift(self.speed, self.last_step_time.elapsed());
        let rect = Rect::new(
            x as f32 * self.cell_size + dx,
            y as f32 * self.cell_size + dy,
            self.cell_size,
            self.cell_size,
        );
        canvas.draw(
            &Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::GREEN)?,
            DrawParam::default(),
        );
        //画移动的尾部
        let &Segment { x, y } = &self.last_tail;
        let u = &self.last_tail;
        let v = self.body.back().unwrap();
        let (dx, dy) = Direction::new(u, v).shift(self.speed, self.last_step_time.elapsed());
        let rect = Rect::new(
            x as f32 * self.cell_size + dx,
            y as f32 * self.cell_size + dy,
            self.cell_size,
            self.cell_size,
        );
        canvas.draw(
            &Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::GREEN)?,
            DrawParam::default(),
        );

        Ok(())
    }
}
