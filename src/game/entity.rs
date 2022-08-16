use std::cmp::Ordering;
use std::convert::From;

use ggez::mint::{Point2, Vector2};
use ggez::{graphics, input::keyboard, GameResult};

use crate::game::entity::Direction::{DownLeft, UpRight};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    None,
}

impl From<Vector2<f32>> for Direction {
    fn from(v: Vector2<f32>) -> Direction {
        match (
            v.x.partial_cmp(&0.0).unwrap(),
            v.y.partial_cmp(&0.0).unwrap(),
        ) {
            (Ordering::Equal, Ordering::Equal) => Direction::None,
            (Ordering::Greater, Ordering::Greater) => Direction::DownRight,
            (Ordering::Less, Ordering::Less) => Direction::UpLeft,
            (Ordering::Equal, Ordering::Less) => Direction::Up,
            (Ordering::Equal, Ordering::Greater) => Direction::Down,
            (Ordering::Greater, Ordering::Less) => UpRight,
            (Ordering::Less, Ordering::Greater) => DownLeft,
            (Ordering::Less, Ordering::Equal) => Direction::Left,
            _ => panic!("no such direction lol"),
        }
    }
}

impl Direction {
    pub fn inverse(&self) -> Self {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::DownLeft => UpRight,
            Direction::DownRight => Direction::UpLeft,
            Direction::UpRight => DownLeft,
            Direction::UpLeft => Direction::DownRight,
            Direction::None => Direction::None,
        }
    }

    pub fn from_keycode(key: &keyboard::KeyCode) -> Option<Direction> {
        match key {
            keyboard::KeyCode::Up => Some(Direction::Up),
            keyboard::KeyCode::Down => Some(Direction::Down),
            _ => None,
        }
    }

    pub fn to_keycode(key: &Direction) -> Option<ggez::winit::event::VirtualKeyCode> {
        match key {
            Direction::Up => Some(keyboard::KeyCode::Up),
            Direction::Down => Some(keyboard::KeyCode::Down),
            _ => None,
        }
    }
}

/// entity trait for game entities/objects
pub(super) trait Entity {
    fn get_velocity(&self) -> &Vector2<f32>;
    fn draw(&self, ctx: &mut graphics::Canvas) -> GameResult<()>;
}
