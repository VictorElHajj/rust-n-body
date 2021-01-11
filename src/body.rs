use crate::vector::Vector3;

#[derive(Copy, Clone, PartialEq)]
pub struct Body {
    pub id: u32,
    pub pos: Vector3,
    pub vel: Vector3,
    pub acc: Vector3,
    pub mass: f64,
}
