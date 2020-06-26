use ggez::{
    conf::{FullscreenType, NumSamples, WindowMode, WindowSetup},
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

    let window_mode = WindowMode {
        width: 1200.0,
        height: 900.0,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: false,
    };

    let (mut ctx, mut event_loop) = ContextBuilder::new("collisions-disallowed", "arminghofrani")
        .window_setup(window_setup)
        .window_mode(window_mode)
        .build()
        .expect("Could not create ggez context!");

    let mut game = Game::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct Game {
    circle_positions: [mint::Point2<f32>; 50],
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        let mut initial_circle_positions: [mint::Point2<f32>; 50] =
            [mint::Point2 { x: 0.0, y: 0.0 }; 50];

        for i in 0..50 {
            initial_circle_positions[i] = mint::Point2 {
                x: rand::random::<f32>() * 1200.0,
                y: rand::random::<f32>() * 900.0,
            };
        }

        Game {
            circle_positions: initial_circle_positions,
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let fps = timer::fps(ctx);
        let fps_display = graphics::Text::new(format!("FPS: {}", fps));

        let mut mesh_builder = graphics::MeshBuilder::new();
        for i in 0..50 {
            mesh_builder.circle(
                graphics::DrawMode::fill(),
                self.circle_positions[i],
                15.0,
                0.1,
                graphics::BLACK,
            );
            self.circle_positions[i].x += 1.0;
            self.circle_positions[i].y += 1.0;
        }
        let mesh = mesh_builder.build(ctx)?;

        graphics::clear(ctx, graphics::WHITE);
        graphics::draw(
            ctx,
            &mesh,
            (mint::Point2 { x: 0.0, y: 0.0 }, graphics::BLACK),
        )?;
        graphics::draw(
            ctx,
            &fps_display,
            (mint::Point2 { x: 0.0, y: 0.0 }, graphics::BLACK),
        )?;
        graphics::present(ctx)
    }
}
