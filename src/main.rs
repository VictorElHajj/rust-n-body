use ggez::{
    conf::{FullscreenType, WindowMode},
    event::{self, EventHandler},
};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use n_body::{body::Body, cube::Cube, octree::OcTree, simulation::Simulation, vector::Vector3};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut bs = Vec::with_capacity(1000);
    for i in 0..1000 {
        let b = Body {
            id: i,
            pos: Vector3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0)
                * rng.gen_range(0.0..(i as f64 + 1.0)),
            vel: Vector3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0)
                * rng.gen_range(0.0..(i as f64 + 1.0))
                / 100.0,
            acc: Vector3::zero(),
            mass: 2_000_000.0,
        };
        bs.push(b);
    }
    // Black hole
    let b = Body {
        id: 1001,
        pos: Vector3::zero(),
        vel: Vector3::zero(),
        acc: Vector3::zero(),
        mass: 2_000_000_000_000.0,
    };
    bs.push(b);

    let sim = Simulation {
        bodies: Box::new(bs),
        ot: OcTree::new(Cube {
            pos: Vector3::zero(),
            size: 0.0,
        }),
        timestep: 1.0,
        theta: 0.8,
    };

    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(&mut ctx, sim);

    graphics::set_mode(
        &mut ctx,
        WindowMode {
            width: 1000.0,
            height: 1000.0,
            maximized: false,
            fullscreen_type: FullscreenType::Windowed,
            borderless: false,
            min_width: 0.0,
            max_width: 0.0,
            min_height: 0.0,
            max_height: 0.0,
            resizable: false,
        },
    )
    .ok();

    graphics::set_screen_coordinates(&mut ctx, graphics::Rect::new(0.0, 0.0, 1000.0, 1000.0)).ok();

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct MyGame {
    sim: Simulation,
    // Your state here...
}

impl MyGame {
    pub fn new(_ctx: &mut Context, sim: Simulation) -> MyGame {
        // Load/create resources such as images here.
        MyGame { sim }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        self.sim.update();

        //let mut ke: f64 = 0.0;
        //for b in self.sim.bodies.iter() {
        //    ke += 0.5 * b.mass * Vector2::zero().distance(b.vel).powi(2);
        //}
        //println!("System kinetic energy: {}", ke);

        //std::thread::sleep(time::Duration::from_secs(1));
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        self.sim.draw(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }
}
