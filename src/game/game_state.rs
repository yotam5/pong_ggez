use ggez;
use ggez::event::{self, EventHandler};
use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode::P;
use ggez::mint::{Point2, Vector2};
use ggez::{
    graphics::{self, Color},
    Context, GameError, GameResult,
};

use crate::game::ball::Ball;

use super::config;
use super::entity::{Direction, Entity};
use super::paddle::{self, Paddle};

/// struct that handle the game managment/running and called with ggez::event
pub(crate) struct Game {
    player_paddle: Paddle,
    bot_paddle: Paddle,
    conf: config::Config,
    ball: Ball,
}

impl Game {
    /// create new Game struct
    pub fn new(_ctx: &mut Context) -> GameResult<Self> {
        let conf = config::Config::new("config.toml");
        let game = Game {
            player_paddle: Paddle::new(
                Point2 {
                    x: conf.player_location.x as f32,
                    y: conf.player_location.y as f32,
                },
                Vector2 {
                    x: conf.paddle.speed_x as f32,
                    y: conf.paddle.speed_y as f32,
                },
                conf.paddle.width as f32,
                conf.paddle.height as f32,
                conf.display.height as u16,
            ),
            bot_paddle: Paddle::new(
                Point2 {
                    x: conf.bot_location.x as f32,
                    y: conf.bot_location.y as f32,
                },
                Vector2 {
                    x: conf.paddle.speed_x as f32,
                    y: conf.paddle.speed_y as f32,
                },
                conf.paddle.width as f32,
                conf.paddle.height as f32,
                conf.display.height as u16,
            ),
            ball: Ball::new(
                _ctx,
                Point2 {
                    x: conf.ball.x as f32,
                    y: conf.ball.y as f32,
                },
                Vector2 {
                    x: conf.ball.speed_x as f32,
                    y: conf.ball.speed_y as f32,
                },
                conf.ball.radius,
            ),
            conf,
        };

        Ok(game)
    }

    fn ball_wall_collision(&self) -> bool {
        let ball_center = self.ball.get_center();
        let ball_radius = self.ball.get_radius() as f32;

        ball_center.y + ball_radius >= self.conf.display.height as f32
            || ball_center.y - ball_radius <= 0.0
    }

    /// start the game main loop with ggez::event::run
    pub fn start() -> ! {
        let conf = config::Config::new("config.toml");
        let cb = ggez::ContextBuilder::new("Pong Game", "YotamST")
            .window_setup(ggez::conf::WindowSetup::default().title("Pong!"))
            .window_mode(
                ggez::conf::WindowMode::default()
                    .dimensions(conf.display.width as f32, conf.display.height as f32),
            );
        let (mut ctx, event_loop) = cb.build().unwrap();
        let state = Game::new(&mut ctx).unwrap();
        event::run(ctx, event_loop, state);
    }
}

impl EventHandler for Game {
    /// update the game each loop
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        while _ctx.time.check_update_time(self.conf.display.fps as u32) {}

        if self.ball_wall_collision() {
            self.ball.change_y_direction();
        }
        if self.player_paddle.hit_circle(self.ball.get_center(),self.ball.get_radius() as f32)
        {
            //todo! check who hit the ball  and ad the velocity
            // each time if the paddle is in the direction the ball moves add speed
            // if not, deduct speed
            println!("{}","paddle hit ball");
        }
        self.ball.update_position();

        let currently_pressed = _ctx.keyboard.pressed_keys();
        for key in currently_pressed {
            if let Some(direction) = Direction::from_keycode(key) {
                self.player_paddle.update_position(&direction);
                //println!("{}", "key preseed");
            }
        }

        Ok(())
    }

    /// handle drawing each game loop
    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(_ctx, Color::BLACK);
        //start drawing here
        self.player_paddle.draw(&mut canvas).unwrap();
        self.bot_paddle.draw(&mut canvas).unwrap();
        self.ball.draw(&mut canvas).unwrap();
        //end drawing here canvas.finish(_ctx)?;
        canvas.finish(_ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _input: keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), GameError> {
        Ok(())
    }
}
