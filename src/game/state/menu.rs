use ggez::graphics::{self, Canvas, Color, Text};
use ggez::input::keyboard::{self, KeyCode};
use ggez::Context;
use crate::game::state::StateResult;

pub struct Menu {
	selected: usize,
	options: Vec<String>
}

impl Menu {
	pub fn new() -> Self {
		Menu{
			selected: 0,
			options: vec!["Start Game".to_string(), "Exit".to_string()]
		}
	}

	pub fn draw(&mut self, _ctx: &mut Context, canvas: &mut Canvas) -> Result<(), ggez::GameError> {
		// 绘制标题
		let title = Text::new("Rusnet\nRust Snake Net!");
		canvas.draw(&title, 
			ggez::graphics::DrawParam::default()
			.dest([100f32, 100f32])
			.color(Color::GREEN)
		);

		// 绘制选项
		for (i, option) in self.options.iter().enumerate() {
			let color = if i == self.selected as usize { Color::YELLOW } else { Color::WHITE };
			let text = Text::new(option);
			canvas.draw(&text, 
				ggez::graphics::DrawParam::default()
				.dest([100.0, 200.0 + i as f32 * 30.0])
				.color(color)
			);
		}

		Ok(())
	}

	pub fn key_down_event(
		&mut self,
		ctx: &mut Context,
		input: keyboard::KeyInput,
		_repeated: bool,
		) -> Result<StateResult, ggez::GameError> {
	
		if let Some(key_code) = input.keycode {
			match key_code {
				KeyCode::Up => {
					self.selected += 1;
					self.selected %= self.options.len();
				},
				KeyCode::Down => {
					self.selected += self.options.len()-1;
					self.selected %= self.options.len();
				},
				KeyCode::Return => {
					match self.selected {
						0 => return Ok(StateResult::StartGame),
						1 => return Ok(StateResult::Exit),
						_ => panic!("invalid option")
					};
				}
				_ => ()
			}
		}

		Ok(StateResult::Ok)
	}
}