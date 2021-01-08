use ggez::{
    graphics::{self},
    mint::Point2,
    Context, GameResult,
};

use crate::{body::Body, physics_helper::*, rectangle::Rectangle, vector::Vector2};

/// A quadtree with a bucket size of one
pub enum QuadTree {
    Leaf(Leaf),
    Root(Root),
}

pub struct Leaf {
    boundary: Rectangle,
    pub body: Body,
}

pub struct Root {
    pub boundary: Rectangle,
    pub center_of_mass: Vector2,
    pub mass: f64,
    pub ne: Option<Box<QuadTree>>,
    pub se: Option<Box<QuadTree>>,
    pub sw: Option<Box<QuadTree>>,
    pub nw: Option<Box<QuadTree>>,
}

impl QuadTree {
    pub fn new(boundary: Rectangle) -> QuadTree {
        QuadTree::Root(Root {
            boundary: boundary,
            center_of_mass: Vector2::zero(),
            mass: 0.0,
            ne: None,
            se: None,
            sw: None,
            nw: None,
        })
    }

    fn subdivide(&mut self) {
        match self {
            QuadTree::Leaf(Leaf { boundary, body }) => {
                let mut qt = QuadTree::new(*boundary);
                qt.insert(*body).ok();
                *self = qt;
            }
            _ => (),
        }
    }

