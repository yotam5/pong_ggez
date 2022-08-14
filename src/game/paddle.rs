use super::entity;
use ggez::graphics::Color;
use ggez::input::keyboard;
use ggez::mint::{Point2, Vector2};
use ggez::{self, graphics};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
}

impl Direction {
    fn inverse(&self) -> Self {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }

    fn from_keycode(key: keyboard::KeyCode) -> Option<Direction> {
        match key {
            keyboard::KeyCode::Up => Some(Direction::Up),
            keyboard::KeyCode::Down => Some(Direction::Down),
            _ => None,
        }
    }
}

pub(super) struct Paddle {
    velocity: Vector2<f32>,
    paddle: graphics::Rect,
}

impl Paddle {
    pub fn new(position: Point2<f32>, velocity: Vector2<f32>, width: f32, height: f32) -> Self {
        Paddle {
            velocity,
            paddle: graphics::Rect::new(position.x, position.y, width, height),
        }
    }
}

impl entity::Entity for Paddle {
    fn get_velocity(&self) -> &Vector2<f32> {
        &self.velocity
    }

    fn get_center(&self) -> Point2<f32> {
        self.paddle.center()
    }

    fn draw(&self, canvas: &mut graphics::Canvas) -> ggez::GameResult<()> {
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest(self.paddle.point())
                .scale(self.paddle.size())
                .color(Color::WHITE),
        );
        Ok(())
    }
}
