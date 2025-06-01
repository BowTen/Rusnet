use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;
use ggez::event::EventHandler;
use ggez::graphics::{self, Canvas, Color, Text};
use ggez::input::keyboard::{self, KeyCode};
use ggez::Context;
use crate::common::{str_option, GameStateHandler, InputBox, InputObject, StateResult, StrOption};
use super::{ClassicGame, MainState, MenuState};

pub struct RoomMenu {
	selected_mode: usize,
	selected_box: usize,
	mode_options: Vec<StrOption>,
	input_boxes: Vec<Box<dyn InputObject>>,
	map_size: u32,
	cell_size: f32,
	step_time: Duration,
}

impl RoomMenu {
	pub fn new(ctx: &Context, map_size: u32, cell_size: f32, step_time: Duration) -> Self {
		let mut room_menu = RoomMenu{
			selected_mode: 0,
			selected_box: 0,
			mode_options: vec![
				StrOption::new("Create a room".to_string()),
				StrOption::new("Join a room".to_string()),
			],
			input_boxes: vec![
				Box::new(InputBox::new(ctx, "Server Address".to_string())),
				Box::new(InputBox::new(ctx, "Server PassWord".to_string())),
			],
			map_size,
			cell_size,
			step_time,
		};
		for str_option in &mut room_menu.mode_options {
			str_option.set_scale(25.0);
		}
		room_menu.mode_options[room_menu.selected_mode].focus();
		room_menu.input_boxes[room_menu.selected_box].focus();

		room_menu
	}
}

impl GameStateHandler for RoomMenu {

	fn draw(&mut self, _ctx: &mut Context, canvas: &mut Canvas) -> Result<StateResult, ggez::GameError> {
		// draw mode options
		for (i, mode) in self.mode_options.iter().enumerate() {
			let color = if i == self.selected_mode as usize { Color::YELLOW } else { Color::WHITE };
			canvas.draw(
				mode, 
				ggez::graphics::DrawParam::default()
				.dest([100.0 + i as f32 * 250.0, 100.0])
				.color(color)
			);
		}
		// draw input options
		for (i, input) in self.input_boxes.iter().enumerate() {
			let color = if i == self.selected_box as usize { Color::YELLOW } else { Color::WHITE };
			input.draw(
				canvas,
				ggez::graphics::DrawParam::default()
				.dest([100.0, 100.0 + (i+1) as f32 * 100.0])
				.color(color)
			);
		}

		Ok(StateResult::Ok)
	}

	fn key_down_event(
		&mut self,
		ctx: &mut Context,
		input: keyboard::KeyInput,
		repeated: bool,
		) -> Result<StateResult, ggez::GameError> {
	
		if let Some(key_code) = &input.keycode {
			self.input_boxes[self.selected_box].unfocus();
			self.mode_options[self.selected_mode].unfocus();
			match key_code {
				KeyCode::Up => {
					self.selected_box += self.input_boxes.len()-1;
					self.selected_box %= self.input_boxes.len();
				},
				KeyCode::Down => {
					self.selected_box += 1;
					self.selected_box %= self.input_boxes.len();
				},
				KeyCode::Left => {
					self.selected_mode += self.mode_options.len()-1;
					self.selected_mode %= self.mode_options.len();
				},
				KeyCode::Right => {
					self.selected_mode += 1;
					self.selected_mode %= self.mode_options.len();
				},
				KeyCode::Escape => return Ok(StateResult::NextState(Box::new(MenuState::new(self.map_size, self.cell_size, self.step_time)))),
				_ => {
					
				}
			}
			self.input_boxes[self.selected_box].focus();
			self.mode_options[self.selected_mode].focus();

		}
		match self.input_boxes[self.selected_box].key_down_event(ctx, input, repeated)? {
			StateResult::NextState(next_state) => Ok(StateResult::NextState(next_state)),
			StateResult::Ok => Ok(StateResult::Ok)
		}
	}
}