use ggez;
use ggez::event::{self, EventHandler};
use ggez::{graphics::{self,Color}, Context, GameResult};
use super::entity;
use super::config;
use super::ball;

/// struct that handle the game managment/running and called with ggez::event
pub(crate) struct Game
{
}

impl Game{
	/// create new Game struct
	pub fn new(ctx: &mut Context) -> GameResult<Self>
	{
		let game = Game{
		};
		Ok(game)
	}

	/// start the game main loop with ggez::event::run
	pub fn start() -> !
	{
		let conf = config::Config::new("config.toml");
		let cb = ggez::ContextBuilder::new("Pong Game","YotamST")
			.window_setup(ggez::conf::WindowSetup::default().title("Pong!"))
			.window_mode(ggez::conf::WindowMode::default().dimensions(
				conf.display.width as f32, conf.display.height as f32));
		let (mut ctx, event_loop) = cb.build().unwrap();
		let state = Game::new(&mut ctx).unwrap();
		event::run(ctx,event_loop,state);
	}

}

impl EventHandler for Game
{
	/// update the game each loop
	fn update(&mut self,_ctx: &mut Context) -> GameResult<()>
	{
		const DESIRED_FPS: u32 = 60;
		
		while ggez::timer::check_update_time(_ctx, DESIRED_FPS)
		{

		}
		Ok(())
	}
	
	/// handle drawing each game loop 
	fn draw(&mut self, _ctx: &mut Context) -> GameResult<()>
	{
		graphics::clear(_ctx,Color::BLACK);

		//start drawing here

		//end drawing here

		graphics::present(_ctx)?;
		Ok(())
	}
}
