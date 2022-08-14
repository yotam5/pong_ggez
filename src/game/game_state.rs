
use super::config;
use super::entity::Entity;
use super::paddle::{self, Paddle};
use ggez;
use ggez::event::{self, EventHandler};
use ggez::mint::{Point2, Vector2};
use ggez::{
    graphics::{self, Color},
    Context, GameError, GameResult,
};

/// struct that handle the game managment/running and called with ggez::event
pub(crate) struct Game {
    player_paddle: paddle::Paddle,
    conf: config::Config,
}

impl Game {
    /// create new Game struct
    pub fn new(_ctx: &mut Context) -> GameResult<Self> {
        let conf = config::Config::new("config.toml");
        let game = Game {
            player_paddle: Paddle::new(
                Point2 { x: 0.0, y: 0.0 },
                Vector2 {
                    x: conf.paddle.speed_x as f32,
                    y: conf.paddle.speed_y as f32,
                },
                conf.paddle.width as f32,
                conf.paddle.height as f32,
            ),
            conf,
        };

        Ok(game)
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
        Ok(())
    }

    /// handle drawing each game loop
    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(_ctx, Color::BLACK);
        //start drawing here
        self.player_paddle.draw(&mut canvas).unwrap();
        //end drawing here canvas.finish(_ctx)?;
        canvas.finish(_ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), GameError> {
        Ok(())
    }
}
