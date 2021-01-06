use crate::{body::Body, rectangle::Rectangle, vector::Vector2};

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
    center_of_mass: Vector2,
    mass: f64,
    ne: Box<QuadTree>,
    se: Box<QuadTree>,
    sw: Box<QuadTree>,
    nw: Box<QuadTree>,
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
                    center_of_mass: Vector2::new(0.0, 0.0),
                    mass: 0.0,
                    ne: Box::new(QuadTree::new(boundary.north_east())),
                    se: Box::new(QuadTree::new(boundary.south_east())),
                    sw: Box::new(QuadTree::new(boundary.south_west())),
                    nw: Box::new(QuadTree::new(boundary.north_west())),
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

    pub fn insert(&mut self, b1: Body) -> Result<(), String> {
        match self {
            // Occupied leaf, split into root
            QuadTree::Leaf(Leaf {
                boundary,
                body: Some(_),
            }) => {
                if boundary.inside(&b1.pos) {
                    self.subdivide();
                    return self.insert(b1);
                } else {
                    return Err(String::from("Inserted body is outside boundary"));
                }
            }
            // Empty leaf, just enter the body
            QuadTree::Leaf(leaf) => {
                if leaf.boundary.inside(&b1.pos) {
                    leaf.body = Some(b1);
                    return Ok(());
                } else {
                    return Err(String::from("Inserted body is outside boundary"));
                }
            }
            QuadTree::Root(root) => {
                if root.ne.insert(b1).is_ok() {
                    root.center_of_mass = Vector2::new(
                        (b1.pos.x * b1.mass + root.mass * root.center_of_mass.x)
                            / (root.mass + b1.mass),
                        (b1.pos.y * b1.mass + root.mass * root.center_of_mass.y)
                            / (root.mass + b1.mass),
                    );
                    root.mass += b1.mass;
                    return Ok(());
                } else if root.se.insert(b1).is_ok() {
                    return Ok(());
                } else if root.sw.insert(b1).is_ok() {
                    return Ok(());
                } else if root.nw.insert(b1).is_ok() {
                    return Ok(());
                } else {
                    return Err(String::from("Inserted is outside boundary"));
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
            vel: Vector2::new(0.0, 0.0),
            mass: 1.0,
        };
        let b2 = Body {
            pos: Vector2::new(3.0, -4.0),
            vel: Vector2::new(0.0, 0.0),
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
