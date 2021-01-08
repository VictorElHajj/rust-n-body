use crate::{body::Body, vector::Vector2};

/// Calculate center of mass
#[inline]
pub fn calc_com(pos1: Vector2, mass1: f64, pos2: Vector2, mass2: f64) -> Vector2 {
    Vector2::new(
        (pos1.x * mass1 + mass2 * pos2.x) / (mass1 + mass2),
        (pos1.y * mass1 + mass2 * pos2.y) / (mass1 + mass2),
    )
}

/// Not G, which is applied last as optimization
pub fn calc_pull(b1: &Body, b2: &Body) -> Vector2 {
    (b2.pos - b1.pos) / (b1.pos.distance(b2.pos)).powi(3) * (b1.mass * b2.mass)
}

pub fn calc_pull_com(b1: &Body, b2_pos: Vector2, b2_mass: f64) -> Vector2 {
    (b2_pos - b1.pos) / (b1.pos.distance(b2_pos)).powi(3) * (b1.mass * b2_mass)
}
