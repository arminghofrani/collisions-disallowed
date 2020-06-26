use ggez::{
    conf::{NumSamples, WindowSetup},
    event::{self, EventHandler},
    *,
};

fn main() {
    let window_setup = WindowSetup {
        title: "collisions-disallowed".to_owned(),
        samples: NumSamples::Zero,
        vsync: true,
        icon: "".to_owned(),
        srgb: true,
    };

    let (mut ctx, mut event_loop) = ContextBuilder::new("collisions-disallowed", "Armin Ghofrani")
        .window_setup(window_setup)
        .build()
        .expect("Could not create ggez context!");

    let mut game = Game::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct Game {
    x: f32,
    y: f32,
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        Game { x: 0.0, y: 0.0 }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let fps = timer::fps(ctx);
        let fps_display = graphics::Text::new(format!("FPS: {}", fps));

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            mint::Point2 {
                x: self.x,
                y: self.y,
            },
            100.0,
            0.1,
            graphics::BLACK,
        )?;

        self.x += 1.0;
        self.y += 1.0;

        graphics::clear(ctx, graphics::WHITE);
        graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
        graphics::draw(
            ctx,
            &fps_display,
            (mint::Point2 { x: 0.0, y: 0.0 }, graphics::BLACK),
        )?;
        graphics::present(ctx)
    }
}
