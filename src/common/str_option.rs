use ggez::{graphics::{Color, DrawParam, Drawable, PxScale, Text}, input::keyboard, Context};
use crate::common::InputObject;
use super::{input_object::KeyDownCallBack, StateResult};

pub struct StrOption {
	text: Text,
	color: Color,
	key_down_cb: Option<Box<KeyDownCallBack>>,
}

impl StrOption {
	pub fn new(text: String) -> Self {
		StrOption { 
			text: Text::new(text),
			color: Color::WHITE,
			key_down_cb: None,
		}
	}

	pub fn set_scale(&mut self, x: f32) {
		self.text.set_scale(PxScale::from(x));
	}

	pub fn contents(&self) -> String {
		self.text.contents()
	}

	pub fn set_key_down_cb(&mut self, cb: Box<KeyDownCallBack>) {
		self.key_down_cb = Some(cb);
	}
}

impl Drawable for StrOption {
	fn draw(&self, canvas: &mut ggez::graphics::Canvas, param: impl Into<ggez::graphics::DrawParam>) {
		let param: DrawParam = param.into();
		canvas.draw(&self.text, param.color(self.color));
	}

	fn dimensions(&self, gfx: &impl ggez::context::Has<ggez::graphics::GraphicsContext>) -> Option<ggez::graphics::Rect> {
		None
	}
}

impl InputObject for StrOption {
	fn focus(&mut self) {
		self.color = Color::YELLOW;
	}	

	fn unfocus(&mut self) {
		self.color = Color::WHITE;
	}

	fn key_down_event(
			&mut self,
			ctx: &mut Context,
			input: ggez::input::keyboard::KeyInput,
			repeated: bool,
			) -> Result<super::StateResult, ggez::GameError> {
		if let Some(cb) = &self.key_down_cb {
			return cb(ctx, input, repeated);
		}
		Ok(super::StateResult::Ok)
	}

	fn draw(&self, canvas: &mut ggez::graphics::Canvas, param: DrawParam) {
		Drawable::draw(self, canvas, param);
	}

	fn dimensions(&self, gfx: &Context) -> Option<ggez::graphics::Rect> {
		Drawable::dimensions(self, gfx)
	}

	fn as_any(&self) -> &dyn std::any::Any {
		self
	}

	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
}