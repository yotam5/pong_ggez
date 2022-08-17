use std::{thread, time};

use ggez;
use ggez::event::{self, EventHandler};
use ggez::mint::{Point2, Vector2};
use ggez::{
    graphics::{self, Color},
    Context, GameResult,
};
use rand::Rng;

use crate::game::ball::Ball;

use super::config;
use super::entity::{Direction, Entity};
use super::paddle::Paddle;

/// struct that handle the game managment/running and called with ggez::event
pub(crate) struct Game {
    player_paddle: Paddle,
    bot_paddle: Paddle,
    conf: config::Config,
    ball: Ball,
    rng: rand::rngs::ThreadRng,
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
            rng: rand::thread_rng(),
        };

        Ok(game)
    }

    /// check if the ball went outside the window or passed the paddle
    fn ball_outside_window(&self) -> bool {
        let ball_center = self.ball.center();
        ball_center.x <= self.conf.bot_location.x as f32
            || ball_center.x >= self.conf.player_location.x as f32
    }

    /// check for ball collision with the wall
    fn ball_wall_collision(&self) -> bool {
        let ball_center = self.ball.center();
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

    ///  handle paddle-ball collision increase/decrease speed
    fn handle_paddle_collision(&mut self) {
        let player_hit: bool = self
            .player_paddle
            .hit_circle(self.ball.center(), self.ball.get_radius() as f32);
        let bot_hit: bool = self
            .bot_paddle
            .hit_circle(self.ball.center(), self.ball.get_radius() as f32);

        let mut hit_direction = &Direction::None;
        let ball_hit: bool = player_hit || bot_hit;
        if player_hit {
            hit_direction = self.player_paddle.get_current_direction();
        } else if bot_hit {
            hit_direction = self.bot_paddle.get_current_direction();
        }

        if ball_hit {
            if Direction::same_vertical(&hit_direction, &self.ball.get_direction()) {
                //println!("paddle hit ball in the same velocity direction");
                self.ball.accelerate_y(self.rng.gen_range(1.0..1.3));
            } else if Direction::opposite_vertical(&hit_direction, &self.ball.get_direction()) {
                self.ball.accelerate_y(self.rng.gen_range(0.7..0.95));
            }

            if rand::random()
            {
                self.ball.accelerate_x(self.rng.gen_range(1.0..1.3))
            }
            self.ball.change_x_direction();
        }
    }

    fn handle_bot_paddle(&mut self) {
        if [Direction::UpLeft, Direction::DownLeft].contains(&self.ball.get_direction()) {
            let ball_center = self.ball.center();

            if self.bot_paddle.top() >= ball_center.y {
                self.bot_paddle.update_position(&Direction::Up);
            } else if self.bot_paddle.bottom() <= ball_center.y {
                self.bot_paddle.update_position(&Direction::Down);
            }
        }
    }

    /// handle the player paddle input and movment
    fn handle_player_paddle(&mut self, _ctx: &mut Context) {
        self.player_paddle.rest();
        let currently_pressed = _ctx.keyboard.pressed_keys();
        for key in currently_pressed {
            if let Some(direction) = Direction::from_keycode(key) {
                self.player_paddle.update_position(&direction);
            }
        }
    }

    fn handle_wall_collision(&mut self) {
        if self.ball_wall_collision() {
            self.ball.change_y_direction();
        }
    }
}

impl EventHandler for Game {
    /// update the game each loop
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        while _ctx.time.check_update_time(self.conf.display.fps as u32) {}
        if self.ball_outside_window() {
            let three_sec = time::Duration::from_secs(3);
            thread::sleep(three_sec);
            ggez::event::request_quit(_ctx);
        }
        self.handle_wall_collision();
        self.ball.update_position();
        self.handle_paddle_collision();
        self.handle_player_paddle(_ctx);
        self.handle_bot_paddle();

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
}
