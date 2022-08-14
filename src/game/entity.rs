use ggez::mint::{Point2,Vector2};
use ggez::{Context,GameResult,graphics};

/// entity trait for game entities/objects
pub(super) trait Entity
{
	fn get_center(&self) -> Point2::<f32>;
	fn get_velocity(&self) -> &Vector2::<f32>;
	fn draw(&self, ctx: &mut graphics::Canvas) -> GameResult<()>;
}