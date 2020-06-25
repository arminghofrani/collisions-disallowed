use ggez::{
    event::{self, EventHandler},
    *,
};

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("collisions-disallowed", "Armin Ghofrani")
        .build()
        .expect("Could not create ggez context!");

    let mut game = Game::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct Game {}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        Game {}
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            mint::Point2 { x: 200.0, y: 200.0 },
            100.0,
            0.1,
            graphics::BLACK,
        )?;

        graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
        graphics::present(ctx)
    }
}
