use crate::vector::Vector2;

/// Calculate center of mass
#[inline]
pub fn calc_com(pos1: Vector2, mass1: f64, pos2: Vector2, mass2: f64) -> Vector2 {
    Vector2::new(
        (pos1.x * mass1 + mass2 * pos2.x) / (mass1 + mass2),
        (pos1.y * mass1 + mass2 * pos2.y) / (mass1 + mass2),
    )
}