    pub fn insert(&mut self, b1: Body) -> Result<(), &'static str> {
        match self {
            // Occupied leaf, split into root
            QuadTree::Leaf(leaf) => {
                if leaf.boundary.contains(&b1.pos) {
                    self.subdivide();
                    return self.insert(b1);
                } else {
                    return Err("Inserted body is outside boundary");
                }
            }
            QuadTree::Root(root) => {
                if root.boundary.north_east().contains(&b1.pos) {
                    match &mut root.ne {
                        None => {
                            let qt = QuadTree::Leaf(Leaf {
                                boundary: root.boundary.north_east(),
                                body: b1,
                            });
                            root.ne = Some(Box::new(qt));
                        }
                        Some(qt) => {
                            qt.insert(b1)?;
                        }
                    }
                    root.center_of_mass = calc_com(b1.pos, b1.mass, root.center_of_mass, root.mass);
                    root.mass += b1.mass;
                    Ok(())
                } else if root.boundary.south_east().contains(&b1.pos) {
                    match &mut root.se {
                        None => {
                            let qt = QuadTree::Leaf(Leaf {
                                boundary: root.boundary.south_east(),
                                body: b1,
                            });
                            root.se = Some(Box::new(qt));
                        }
                        Some(qt) => {
                            qt.insert(b1)?;
                        }
                    }
                    root.center_of_mass = calc_com(b1.pos, b1.mass, root.center_of_mass, root.mass);
                    root.mass += b1.mass;
                    Ok(())
                } else if root.boundary.south_west().contains(&b1.pos) {
                    match &mut root.sw {
                        None => {
                            let qt = QuadTree::Leaf(Leaf {
                                boundary: root.boundary.south_west(),
                                body: b1,
                            });
                            root.sw = Some(Box::new(qt));
                        }
                        Some(qt) => {
                            qt.insert(b1)?;
                        }
                    }
                    root.center_of_mass = calc_com(b1.pos, b1.mass, root.center_of_mass, root.mass);
                    root.mass += b1.mass;
                    Ok(())
                } else if root.boundary.north_west().contains(&b1.pos) {
                    match &mut root.nw {
                        None => {
                            let qt = QuadTree::Leaf(Leaf {
                                boundary: root.boundary.north_west(),
                                body: b1,
                            });
                            root.nw = Some(Box::new(qt));
                        }
                        Some(qt) => {
                            qt.insert(b1)?;
                        }
                    }
                    root.center_of_mass = calc_com(b1.pos, b1.mass, root.center_of_mass, root.mass);
                    root.mass += b1.mass;
                    Ok(())
                } else {
                    return Err("Inserted body is outside boundary");
                }
            }
        }
    }

    // Only for visualizing quad tree, expensive and inefficient.
    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        match self {
            QuadTree::Leaf(leaf) => {
                // Draw boundary
                let bounds = graphics::Rect::new(
                    leaf.boundary.pos.x as f32,
                    leaf.boundary.pos.y as f32,
                    leaf.boundary.size as f32,
                    leaf.boundary.size as f32,
                );
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::stroke(1.0),
                    bounds,
                    graphics::WHITE,
                )?;
                graphics::draw(
                    ctx,
                    &rectangle,
                    (ggez::mint::Point2 { x: 500.0, y: 500.0 },),
                )?;
                // Draw body
                let circle = graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::fill(),
                    Point2 {
                        x: leaf.body.pos.x as f32,
                        y: leaf.body.pos.y as f32,
                    },
                    2.0,
                    1.0,
                    graphics::WHITE,
                )?;
                graphics::draw(ctx, &circle, (ggez::mint::Point2 { x: 500.0, y: 500.0 },))?;
                Ok(())
            }
            QuadTree::Root(root) => {
                match &root.ne {
                    None => {
                        let r = root.boundary.north_east();
                        let bounds = graphics::Rect::new(
                            r.pos.x as f32,
                            r.pos.y as f32,
                            r.size as f32,
                            r.size as f32,
                        );
                        let rectangle = graphics::Mesh::new_rectangle(
                            ctx,
                            graphics::DrawMode::stroke(1.0),
                            bounds,
                            graphics::WHITE,
                        )?;
                        graphics::draw(
                            ctx,
                            &rectangle,
                            (ggez::mint::Point2 { x: 500.0, y: 500.0 },),
                        )
                    }
                    Some(qt) => qt.draw(ctx),
                }
                .ok();
                match &root.se {
                    None => {
                        let r = root.boundary.south_east();
                        let bounds = graphics::Rect::new(
                            r.pos.x as f32,
                            r.pos.y as f32,
                            r.size as f32,
                            r.size as f32,
                        );
                        let rectangle = graphics::Mesh::new_rectangle(
                            ctx,
                            graphics::DrawMode::stroke(1.0),
                            bounds,
                            graphics::WHITE,
                        )?;
                        graphics::draw(
                            ctx,
                            &rectangle,
                            (ggez::mint::Point2 { x: 500.0, y: 500.0 },),
                        )
                    }
                    Some(qt) => qt.draw(ctx),
                }
                .ok();
                match &root.sw {
                    None => {
                        let r = root.boundary.south_west();
                        let bounds = graphics::Rect::new(
                            r.pos.x as f32,
                            r.pos.y as f32,
                            r.size as f32,
                            r.size as f32,
                        );
                        let rectangle = graphics::Mesh::new_rectangle(
                            ctx,
                            graphics::DrawMode::stroke(1.0),
                            bounds,
                            graphics::WHITE,
                        )?;
                        graphics::draw(
                            ctx,
                            &rectangle,
                            (ggez::mint::Point2 { x: 500.0, y: 500.0 },),
                        )
                    }
                    Some(qt) => qt.draw(ctx),
                }
                .ok();
                match &root.nw {
                    None => {
                        let r = root.boundary.north_west();
                        let bounds = graphics::Rect::new(
                            r.pos.x as f32,
                            r.pos.y as f32,
                            r.size as f32,
                            r.size as f32,
                        );
                        let rectangle = graphics::Mesh::new_rectangle(
                            ctx,
                            graphics::DrawMode::stroke(1.0),
                            bounds,
                            graphics::WHITE,
                        )?;
                        graphics::draw(
                            ctx,
                            &rectangle,
                            (ggez::mint::Point2 { x: 500.0, y: 500.0 },),
                        )
                    }
                    Some(qt) => qt.draw(ctx),
                }
                .ok();
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadtree_insert() {
        let mut qt = QuadTree::new(Rectangle {
            pos: Vector2::new(-5.0, -5.0),
            size: 10.0,
        });
        let b1 = Body {
            id: 1,
            pos: Vector2::new(4.0, -4.0),
            vel: Vector2::zero(),
            acc: Vector2::zero(),
            mass: 1.0,
        };
        let b2 = Body {
            id: 2,
            pos: Vector2::new(3.0, -4.0),
            vel: Vector2::zero(),
            acc: Vector2::zero(),
            mass: 10.0,
        };
        assert!(qt.insert(b1).is_ok());
        assert!(qt.insert(b2).is_ok());
        assert_eq!(
            match qt {
                QuadTree::Root(root) => root.center_of_mass,
                QuadTree::Leaf(_) => panic!("Should be root"),
            },
            // Rounding errors, should be 3.091
            Vector2::new(3.090909090909091, -4.0)
        );
    }
}
