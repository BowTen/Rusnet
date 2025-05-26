use std::time::Duration;
use ggez::input::keyboard::KeyCode;
use crate::game::Segment;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
	Rest,
}

impl Direction {
	pub fn new(u: &Segment, v: &Segment) -> Self {
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

	pub fn inverse(&self) -> Self {
		match self {
			Direction::Up => Direction::Down,
			Direction::Down => Direction::Up,
			Direction::Left => Direction::Right,
			Direction::Right => Direction::Left,
			Direction::Rest => Direction::Rest
		}
	}
	
	pub fn from_keycode(key_code: KeyCode) -> Option<Self> {
		match key_code {
			KeyCode::Up => Some(Direction::Up),
			KeyCode::Down => Some(Direction::Down),
			KeyCode::Left => Some(Direction::Left),
			KeyCode::Right => Some(Direction::Right),
			_ => None
		}
	}

	pub fn shift(&self, speed: f32, duration: Duration) -> (f32, f32) {
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