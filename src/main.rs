use ggez::{
    conf::{FullscreenType, NumSamples, WindowMode, WindowSetup},
    event::{self, EventHandler},
    *,
};
use rand::Rng;

#[macro_use]
extern crate text_io;

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 900.0;

fn main() {
    println!("Number of circles:");
    let n_circles: usize = read!();

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

    let mut game = Game::new(&mut ctx, n_circles);

    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct Game {
    positions: Vec<mint::Point2<f32>>,
    velocities: Vec<mint::Vector2<f32>>,
}

impl Game {
    pub fn new(_ctx: &mut Context, n_circles: usize) -> Game {
        let mut rng = rand::thread_rng();

        let mut init_positions: Vec<mint::Point2<f32>> =
            vec![mint::Point2 { x: 0.0, y: 0.0 }; n_circles];
        let mut init_velocities: Vec<mint::Vector2<f32>> =
            vec![mint::Vector2 { x: 0.0, y: 0.0 }; n_circles];

        for i in 0..n_circles {
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

            init_velocities[i] = mint::Vector2 {
                x: -1.0 * angle.sin() * velocity,
                y: angle.cos() * velocity,
            };
        }

        Game {
            positions: init_positions,
            velocities: init_velocities,
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let attraction_force = 0.05;
        let center = mint::Point2 {
            x: WINDOW_WIDTH * 0.5,
            y: WINDOW_HEIGHT * 0.5,
        };

        for i in 0..self.positions.len() {
            let to_center = subtract_points(center, self.positions[i]);
            self.velocities[i] = add_vector(
                self.velocities[i],
                scale_vector(to_center, attraction_force),
            );

            for j in (i + 1)..self.positions.len() {
                let to_collider_dist = dist_points(self.positions[i], self.positions[j]);

                let min_dist = 40.0;
                if to_collider_dist < min_dist {
                    let collision = scale_vector(
                        subtract_points(self.positions[i], self.positions[j]),
                        1.0 / to_collider_dist,
                    );

                    self.positions[i] = add_to_point(
                        self.positions[i],
                        scale_vector(collision, 0.5 * (min_dist - to_collider_dist)),
                    );
                    self.positions[j] = add_to_point(
                        self.positions[j],
                        scale_vector(collision, -0.5 * (min_dist - to_collider_dist)),
                    );
                }
            }
        }

        for i in 0..self.positions.len() {
            self.positions[i] = add_to_point(
                self.positions[i],
                scale_vector(
                    self.velocities[i],
                    timer::delta(ctx).as_secs_f32(),
                ),
            );
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let fps = timer::fps(ctx);
        let fps_display = graphics::Text::new(format!("FPS: {:.2}", fps));

        let mut mesh_builder = graphics::MeshBuilder::new();
        for i in 0..self.positions.len() {
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

fn subtract_points(p1: mint::Point2<f32>, p2: mint::Point2<f32>) -> mint::Vector2<f32> {
    mint::Vector2 {
        x: p1.x - p2.x,
        y: p1.y - p2.y,
    }
}

fn dist_points(p1: mint::Point2<f32>, p2: mint::Point2<f32>) -> f32 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

fn scale_vector(v: mint::Vector2<f32>, s: f32) -> mint::Vector2<f32> {
    mint::Vector2 {
        x: v.x * s,
        y: v.y * s,
    }
}

fn add_vector(v1: mint::Vector2<f32>, v2: mint::Vector2<f32>) -> mint::Vector2<f32> {
    mint::Vector2 {
        x: v1.x + v2.x,
        y: v1.y + v2.y,
    }
}

fn add_to_point(p: mint::Point2<f32>, v: mint::Vector2<f32>) -> mint::Point2<f32> {
    mint::Point2 {
        x: p.x + v.x,
        y: p.y + v.y,
    }
}
