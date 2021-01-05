use crate::{body::body, rectangle::Rectangle, vector::vector2};

/// A quadtree with a bucketsize of one
pub enum QuadTree {
    Leaf {
        boundary: Rectangle,
        body: Option<body>,
    },
    Root {
        boundary: Rectangle,
        center_of_mass: vector2,
        mass: f64,
        ne: Box<QuadTree>,
        se: Box<QuadTree>,
        sw: Box<QuadTree>,
        nw: Box<QuadTree>,
    },
}
