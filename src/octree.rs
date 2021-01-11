use ggez::{
    graphics::{self},
    mint::Point2,
    Context, GameResult,
};

use crate::{
    body::Body,
    cube::{Cube, Region},
    physics_helper::*,
    vector::Vector3,
};

/// A quadtree with a bucket size of one
pub enum OcTree {
    Leaf(Leaf),
    Root(Root),
}

pub struct Leaf {
    boundary: Cube,
    pub body: Body,
}

pub struct Root {
    pub boundary: Cube,
    pub center_of_mass: Vector3,
    pub mass: f64,
    pub tne: Option<Box<OcTree>>,
    pub tse: Option<Box<OcTree>>,
    pub tsw: Option<Box<OcTree>>,
    pub tnw: Option<Box<OcTree>>,
    pub bne: Option<Box<OcTree>>,
    pub bse: Option<Box<OcTree>>,
    pub bsw: Option<Box<OcTree>>,
    pub bnw: Option<Box<OcTree>>,
}

impl OcTree {
    pub fn new(boundary: Cube) -> OcTree {
        OcTree::Root(Root {
            boundary: boundary,
            center_of_mass: Vector3::zero(),
            mass: 0.0,
            tne: None,
            tse: None,
            tsw: None,
            tnw: None,
            bne: None,
            bse: None,
            bsw: None,
            bnw: None,
        })
    }

    fn subdivide(&mut self) {
        match self {
            OcTree::Leaf(Leaf { boundary, body }) => {
                let mut ot = OcTree::new(*boundary);
                ot.insert(*body).ok();
                *self = ot;
            }
            _ => (),
        }
    }

    pub fn insert(&mut self, b1: Body) -> Result<(), &'static str> {
        match self {
            // Occupied leaf, split into root
            OcTree::Leaf(leaf) => {
                if leaf.boundary.contains(&b1.pos) {
                    self.subdivide();
                    return self.insert(b1);
                } else {
                    println!("Tried to insert {:?}", b1.pos);
                    println!("into {:?}", leaf.boundary);
                    return Err("Inserted body is outside boundary");
                }
            }
            OcTree::Root(root) => {
                if root.boundary.contains(&b1.pos) {
                    let region = root.boundary.region(&b1.pos);
                    let node = match region {
                        Region::TNE => &mut root.tne,
                        Region::TSE => &mut root.tse,
                        Region::TSW => &mut root.tsw,
                        Region::TNW => &mut root.tnw,
                        Region::BNE => &mut root.bne,
                        Region::BSE => &mut root.bse,
                        Region::BSW => &mut root.bsw,
                        Region::BNW => &mut root.bnw,
                    };
                    match node {
                        None => {
                            let ot = OcTree::Leaf(Leaf {
                                boundary: root.boundary.region_boundary(region),
                                body: b1,
                            });
                            *node = Some(Box::new(ot));
                        }
                        Some(ot) => {
                            ot.insert(b1)?;
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
    // Currently only draw top and not bottom of octree, waiting on iteration over regions for legible code
    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        match self {
            OcTree::Leaf(leaf) => {
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
            OcTree::Root(_) => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_octree_insert() {
        let mut ot = OcTree::new(Cube {
            pos: Vector3::new(-5.0, -5.0, -5.0),
            size: 10.0,
        });
        let b1 = Body {
            id: 1,
            pos: Vector3::new(4.0, -4.0, 0.0),
            vel: Vector3::zero(),
            acc: Vector3::zero(),
            mass: 1.0,
        };
        let b2 = Body {
            id: 2,
            pos: Vector3::new(3.0, -4.0, 0.0),
            vel: Vector3::zero(),
            acc: Vector3::zero(),
            mass: 10.0,
        };
        assert!(ot.insert(b1).is_ok());
        assert!(ot.insert(b2).is_ok());
        assert_eq!(
            match ot {
                OcTree::Root(root) => root.center_of_mass,
                OcTree::Leaf(_) => panic!("Should be root"),
            },
            // Rounding errors, should be 3.091
            Vector3::new(3.090909090909091, -4.0, 0.0)
        );
    }
}
