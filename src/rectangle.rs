use crate::vector::Vector2;
pub struct Rectangle {
    /// Top left corner of rectangle
    pos: Vector2,
    size: f64,
}

impl Rectangle {
    fn inside(&self, p: Vector2) -> bool {
        let s = &self.pos;
        return p.x >= s.x && p.x <= (s.x + self.size) && p.y >= s.y && p.y <= (s.y + self.size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inside() {
        let r = Rectangle {
            pos: Vector2::new(-5.0, -5.0),
            size: 10.0,
        };
        let p1 = Vector2::new(1.0, 4.0);
        let p2 = Vector2::new(-6.0, 4.0);

        assert!(r.inside(p1));
        assert!(!r.inside(p2));
    }
}
