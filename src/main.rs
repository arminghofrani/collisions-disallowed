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
    let mut n_circles: i32 = 0;
    while n_circles < 1 {
        println!("Number of circles (at least 1):");
        n_circles = read!();
    }

    let mut max_radius: i32 = 0;
    while max_radius < 5 {
        println!("Maximum circle radius in pixels (at least 5):");
        max_radius = read!();
    }

    let mut max_velocity: i32 = 0;
    while max_velocity < 1 {
        println!("Maximum velocity (at least 1):");
        max_velocity = read!();
    }

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

    let mut game = Game::new(
        &mut ctx,
        n_circles as usize,
        max_radius as f32,
        max_velocity as f32,
    );

    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct Game {
    positions: Vec<mint::Point2<f32>>,
    velocities: Vec<mint::Vector2<f32>>,
    radii: Vec<f32>,
    cleans: Vec<u32>,
    stables: Vec<bool>,
    n_stables: u32,
    stable: bool,
}

impl Game {
    pub fn new(_ctx: &mut Context, n_circles: usize, max_radius: f32, max_velocity: f32) -> Game {
        let mut rng = rand::thread_rng();

        let mut init_positions: Vec<mint::Point2<f32>> =
            vec![mint::Point2 { x: 0.0, y: 0.0 }; n_circles];
        let mut init_velocities: Vec<mint::Vector2<f32>> =
            vec![mint::Vector2 { x: 0.0, y: 0.0 }; n_circles];
        let mut radii: Vec<f32> = vec![0.0; n_circles];
        let cleans: Vec<u32> = vec![0; n_circles];
        let stables: Vec<bool> = vec![false; n_circles];

        for i in 0..n_circles {
            init_positions[i] = mint::Point2 {
                x: rng.gen::<f32>() * WINDOW_WIDTH,
                y: rng.gen::<f32>() * WINDOW_HEIGHT,
            };
            init_velocities[i] = mint::Vector2 {
                x: (if rng.gen::<i32>() % 2 == 0 { -1.0 } else { 1.0 })
                    * rng.gen::<f32>()
                    * max_velocity,
                y: (if rng.gen::<i32>() % 2 == 0 { -1.0 } else { 1.0 })
                    * rng.gen::<f32>()
                    * max_velocity,
            };
            radii[i] = rng.gen::<f32>() * (max_radius - 5.0) + 5.0;
        }

        Game {
            positions: init_positions,
            velocities: init_velocities,
            radii: radii,
            cleans: cleans,
            stables: stables,
            n_stables: 0,
            stable: false,
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

        self.stable = true;
        for i in 0..self.positions.len() {
            let to_center = subtract_points(center, self.positions[i]);
            self.velocities[i] = add_vector(
                self.velocities[i],
                scale_vector(to_center, attraction_force),
            );

            let mut clean = true;
            for j in (i + 1)..self.positions.len() {
                let to_collider_dist = dist_points(self.positions[i], self.positions[j]);

                let min_dist = self.radii[i] + self.radii[j];
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

                    clean = false;
                    self.stable = false;

                    self.cleans[j] = 0;
                    self.stables[j] = false;
                }
            }

            if clean {
                if !self.stables[i] {
                    self.stable = false;
                    self.cleans[i] += 1;

                    if self.cleans[i] >= 600 {
                        self.stables[i] = true;
                    }
                }
            } else {
                self.cleans[i] = 0;
                self.stables[i] = false;
            }
        }

        self.n_stables = 0;
        for i in 0..self.positions.len() {
            self.positions[i] = add_to_point(
                self.positions[i],
                scale_vector(self.velocities[i], timer::delta(ctx).as_secs_f32()),
            );

            if self.stables[i] {
                self.n_stables += 1;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let fps = timer::fps(ctx);
        let fps_display = graphics::Text::new(format!("FPS: {:.2}", fps));
        let stable_display = graphics::Text::new(format!(
            "STABLE: {}/{}",
            self.n_stables,
            self.positions.len()
        ));

        let mut mesh_builder = graphics::MeshBuilder::new();
        for i in 0..self.positions.len() {
            mesh_builder.circle(
                graphics::DrawMode::fill(),
                self.positions[i],
                self.radii[i],
                0.1,
                if self.stables[i] {
                    graphics::Color::new(0.0, 0.0, 1.0, 1.0)
                } else {
                    graphics::Color::new(1.0, 0.0, 0.0, 1.0)
                },
            );
            mesh_builder.circle(
                graphics::DrawMode::stroke(3.0),
                self.positions[i],
                self.radii[i],
                0.1,
                graphics::WHITE,
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
        graphics::draw(
            ctx,
            &stable_display,
            (mint::Point2 { x: 0.0, y: 16.0 }, graphics::WHITE),
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
