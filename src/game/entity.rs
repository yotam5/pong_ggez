use ggez::mint::{Point2,Vector2};
use ggez::{Context,GameResult};

/// entity trait for game entities/objects
pub(super) trait Entity
{
	fn get_pos() -> Point2::<f32>;
	fn get_velocity() -> Vector2::<f32>;
	fn update_pos(point: Point2::<f32>);
	fn update_velocity(veocity: Vector2::<f32>);
	fn draw(ctx: &mut Context) -> GameResult<()>;
}