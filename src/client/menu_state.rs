use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use ggez::event::EventHandler;
use ggez::graphics::{self, Canvas, Color, Text};
use ggez::input::keyboard::{self, KeyCode};
use ggez::Context;
use crate::common::StateResult;
use crate::common::*;
use super::room_menu::RoomMenu;
use super::{ClassicGame, MainState};

pub struct MenuState {
	selected: usize,
	options: Vec<String>,
	map_size: u32,
	cell_size: f32,
	step_time: Duration,
}

impl MenuState {
	pub fn new(map_size: u32, cell_size: f32, step_time: Duration) -> Self {
		MenuState{
			selected: 0,
			options: vec![
				"Classic Mode".to_string(), 
				"Online Mode".to_string(),
				"Exit".to_string(),
			],
			map_size,
			cell_size,
			step_time
		}
	}
}

impl GameStateHandler for MenuState {

	fn draw(&mut self, _ctx: &mut Context, canvas: &mut Canvas) -> Result<StateResult, ggez::GameError> {
		// 绘制标题
		let title = Text::new("Rusnet\nRust Snake Net!");
		canvas.draw(
			&title, 
			ggez::graphics::DrawParam::default()
			.dest([100f32, 100f32])
			.color(Color::GREEN)
		);

		// 绘制选项
		for (i, option) in self.options.iter().enumerate() {
			let color = if i == self.selected as usize { Color::YELLOW } else { Color::WHITE };
			let text = Text::new(option);
			canvas.draw(
				&text, 
				ggez::graphics::DrawParam::default()
				.dest([100.0, 200.0 + i as f32 * 30.0])
				.color(color)
			);
		}

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
				KeyCode::Up => {
					self.selected += self.options.len()-1;
					self.selected %= self.options.len();
				},
				KeyCode::Down => {
					self.selected += 1;
					self.selected %= self.options.len();
				},
				KeyCode::Return => {
					match self.selected {
						i if i < self.options.len() => {
							match &self.options[i][0..] {
								"Classic Mode" => return Ok(StateResult::NextState(Box::new(ClassicGame::new(self.map_size, self.cell_size, self.step_time)))),
								"Online Mode" => return Ok(StateResult::NextState(Box::new(RoomMenu::new(ctx, self.map_size, self.cell_size, self.step_time)))),
								"Exit" => { ctx.request_quit(); return Ok(StateResult::Ok); }
								_ => panic!("invalid option")
							}
						}
						_ => panic!("invalid option")
					};
				}
				_ => ()
			}
		}

		Ok(StateResult::Ok)
	}
}