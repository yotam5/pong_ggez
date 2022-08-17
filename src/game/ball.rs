use ggez::{self, GameResult, graphics};
use ggez::graphics::{Canvas,Color};
use super::entity::{self,Direction};
use ggez::mint::{Point2, Vector2};

pub(super) struct Ball{
	center: Point2<f32>,
	ball: graphics::Mesh,
	velocity: Vector2<f32>,
	direction: Direction,
	radius: u8,
}

impl Ball{
	pub fn new(ctx: &mut ggez::Context, position: Point2<f32>, velocity: Vector2<f32>, radius: u8) -> Self{

		Ball{
			velocity,
			radius,
			ball: graphics::Mesh::new_circle(ctx,graphics::DrawMode::fill(),
				Point2{x: 0.0,y: 0.0},radius as f32,0.1,Color::WHITE).unwrap(),
				center: position,
				direction: Direction::Right,
		}
	}

	pub fn accelerate_x(&mut self,ratio: f32)
	{
		self.velocity.x *= ratio;
	}

	pub fn accelerate_y(&mut self, ratio: f32)
	{
		self.velocity.y *= ratio;
	}

	pub fn change_y_direction(&mut self)
	{
		self.velocity.y *= -1.0;
	}

	pub fn change_x_direction(&mut self)
	{
		self.velocity.x *= -1.0;
	}

	pub fn get_direction(&self) -> Direction
	{
		self.velocity.into()
	}

	pub	fn update_position(&mut self){

		self.center.x += self.velocity.x;
		self.center.y += self.velocity.y;
		self.direction = self.get_direction();
		//println!("{:?}",self.direction);
	}

	pub fn get_radius(&self) -> u8 {
		self.radius
	}

	pub fn center(&self) -> &Point2<f32>
	{
		&self.center
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