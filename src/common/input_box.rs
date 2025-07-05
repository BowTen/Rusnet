use super::{InputObject, StateResult};
use crate::common::tool;
use ggez::graphics::{PxScale, Transform};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::mint::Point2;
use ggez::{
    Context,
    graphics::{self, Color, DrawParam, Drawable, Mesh, Text},
};

pub struct InputBox {
    name: Text,
    color: Color,
    rect: Mesh,
    text: Text,
}

impl InputBox {
    pub fn new(ctx: &Context, name: String) -> Self {
        let mut name = Text::new(name);
        name.set_scale(PxScale::from(20.0));
        let mut text = Text::new(String::new());
        text.set_scale(PxScale::from(35.0));
        InputBox {
            name,
            color: Color::WHITE,
            rect: graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(2f32),
                graphics::Rect::new(0f32, 0f32, 500f32, 50f32),
                graphics::Color::WHITE,
            )
            .unwrap(),
            text,
        }
    }

    pub fn set_rect(&mut self, rect: Mesh) {
        self.rect = rect;
    }

    pub fn content(&self) -> String {
        self.text.contents()
    }

    pub fn set_content(&mut self, content: String) {
        self.text = Text::new(content);
        self.text.set_scale(35.0);
    }
}

impl Drawable for InputBox {
    fn draw(
        &self,
        canvas: &mut ggez::graphics::Canvas,
        param: impl Into<ggez::graphics::DrawParam>,
    ) {
        let param: DrawParam = param.into();
        canvas.draw(&self.rect, param.color(self.color.clone()));
        canvas.draw(&self.text, param.color(Color::GREEN).offset([-5.0, -10.0]));
        let mut param = param;
        if let Transform::Values { dest, .. } = &mut param.transform {
            dest.x -= 5.0;
            dest.y -= 20.0;
        }
        canvas.draw(&self.name, param.color(self.color));
    }

    fn dimensions(
        &self,
        gfx: &impl ggez::context::Has<ggez::graphics::GraphicsContext>,
    ) -> Option<ggez::graphics::Rect> {
        None
    }
}

impl InputObject for InputBox {
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
        if let Some(c) = tool::to_char(&input) {
            self.text.add(c);
        } else if Some(KeyCode::Back) == input.keycode {
            let mut content = self.text.contents();
            content.pop();
            self.text = Text::new(content);
            self.text.set_scale(PxScale::from(35.0));
        }
        Ok(StateResult::Ok)
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
