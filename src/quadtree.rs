use crate::{body::Body, physics_helper::*, rectangle::Rectangle, vector::Vector2};

/// A quadtree with a bucket size of one
pub enum QuadTree {
    Leaf(Leaf),
    Root(Root),
}

pub struct Leaf {
    boundary: Rectangle,
    body: Option<Body>,
}

pub struct Root {
    boundary: Rectangle,
    center_of_mass: Vector2,
    mass: f64,
    ne: Option<Box<QuadTree>>,
    se: Option<Box<QuadTree>>,
    sw: Option<Box<QuadTree>>,
    nw: Option<Box<QuadTree>>,
}

impl QuadTree {
    pub fn new(boundary: Rectangle) -> QuadTree {
        QuadTree::Leaf(Leaf {
            boundary: boundary,
            body: None,
        })
    }

    fn subdivide(&mut self) {
        match self {
            QuadTree::Leaf(Leaf { boundary, body }) => {
                let mut qt = QuadTree::Root(Root {
                    boundary: *boundary,
                    center_of_mass: Vector2::new(0.0, 0.0),
                    mass: 0.0,
                    ne: None,
                    se: None,
                    sw: None,
                    nw: None,
                });
                match body {
                    Some(body) => qt.insert(*body),
                    _ => Ok(()),
                }
                .ok();
                *self = qt;
            }
            _ => (),
        }
    }

    pub fn insert(&mut self, b1: Body) -> Result<(), &'static str> {
        match self {
            // Occupied leaf, split into root
            QuadTree::Leaf(Leaf {
                boundary,
                body: Some(_),
            }) => {
                if boundary.contains(&b1.pos) {
                    self.subdivide();
                    return self.insert(b1);
                } else {
                    return Err("Inserted body is outside boundary");
                }
            }
            // Empty leaf, just enter the body
            QuadTree::Leaf(leaf) => {
                if leaf.boundary.contains(&b1.pos) {
                    leaf.body = Some(b1);
                    return Ok(());
                } else {
                    return Err("Inserted body is outside boundary");
                }
            }
            QuadTree::Root(root) => {
                if root.boundary.north_east().contains(&b1.pos) {
                    let qt = QuadTree::Leaf(Leaf {
                        boundary: root.boundary.north_east(),
                        body: Some(b1),
                    });
                    root.ne = Some(Box::new(qt));
                    root.center_of_mass = calc_com(b1.pos, b1.mass, root.center_of_mass, root.mass);
                    root.mass += b1.mass;
                    Ok(())
                } else if root.boundary.south_east().contains(&b1.pos) {
                    let qt = QuadTree::Leaf(Leaf {
                        boundary: root.boundary.south_east(),
                        body: Some(b1),
                    });
                    root.se = Some(Box::new(qt));
                    root.center_of_mass = calc_com(b1.pos, b1.mass, root.center_of_mass, root.mass);
                    root.mass += b1.mass;
                    Ok(())
                } else if root.boundary.south_west().contains(&b1.pos) {
                    let qt = QuadTree::Leaf(Leaf {
                        boundary: root.boundary.south_west(),
                        body: Some(b1),
                    });
                    root.sw = Some(Box::new(qt));
                    root.center_of_mass = calc_com(b1.pos, b1.mass, root.center_of_mass, root.mass);
                    root.mass += b1.mass;
                    Ok(())
                } else if root.boundary.north_west().contains(&b1.pos) {
                    let qt = QuadTree::Leaf(Leaf {
                        boundary: root.boundary.north_west(),
                        body: Some(b1),
                    });
                    root.nw = Some(Box::new(qt));
                    root.center_of_mass = calc_com(b1.pos, b1.mass, root.center_of_mass, root.mass);
                    root.mass += b1.mass;
                    Ok(())
                } else {
                    return Err("Inserted body is outside boundary");
                }
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
            pos: Vector2::new(4.0, -4.0),
            vel: Vector2::zero(),
            mass: 1.0,
        };
        let b2 = Body {
            pos: Vector2::new(3.0, -4.0),
            vel: Vector2::zero(),
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
