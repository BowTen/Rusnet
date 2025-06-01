use ggez::Context;
use ggez::input::keyboard;
use ggez::graphics::Canvas;

pub enum StateResult {
	NextState(Box<dyn GameStateHandler>),
	Ok,
}

pub trait GameStateHandler {
	fn update(&mut self, ctx: &mut Context) -> Result<StateResult, ggez::GameError> {
		Ok(StateResult::Ok)
	}
	
	fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> Result<StateResult, ggez::GameError>;

	fn key_down_event(
			&mut self,
			ctx: &mut Context,
			input: keyboard::KeyInput,
			repeated: bool,
		) -> Result<StateResult, ggez::GameError>;
}