use ggez::{graphics::{Color, Drawable}, input::keyboard::KeyCode};

use crate::common::{InputObject, StateResult};


pub struct Button{
	callback: Box<dyn Fn ()-> StateResult>,
	color: Color,
}

impl Button {
	pub fn new(cb: Box<dyn Fn ()-> StateResult>) -> Self {
		Button { 
			callback: cb,
			color: Color::WHITE,
		}
	}
}

impl Drawable for Button {
	fn draw(&self, canvas: &mut ggez::graphics::Canvas, param: impl Into<ggez::graphics::DrawParam>) {
		
	}

	fn dimensions(&self, gfx: &impl ggez::context::Has<ggez::graphics::GraphicsContext>) -> Option<ggez::graphics::Rect> {
		None
	}
}

impl InputObject for Button {
	fn focus(&mut self) {
		self.color = Color::YELLOW
	}

	fn unfocus(&mut self) {
		self.color = Color::WHITE
	}

	fn key_down_event(
			&mut self,
			ctx: &mut ggez::Context,
			input: ggez::input::keyboard::KeyInput,
			repeated: bool,
			) -> Result<StateResult, ggez::GameError> {
		if let Some(key_code) = input.keycode {
			if key_code == KeyCode::Return {
				return Ok((*self.callback)());
			}
		}
		Ok(StateResult::Ok)
	}

	fn draw(&self, canvas: &mut ggez::graphics::Canvas, param: ggez::graphics::DrawParam) {
		Drawable::draw(self, canvas, param);
	}

	fn dimensions(&self, gfx: &ggez::Context) -> Option<ggez::graphics::Rect> {
		Drawable::dimensions(self, gfx)
	}
	
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}

	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
}