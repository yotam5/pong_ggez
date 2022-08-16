use super::entity::{self,Direction};
use ggez::graphics::Color;
use ggez::mint::{Point2, Vector2};
use ggez::{self, graphics};

/// paddle struct which player hit the ball with 
pub(super) struct Paddle {
    velocity: Vector2<f32>,
    paddle: graphics::Rect,
    window_height: u16,
}

/// impl paddle functions
impl Paddle {
    /// create a new paddle at a given position, velocity and width/height
    pub fn new(position: Point2<f32>, velocity: Vector2<f32>, width: f32, height: f32,
            window_height: u16) -> Self {
        Paddle {
            velocity,
            paddle: graphics::Rect::new(position.x, position.y, width, height),
            window_height
        }
    }

    /// moving the paddle up according to the current velocity
    fn move_up(&mut self)
    {
        self.paddle.y -= self.velocity.y;
    }

    /// move the paddle down according to the current velocity
    fn move_down(&mut self)
    {
        self.paddle.y += self.velocity.y;
    }

    /// update the paddle position according to the directions
    /// and check before that the paddle wont go outside of the window
    pub fn update_position(&mut self,direction: &Direction)
    {
        match *direction{
            Direction::Up => {
                if self.paddle.top() - self.velocity.y >= 0.0{
                    self.move_up();
                }
            },
            Direction::Down => {
                if self.paddle.bottom() + self.velocity.y <= self.window_height as f32{
                    self.move_down()
                }
            },

            Direction::Right  | Direction::Left => {}
        }
    }
}

/// impl Entity trait for the paddle
impl entity::Entity for Paddle {
    /// return reference to the current velocity
    fn get_velocity(&self) -> &Vector2<f32> {
        &self.velocity
    }

    /// draw the paddle to the screen
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
