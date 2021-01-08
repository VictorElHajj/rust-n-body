use std::time::{self, Duration};

use ggez::{
    conf::{FullscreenType, WindowMode},
    event::{self, EventHandler},
    mint::Point2,
    timer::sleep,
};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use graphics::MeshBuilder;
use n_body::{
    body::Body, quadtree::QuadTree, rectangle::Rectangle, simulation::Simulation, vector::Vector2,
};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut bs = Vec::with_capacity(1000);
    for i in 0..1000 {
        let b = Body {
            id: i,
            pos: Vector2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0))
                * rng.gen_range(0.0..(i as f64 + 1.0)),
            vel: Vector2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0))
                * rng.gen_range(0.0..(i as f64 + 1.0))
                / 100.0,
            acc: Vector2::zero(),
            mass: 2_000_000.0,
        };
        bs.push(b);
    }
    // Black hole
    let b = Body {
        id: 1001,
        pos: Vector2::zero(),
        vel: Vector2::zero(),
        acc: Vector2::zero(),
        mass: 2_000_000_000_000.0,
    };
    bs.push(b);

    let mut sim = Simulation {
        bodies: Box::new(bs),
        qt: QuadTree::new(Rectangle {
            pos: Vector2::zero(),
            size: 0.0,
        }),
        timestep: 0.1,
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
