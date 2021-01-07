use ggez::event::{self, EventHandler};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use n_body::{body::Body, quadtree::QuadTree, rectangle::Rectangle, vector::Vector2};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut qt = QuadTree::new(Rectangle {
        pos: Vector2::new(0.0, 0.0),
        size: 500.0,
    });
    for _ in 0..1000 {
        let b = Body {
            pos: Vector2::new(rng.gen_range(0.0..800.0), rng.gen_range(0.0..800.0)),
            vel: Vector2::zero(),
            mass: rng.gen_range(1.0..100.0),
        };
        qt.insert(b).ok();
    }

    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(&mut ctx, qt);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct MyGame {
    qt: QuadTree,
    // Your state here...
}

impl MyGame {
    pub fn new(_ctx: &mut Context, qt: QuadTree) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            qt
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        self.qt.draw(ctx)?;
        graphics::present(ctx)?;

        Ok(())
    }
}
