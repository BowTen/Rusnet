use ggez::event::EventHandler;
use ggez::graphics::{Canvas, Color};
use ggez::input::keyboard;
use ggez::Context;
use ggez::GameResult;
use std::time::Duration;
use crate::game::state::{Game, Menu};

pub enum StateResult {
	StartGame,
	Exit,
	GameOver,
	Ok
}

pub struct GameState {
	game_state: GameStateData,
	map_size: u32,
	cell_size: f32,
	step_time: Duration
}

impl GameState {
	pub fn new(ctx: &mut Context, map_size: u32, cell_size: f32, step_time: Duration) -> Self {
		GameState { 
			game_state: GameStateData::Menu(Menu::new()), 
			map_size, 
			cell_size, 
			step_time 
		}
	}
}

impl EventHandler for GameState {
	fn update(&mut self, _ctx: &mut Context) -> GameResult {
		match &mut self.game_state {
			GameStateData::Menu(menu) => (),
			GameStateData::Game(game) => {
				if let StateResult::GameOver = game.update(_ctx)? {
					self.game_state = GameStateData::Menu(Menu::new());
				}
			}
		}
		Ok(())
	}
	
	fn draw(&mut self, _ctx: &mut Context) -> GameResult {
		let mut canvas = Canvas::from_frame(_ctx, Color::BLACK);
		match &mut self.game_state {
			GameStateData::Menu(menu) => menu.draw(_ctx, &mut canvas)?,
			GameStateData::Game(game) => game.draw(_ctx, &mut canvas)?
		}
		canvas.finish(_ctx)?;
		Ok(())
	}

	fn key_down_event(
			&mut self,
			ctx: &mut Context,
			input: keyboard::KeyInput,
			_repeated: bool,
		) -> Result<(), ggez::GameError> {
		match &mut self.game_state {
			GameStateData::Menu(menu) => {
				match menu.key_down_event(ctx, input, _repeated)? {
					StateResult::StartGame => self.game_state = GameStateData::Game(Game::new(ctx, self.map_size, self.cell_size, self.step_time)),
					StateResult::Exit => ctx.request_quit(),
					StateResult::Ok => (),
					_ => panic!("unexpected result")
				};
			},
			GameStateData::Game(game) => {
				game.key_down_event(ctx, input, _repeated)?;
			}
		}

		Ok(())
	}
}

enum GameStateData {
	Menu(Menu),
	Game(Game)
}