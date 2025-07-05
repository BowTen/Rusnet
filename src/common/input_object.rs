use std::any::Any;

use super::StateResult;
use ggez::Context;
use ggez::graphics::{DrawParam, Drawable};
use ggez::input::keyboard;

pub type KeyDownCallBack = dyn Fn(
    &mut Context,
    ggez::input::keyboard::KeyInput,
    bool,
) -> Result<super::StateResult, ggez::GameError>;

pub trait InputObject: Any {
    fn focus(&mut self);
    fn unfocus(&mut self);
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: keyboard::KeyInput,
        repeated: bool,
    ) -> Result<StateResult, ggez::GameError> {
        Ok(StateResult::Ok)
    }
    fn draw(&self, canvas: &mut ggez::graphics::Canvas, param: DrawParam);
    fn dimensions(&self, gfx: &Context) -> Option<ggez::graphics::Rect>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
