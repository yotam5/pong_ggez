use ggez::mint::{Point2,Vector2};
use ggez::{GameResult,graphics,input::keyboard};
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
   pub fn inverse(&self) -> Self {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn from_keycode(key: &keyboard::KeyCode) -> Option<Direction> {
        match key {
            keyboard::KeyCode::Up => Some(Direction::Up),
            keyboard::KeyCode::Down => Some(Direction::Down),
            _ => None,
        }
    }

    pub fn to_keycode(key: &Direction) -> Option<ggez::winit::event::VirtualKeyCode>
    {
    	match key{
    		Direction::Up => Some(keyboard::KeyCode::Up),
    		Direction::Down => Some(keyboard::KeyCode::Down),
    		_ => None,
    	}	
    }
}

/// entity trait for game entities/objects
pub(super) trait Entity
{
	fn get_velocity(&self) -> &Vector2<f32>;
	fn draw(&self, ctx: &mut graphics::Canvas) -> GameResult<()>;
}