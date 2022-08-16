use ggez::{self, GameResult, graphics};
use ggez::graphics::{Canvas,Color};
use super::entity::{self,Direction};
use ggez::mint::{Point2, Vector2};

pub(super) struct Ball{
	center: Point2<f32>,
	ball: graphics::Mesh,
	velocity: Vector2<f32>,
}

impl Ball{
	pub fn new(ctx: &mut ggez::Context, position: Point2<f32>, velocity: Vector2<f32>, radius: u8) -> Self{

		Ball{
			velocity,
			ball: graphics::Mesh::new_circle(ctx,graphics::DrawMode::fill(),
				position,radius as f32,0.1,Color::WHITE).unwrap(),
				center: position
		}
	}
}

impl entity::Entity for Ball
{
	fn get_velocity(&self) -> &Vector2<f32> {
		&self.velocity
	}

	fn draw(&self, ctx: &mut Canvas) -> GameResult<()> {
		ctx.draw(&self.ball,self.center);
		Ok(())
	}
}