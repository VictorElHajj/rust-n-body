use crate::vector::Vector2;

#[derive(Copy, Clone)]
pub struct Body {
    pub pos: Vector2,
    pub vel: Vector2,
    pub mass: f64,
}
