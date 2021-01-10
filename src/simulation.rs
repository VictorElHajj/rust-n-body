use ggez::{
    graphics::{self, MeshBuilder},
    mint::Point2,
    Context, GameResult,
};

use crate::{
    body::Body,
    physics_helper::{calc_pull, calc_pull_com},
    quadtree::QuadTree,
    rectangle::Rectangle,
    vector::Vector2,
};

const G: f64 = 6.6674e-11;

pub struct Simulation {
    pub bodies: Box<Vec<Body>>,
    pub qt: QuadTree,
    pub timestep: f64,
    pub theta: f64,
}

impl Simulation {
    pub fn update(&mut self) {
        // Find bounds
        let max_x: f64 = self.bodies.iter().fold(0.0, |a, &b| a.max(b.pos.x.abs()));
        let max_y: f64 = self.bodies.iter().fold(0.0, |a, &b| a.max(b.pos.y.abs()));
        let max_dist = max_x.max(max_y);

        // Build quad tree
        let mut qt = QuadTree::new(Rectangle {
            pos: Vector2::new(0.0 - max_dist, 0.0 - max_dist),
            size: max_dist * 2.0,
        });
        for b in self.bodies.iter() {
            qt.insert(*b).ok();
        }

        // Build new position and velocity
        for b in self.bodies.iter_mut() {
            b.pos = b.pos + b.vel * self.timestep + b.acc * self.timestep * self.timestep * 0.5;
            let old_acc = b.acc;
            b.acc = Simulation::apply_forces(self.theta, &b, &qt) / b.mass * G;
            b.vel = b.vel + (b.acc + old_acc) * (self.timestep * 0.5);
        }

        self.qt = qt;
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let mut builder = MeshBuilder::new();
        for b in self.bodies.iter() {
            builder.circle(
                graphics::DrawMode::fill(),
                Point2 {
                    x: b.pos.x as f32,
                    y: b.pos.y as f32,
                },
                2.0,
                1.0,
                graphics::WHITE,
            );
        }
        let mesh = builder.build(ctx)?;
        graphics::draw(ctx, &mesh, (ggez::mint::Point2 { x: 500.0, y: 500.0 },))?;
        Ok(())
    }

    fn apply_forces(theta: f64, b: &Body, qt: &QuadTree) -> Vector2 {
        match qt {
            QuadTree::Leaf(leaf) => {
                if b.id != leaf.body.id {
                    calc_pull(b, &leaf.body)
                } else {
                    Vector2::zero()
                }
            }
            QuadTree::Root(root) => {
                let s = root.boundary.size;
                let d = b.pos.distance(root.center_of_mass);
                if s / d < theta {
                    calc_pull_com(b, root.center_of_mass, root.mass)
                } else {
                    let ne = match &root.ne {
                        None => Vector2::zero(),
                        Some(qt2) => Simulation::apply_forces(theta, b, qt2.as_ref()),
                    };
                    let se = match &root.se {
                        None => Vector2::zero(),
                        Some(qt2) => Simulation::apply_forces(theta, b, qt2.as_ref()),
                    };
                    let sw = match &root.sw {
                        None => Vector2::zero(),
                        Some(qt2) => Simulation::apply_forces(theta, b, qt2.as_ref()),
                    };
                    let nw = match &root.nw {
                        None => Vector2::zero(),
                        Some(qt2) => Simulation::apply_forces(theta, b, qt2.as_ref()),
                    };
                    ne + se + sw + nw
                }
            }
        }
    }
}
