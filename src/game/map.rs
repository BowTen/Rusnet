use ggez::{Context, GameResult};
use ggez::graphics::{Color, DrawMode, DrawParam, Mesh, Rect, Canvas};
use rand::{rngs::ThreadRng, Rng};

pub struct Map {
    pub fruits: Vec<Vec<bool>>,
    n: u32,
}

impl Map {
    pub fn new(n: u32) -> Self {
        Self {
            fruits: vec![vec![false; n as usize]; n as usize],
            n,
        }
    }

    pub fn gen_fruit(&mut self, rng: &mut ThreadRng) {
        let x = rng.gen_range(1..=self.n - 2);
        let y = rng.gen_range(1..=self.n - 2);
        self.fruits[x as usize][y as usize] = true;
    }

    pub fn eat(&mut self, x: usize, y: usize) -> bool {
        let ret = self.fruits[x][y];
        self.fruits[x][y] = false;
        ret
    }

	pub fn draw_map(&self, ctx: &mut Context, canvas: &mut Canvas, cell_size: f32, map_size: u32) -> GameResult {
        let border_rect = Rect::new(0.0, 0.0, cell_size * map_size as f32, cell_size);
        canvas.draw(
            &Mesh::new_rectangle(ctx, DrawMode::fill(), border_rect, Color::WHITE)?,
            DrawParam::default(),
        );
        let border_rect = Rect::new(0.0, cell_size * (map_size-1) as f32, cell_size * map_size as f32, cell_size);
        canvas.draw(
            &Mesh::new_rectangle(ctx, DrawMode::fill(), border_rect, Color::WHITE)?,
            DrawParam::default(),
        );
        let border_rect = Rect::new(0.0, 0.0, cell_size, cell_size * map_size as f32);
        canvas.draw(
            &Mesh::new_rectangle(ctx, DrawMode::fill(), border_rect, Color::WHITE)?,
            DrawParam::default(),
        );
        let border_rect = Rect::new(cell_size * (map_size-1) as f32, 0.0, cell_size, cell_size * map_size as f32);
        canvas.draw(
            &Mesh::new_rectangle(ctx, DrawMode::fill(), border_rect, Color::WHITE)?,
            DrawParam::default(),
        );
		Ok(())
	}

	pub fn draw_fruits(&self, ctx: &mut Context, canvas: &mut Canvas, cell_size: f32) -> GameResult {
		for (i, row) in self.fruits.iter().enumerate() {
            for (j, &has_fruit) in row.iter().enumerate() {
                if has_fruit {
                    let rect = Rect::new(i as f32 * cell_size, j as f32 * cell_size, cell_size, cell_size);
                    canvas.draw(
                        &Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::RED)?,
                        DrawParam::default(),
                    );
                }
            }
        }
		Ok(())
	}
}
