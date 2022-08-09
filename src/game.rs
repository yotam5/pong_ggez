use ggez;
use ggez::event::{self, EventHandler, MouseButton};
use ggez::{graphics::{self,Color}, Context, GameResult};
type Vector = ggez::mint::Vector2<f32>;

pub struct Game
{
}

impl Game{
	pub fn new(ctx: &mut Context) -> GameResult<Self>
	{
		let game = Game{};
		Ok(game)
	}

	pub fn start() -> GameResult
	{
		let cb = ggez::ContextBuilder::new("Pong Game","YotamST")
			.window_setup(ggez::conf::WindowSetup::default().title("Pong!"));
		let (mut ctx, event_loop) = cb.build().unwrap();
		let state = Game::new(&mut ctx).unwrap();
		event::run(ctx,event_loop,state);
		Ok(())
	}
}

impl EventHandler for Game
{
	/// update the game each loop
	fn update(&mut self,_ctx: &mut Context) -> GameResult<()>
	{
		Ok(())
	}
	    
	fn draw(&mut self, _ctx: &mut Context) -> GameResult<()>
	{
		graphics::clear(_ctx,Color::BLACK);

		//start drawing here

		//end drawing here

		graphics::present(_ctx)?;
		Ok(())
	}
}
