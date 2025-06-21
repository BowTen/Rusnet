use ggez::event::EventHandler;
use ggez::graphics::{Canvas, Color};
use ggez::input::keyboard;
use ggez::Context;
use ggez::GameResult;
use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;
use crate::common::{GameStateHandler, StateResult};
use crate::server::room_state::RoomState;
use crate::net::message::Message;


pub struct MainState {
	game_state: Box<dyn GameStateHandler>,
	map_size: u32,
	cell_size: f32,
	step_time: Duration,
}

impl MainState {
	pub fn new(ctx: &mut Context, game_state: Box<dyn GameStateHandler>, map_size: u32, cell_size: f32, step_time: Duration) -> Result<MainState, String> {
		Ok(MainState { 
			game_state,
			map_size, 
			cell_size, 
			step_time,
		})
	}
}

impl EventHandler for MainState {
	fn update(&mut self, ctx: &mut Context) -> GameResult {
		match self.game_state.update(ctx)? {
			StateResult::NextState(next_state) => self.game_state = next_state,
			StateResult::Ok => (),
			StateResult::ShutDown => ctx.request_quit(),
		}
		Ok(())
	}
	
	fn draw(&mut self, ctx: &mut Context) -> GameResult {
		let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
		match self.game_state.draw(ctx, &mut canvas)? {
			StateResult::NextState(next_state) => self.game_state = next_state,
			StateResult::Ok => (),
			StateResult::ShutDown => ctx.request_quit(),
		}
		canvas.finish(ctx)?;
		Ok(())
	}
	
	fn key_down_event(
		&mut self,
		ctx: &mut Context,
		input: keyboard::KeyInput,
		repeated: bool,
	) -> Result<(), ggez::GameError> {
		match self.game_state.key_down_event(ctx, input, repeated)? {
			StateResult::NextState(next_state) => self.game_state = next_state,
			StateResult::Ok => (),
			StateResult::ShutDown => ctx.request_quit(),
		}
		Ok(())
	}
}