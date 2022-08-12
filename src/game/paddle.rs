use ggez::mint::{Point2, Vector2};
use ggez::{self, graphics};

use super::entity;

pub(super) struct Paddle {
    position: Point2<f32>,
    velocity: Vector2<f32>,
    width: f32,
    height: f32,
    rectangle: graphics::Mesh,
}

impl Paddle {
    pub fn new(pos: Point2<f32>, vel: Vector2<f32>, width: f32, height: f32) -> Self {
    	Paddle {
    		position: pos,
    		velocity: vel,
    		width: width,
    		height: height,
    		rectangle: graphics::Mesh::new_rectangle(gfx, mode, bounds, color)
    	}
    }
}

impl entity::Entity for Paddle {
    fn get_pos() -> Point2<f32> {
        todo!()
    }

    fn get_velocity() -> Vector2<f32> {
        todo!()
    }

    fn update_pos(point: Point2<f32>) {
        todo!()
    }

    fn update_velocity(veocity: Vector2<f32>) {
        todo!()
    }

    fn draw(ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        todo!()
    }
}
