use crate::vector::Vector2;

#[derive(Copy, Clone, PartialEq)]
pub struct Body {
    pub id: u32,
    pub pos: Vector2,
    pub vel: Vector2,
    pub acc: Vector2,
    pub mass: f64,
}
