use crate::vector::Vector3;
use Region::*;
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Cube {
    /// Top left corner of rectangle
    pub pos: Vector3,
    pub size: f64,
}

#[derive(PartialEq, Debug)]
pub enum Region {
    TNE,
    TSE,
    TSW,
    TNW,
    BNE,
    BSE,
    BSW,
    BNW,
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

    pub fn region(&self, p: &Vector3) -> Region {
        let s = &self.pos;
        let half = self.size / 2.0;
        if p.x >= s.x + half {
            if p.y >= s.y + half {
                if p.z >= s.z + half {
                    Region::BSE
                } else {
                    Region::TSE
                }
            } else {
                if p.z >= s.z + half {
                    Region::BNE
                } else {
                    Region::TNE
                }
            }
        } else {
            if p.y >= s.y + half {
                if p.z >= s.z + half {
                    Region::BSW
                } else {
                    Region::TSW
                }
            } else {
                if p.z >= s.z + half {
                    Region::BNW
                } else {
                    Region::TNW
                }
            }
        }
    }

    pub fn region_boundary(&self, region: Region) -> Cube {
        match region {
            TNE => Cube {
                pos: Vector3::new(self.pos.x + self.size / 2.0, self.pos.y, self.pos.z),
                size: self.size / 2.0,
            },
            TSE => Cube {
                pos: Vector3::new(
                    self.pos.x + self.size / 2.0,
                    self.pos.y + self.size / 2.0,
                    self.pos.z,
                ),
                size: self.size / 2.0,
            },
            TSW => Cube {
                pos: Vector3::new(self.pos.x, self.pos.y + self.size / 2.0, self.pos.z),
                size: self.size / 2.0,
            },
            TNW => Cube {
                pos: Vector3::new(self.pos.x, self.pos.y, self.pos.z),
                size: self.size / 2.0,
            },
            BNE => Cube {
                pos: Vector3::new(
                    self.pos.x + self.size / 2.0,
                    self.pos.y,
                    self.pos.z + self.size / 2.0,
                ),
                size: self.size / 2.0,
            },
            BSE => Cube {
                pos: Vector3::new(
                    self.pos.x + self.size / 2.0,
                    self.pos.y + self.size / 2.0,
                    self.pos.z + self.size / 2.0,
                ),
                size: self.size / 2.0,
            },
            BSW => Cube {
                pos: Vector3::new(
                    self.pos.x,
                    self.pos.y + self.size / 2.0,
                    self.pos.z + self.size / 2.0,
                ),
                size: self.size / 2.0,
            },
            BNW => Cube {
                pos: Vector3::new(self.pos.x, self.pos.y, self.pos.z + self.size / 2.0),
                size: self.size / 2.0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inside() {
        let c = Cube {
            pos: Vector3::new(-5.0, -5.0, -5.0),
            size: 10.0,
        };
        let p1 = Vector3::new(1.0, 4.0, -3.0);
        let p2 = Vector3::new(-6.0, 4.0, 3.0);

        assert!(c.contains(&p1));
        assert!(!c.contains(&p2));
    }

    #[test]
    fn test_region() {
        let c = Cube {
            pos: Vector3::new(-5.0, -5.0, -5.0),
            size: 10.0,
        };
        let p1 = Vector3::new(1.0, -4.0, -4.0);
        let p2 = Vector3::new(1.0, 4.0, -4.0);
        let p3 = Vector3::new(-1.0, 4.0, -4.0);
        let p4 = Vector3::new(-1.0, -4.0, -4.0);

        let p5 = Vector3::new(1.0, -4.0, 4.0);
        let p6 = Vector3::new(1.0, 4.0, 4.0);
        let p7 = Vector3::new(-1.0, 4.0, 4.0);
        let p8 = Vector3::new(-1.0, -4.0, 4.0);

        assert_eq!(c.region(&p1), TNE);
        assert_eq!(c.region(&p2), TSE);
        assert_eq!(c.region(&p3), TSW);
        assert_eq!(c.region(&p4), TNW);

        assert_eq!(c.region(&p5), BNE);
        assert_eq!(c.region(&p6), BSE);
        assert_eq!(c.region(&p7), BSW);
        assert_eq!(c.region(&p8), BNW);
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
        assert_eq!(tne, c.region_boundary(Region::TNE));
        assert_eq!(tse, c.region_boundary(Region::TSE));
        assert_eq!(tsw, c.region_boundary(Region::TSW));
        assert_eq!(tnw, c.region_boundary(Region::TNW));
        assert_eq!(bne, c.region_boundary(Region::BNE));
        assert_eq!(bse, c.region_boundary(Region::BSE));
        assert_eq!(bsw, c.region_boundary(Region::BSW));
        assert_eq!(bnw, c.region_boundary(Region::BNW));
    }
}
