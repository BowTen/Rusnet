use std::time::Duration;
use rusnet::game::GameState;

const CELL_SIZE: f32 = 35.0; // 每个格子大小
const MAP_SIZE: u32 = 35;    // 地图大小（30x30 格子）
const STEP_TIME: Duration = Duration::from_millis(180);

fn main() {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("snake_game", "Your Name")
        .window_setup(ggez::conf::WindowSetup::default().title("贪吃蛇"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(CELL_SIZE * MAP_SIZE as f32, CELL_SIZE * MAP_SIZE as f32))
        .build()
        .expect("无法创建上下文");

    let game_state = GameState::new(&mut ctx, MAP_SIZE, CELL_SIZE, STEP_TIME);
    ggez::event::run(ctx, event_loop, game_state);
}