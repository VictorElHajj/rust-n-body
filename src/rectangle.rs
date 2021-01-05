use crate::vector::Vector2;
#[derive(PartialEq, Debug)]
pub struct Rectangle {
    /// Top left corner of rectangle
    pub pos: Vector2,
    pub size: f64,
}

impl Rectangle {
    pub fn inside(&self, p: &Vector2) -> bool {
        let s = &self.pos;
        return p.x >= s.x && p.x <= (s.x + self.size) && p.y >= s.y && p.y <= (s.y + self.size);
    }

    pub fn ne(&self) -> Rectangle {
        Rectangle {
            pos: Vector2::new(self.pos.x + self.size / 2.0, self.pos.y),
            size: self.size / 2.0,
        }
    }

    pub fn se(&self) -> Rectangle {
        Rectangle {
            pos: Vector2::new(self.pos.x + self.size / 2.0, self.pos.y + self.size / 2.0),
            size: self.size / 2.0,
        }
    }

    pub fn sw(&self) -> Rectangle {
        Rectangle {
            pos: Vector2::new(self.pos.x, self.pos.y + self.size / 2.0),
            size: self.size / 2.0,
        }
    }

    pub fn nw(&self) -> Rectangle {
        Rectangle {
            pos: Vector2::new(self.pos.x, self.pos.y),
            size: self.size / 2.0,
        }
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

        assert!(r.inside(&p1));
        assert!(!r.inside(&p2));
    }

    #[test]
    fn test_square_split() {
        let r = Rectangle {
            pos: Vector2::new(-5.0, -5.0),
            size: 10.0,
        };
        let ne = Rectangle {
            pos: Vector2::new(0.0, -5.0),
            size: 5.0,
        };
        let se = Rectangle {
            pos: Vector2::new(0.0, 0.0),
            size: 5.0,
        };
        let sw = Rectangle {
            pos: Vector2::new(-5.0, 0.0),
            size: 5.0,
        };
        let nw = Rectangle {
            pos: Vector2::new(-5.0, -5.0),
            size: 5.0,
        };
        assert_eq!(ne, r.ne());
        assert_eq!(se, r.se());
        assert_eq!(sw, r.sw());
        assert_eq!(nw, r.nw());
    }
}
