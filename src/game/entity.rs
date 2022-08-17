use std::cmp::Ordering;
use std::convert::From;

use ggez::mint::Vector2;
use ggez::{graphics, input::keyboard, GameResult};

use crate::game::entity::Direction::{DownLeft, UpRight};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Direction {
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
    /// change vector of 2d velocity to direction
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
    /// from keycode to the according direction
    pub fn from_keycode(key: &keyboard::KeyCode) -> Option<Direction> {
        match key {
            keyboard::KeyCode::Up => Some(Direction::Up),
            keyboard::KeyCode::Down => Some(Direction::Down),
            _ => None,
        }
    }

    /// check if two directions have common Y vector
    pub fn same_vertical(dir1: &Direction, dir2: &Direction) -> bool {
        match (dir1, dir2) {
            (
                Direction::Up | Direction::UpRight | Direction::UpLeft,
                Direction::Up | Direction::UpRight | Direction::UpLeft,
            ) => true,
            (
                Direction::Down | Direction::DownLeft | Direction::DownRight,
                Direction::Down | Direction::DownLeft | Direction::DownRight,
            ) => true,

            _ => false,
        }
    }

    /// check if two directions have opposite Y vector
    pub fn opposite_vertical(dir1: &Direction, dir2: &Direction) -> bool {
        match (dir1, dir2) {
            (
                Direction::Up | Direction::UpRight | Direction::UpLeft,
                Direction::Down | Direction::DownLeft | Direction::DownRight,
            ) => true,
            (
                Direction::Down | Direction::DownLeft | Direction::DownRight,
                Direction::Up | Direction::UpRight | Direction::UpLeft,
            ) => true,
            _ => false,
        }
    }
}

/// entity trait for game entities/objects
pub(super) trait Entity {
    /// return the velocity vector by ref
    fn get_velocity(&self) -> &Vector2<f32>;
    /// draw the entity to the canvas
    fn draw(&self, ctx: &mut graphics::Canvas) -> GameResult<()>;
}
