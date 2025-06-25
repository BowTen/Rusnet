use ggez::{Context, GameResult};
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam, Drawable, Mesh, Rect};
use rand::{rngs::ThreadRng, Rng};

pub struct Map {
    pub fruits: Vec<Vec<bool>>,
    map_size: u32,
    cell_size: f32,
}

impl Map {
    pub fn new(map_size: u32, cell_size: f32) -> Self {
        Self {
            fruits: vec![vec![false; (map_size+1) as usize]; (map_size+1) as usize],
            map_size,
            cell_size,
        }
    }

    pub fn gen_fruit(&mut self, rng: &mut ThreadRng) {
        let x = rng.gen_range(1..=self.map_size);
        let y = rng.gen_range(1..=self.map_size);
        self.fruits[x as usize][y as usize] = true;
    }

    pub fn eat(&mut self, x: usize, y: usize) -> bool {
        let ret = self.fruits[x][y];
        self.fruits[x][y] = false;
        ret
    }

	pub fn draw_map(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let border_rect = Rect::new(0.0, 0.0, self.cell_size * (self.map_size + 2) as f32, self.cell_size);
        canvas.draw(
            &Mesh::new_rectangle(ctx, DrawMode::fill(), border_rect, Color::WHITE)?,
            DrawParam::default(),
        );
        let border_rect = Rect::new(0.0, self.cell_size * ((self.map_size + 2)-1) as f32, self.cell_size * (self.map_size + 2) as f32, self.cell_size);
        canvas.draw(
            &Mesh::new_rectangle(ctx, DrawMode::fill(), border_rect, Color::WHITE)?,
            DrawParam::default(),
        );
        let border_rect = Rect::new(0.0, 0.0, self.cell_size, self.cell_size * (self.map_size + 2) as f32);
        canvas.draw(
            &Mesh::new_rectangle(ctx, DrawMode::fill(), border_rect, Color::WHITE)?,
            DrawParam::default(),
        );
        let border_rect = Rect::new(self.cell_size * ((self.map_size + 2)-1) as f32, 0.0, self.cell_size, self.cell_size * (self.map_size + 2) as f32);
        canvas.draw(
            &Mesh::new_rectangle(ctx, DrawMode::fill(), border_rect, Color::WHITE)?,
            DrawParam::default(),
        );
		Ok(())
	}

	pub fn draw_fruits(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
		for (i, row) in self.fruits.iter().enumerate() {
            for (j, &has_fruit) in row.iter().enumerate() {
                if has_fruit {
                    let rect = Rect::new(i as f32 * self.cell_size, j as f32 * self.cell_size, self.cell_size, self.cell_size);
                    canvas.draw(
                        &Mesh::new_rectangle(ctx, DrawMode::fill(), rect, Color::RED)?,
                        DrawParam::default(),
                    );
                }
            }
        }
		Ok(())
	}

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        self.draw_map(ctx, canvas)?;
        self.draw_fruits(ctx, canvas)
    }
}