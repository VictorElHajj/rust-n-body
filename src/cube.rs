use crate::vector::Vector3;
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Cube {
    /// Top left corner of rectangle
    pub pos: Vector3,
    pub size: f64,
}

impl Cube {
    pub fn contains(&self, p: &Vector3) -> bool {
        let s = &self.pos;
        p.x >= s.x
            && p.x <= (s.x + self.size)
            && p.y >= s.y
            && p.y <= (s.y + self.size)
            && p.z >= s.z
            && p.z <= (s.z + self.size)
    }

    pub fn top_north_east(&self) -> Cube {
        Cube {
            pos: Vector3::new(self.pos.x + self.size / 2.0, self.pos.y, self.pos.z),
            size: self.size / 2.0,
        }
    }

    pub fn top_south_east(&self) -> Cube {
        Cube {
            pos: Vector3::new(
                self.pos.x + self.size / 2.0,
                self.pos.y + self.size / 2.0,
                self.pos.z,
            ),
            size: self.size / 2.0,
        }
    }

    pub fn top_south_west(&self) -> Cube {
        Cube {
            pos: Vector3::new(self.pos.x, self.pos.y + self.size / 2.0, self.pos.z),
            size: self.size / 2.0,
        }
    }

    pub fn top_north_west(&self) -> Cube {
        Cube {
            pos: Vector3::new(self.pos.x, self.pos.y, self.pos.z),
            size: self.size / 2.0,
        }
    }

    pub fn bottom_north_east(&self) -> Cube {
        Cube {
            pos: Vector3::new(
                self.pos.x + self.size / 2.0,
                self.pos.y,
                self.pos.z + self.size / 2.0,
            ),
            size: self.size / 2.0,
        }
    }

    pub fn bottom_south_east(&self) -> Cube {
        Cube {
            pos: Vector3::new(
                self.pos.x + self.size / 2.0,
                self.pos.y + self.size / 2.0,
                self.pos.z + self.size / 2.0,
            ),
            size: self.size / 2.0,
        }
    }

    pub fn bottom_south_west(&self) -> Cube {
        Cube {
            pos: Vector3::new(
                self.pos.x,
                self.pos.y + self.size / 2.0,
                self.pos.z + self.size / 2.0,
            ),
            size: self.size / 2.0,
        }
    }

    pub fn bottom_north_west(&self) -> Cube {
        Cube {
            pos: Vector3::new(self.pos.x, self.pos.y, self.pos.z + self.size / 2.0),
            size: self.size / 2.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inside() {
        let r = Cube {
            pos: Vector3::new(-5.0, -5.0, 0.0),
            size: 10.0,
        };
        let p1 = Vector3::new(1.0, 4.0, 0.0);
        let p2 = Vector3::new(-6.0, 4.0, 0.0);

        assert!(r.contains(&p1));
        assert!(!r.contains(&p2));
    }

    #[test]
    fn test_square_split() {
        let c = Cube {
            pos: Vector3::new(-5.0, -5.0, -5.0),
            size: 10.0,
        };
        let tne = Cube {
            pos: Vector3::new(0.0, -5.0, -5.0),
            size: 5.0,
        };
        let tse = Cube {
            pos: Vector3::new(0.0, 0.0, -5.0),
            size: 5.0,
        };
        let tsw = Cube {
            pos: Vector3::new(-5.0, 0.0, -5.0),
            size: 5.0,
        };
        let tnw = Cube {
            pos: Vector3::new(-5.0, -5.0, -5.0),
            size: 5.0,
        };
        let bne = Cube {
            pos: Vector3::new(0.0, -5.0, 0.0),
            size: 5.0,
        };
        let bse = Cube {
            pos: Vector3::new(0.0, 0.0, 0.0),
            size: 5.0,
        };
        let bsw = Cube {
            pos: Vector3::new(-5.0, 0.0, 0.0),
            size: 5.0,
        };
        let bnw = Cube {
            pos: Vector3::new(-5.0, -5.0, 0.0),
            size: 5.0,
        };
        assert_eq!(tne, c.top_north_east());
        assert_eq!(tse, c.top_south_east());
        assert_eq!(tsw, c.top_south_west());
        assert_eq!(tnw, c.top_north_west());
        assert_eq!(bne, c.bottom_north_east());
        assert_eq!(bse, c.bottom_south_east());
        assert_eq!(bsw, c.bottom_south_west());
        assert_eq!(bnw, c.bottom_north_west());
    }
}
