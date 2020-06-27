use ggez::{
    conf::{FullscreenType, NumSamples, WindowMode, WindowSetup},
    event::{self, EventHandler},
    *,
};
use rand::Rng;

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 900.0;

fn main() {
    let window_setup = WindowSetup {
        title: "collisions-disallowed".to_owned(),
        samples: NumSamples::Zero,
        vsync: true,
        icon: "".to_owned(),
        srgb: true,
    };

    let window_mode = WindowMode {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
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
        .expect("Could not create ggez context");

    let mut game = Game::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct SpeedVars {
    speed_down_factor: f32,
    speed_down_counter: f32,
    waiting_speed_factor: f32,
    speed_down_factor_goal: f32,
}

struct Game {
    positions: [mint::Point2<f32>; 50],
    velocities: [(f32, f32); 50],
    speed_vars: SpeedVars,
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        let mut rng = rand::thread_rng();

        let mut init_positions: [mint::Point2<f32>; 50] = [mint::Point2 { x: 0.0, y: 0.0 }; 50];
        let mut init_velocities: [(f32, f32); 50] = [(0.0, 0.0); 50];
        let init_speed_vars = SpeedVars {
            speed_down_factor: 1.0,
            speed_down_counter: 1.0,
            waiting_speed_factor: 1.0,
            speed_down_factor_goal: 1.0,
        };

        for i in 0..50 {
            let angle = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
            let radius = WINDOW_HEIGHT * 0.5;

            let init_x = radius * angle.cos();
            let init_y = radius * angle.sin();

            let velocity = (if rng.gen::<i32>() % 2 == 0 { 1.0 } else { -1.0 })
                * ((rng.gen::<i32>() % 20 + 150) as f32);

            init_positions[i] = mint::Point2 {
                x: init_x + WINDOW_WIDTH * 0.5,
                y: init_y + WINDOW_HEIGHT * 0.5,
            };

            init_velocities[i] = (-1.0 * angle.sin() * velocity, angle.cos() * velocity);
        }

        Game {
            positions: init_positions,
            velocities: init_velocities,
            speed_vars: init_speed_vars,
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let fps = timer::fps(ctx);
        let fps_display = graphics::Text::new(format!("FPS: {:.2}", fps));

        let mut mesh_builder = graphics::MeshBuilder::new();
        for i in 0..50 {
            mesh_builder.circle(
                graphics::DrawMode::fill(),
                self.positions[i],
                20.0,
                0.1,
                graphics::WHITE,
            );
            mesh_builder.circle(
                graphics::DrawMode::stroke(2.0),
                self.positions[i],
                20.0,
                0.1,
                graphics::Color::new(0.0, 0.0, 1.0, 1.0),
            );
        }
        let mesh = mesh_builder.build(ctx)?;

        graphics::clear(ctx, graphics::BLACK);
        graphics::draw(
            ctx,
            &mesh,
            (mint::Point2 { x: 0.0, y: 0.0 }, graphics::WHITE),
        )?;
        graphics::draw(
            ctx,
            &fps_display,
            (mint::Point2 { x: 0.0, y: 0.0 }, graphics::WHITE),
        )?;
        graphics::present(ctx)
    }
}
