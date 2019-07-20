//Based on: https://www.redblobgames.com/grids/hexagons/
//We use pointy top representation

extern crate sfml;

use std::ops::{Add, Neg, Sub};

use sfml::system::Vector2f;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
pub struct HexCoordinates {
    x: i32,
    y: i32,
    z: i32,
}

impl HexCoordinates {
    pub fn new_cube(x: i32, y: i32, z: i32) -> HexCoordinates {
        assert_eq!(x.clone() + y.clone() + z.clone(), 0);
        HexCoordinates { x: x, y: y, z: z }
    }

    pub fn new_axial(q: i32, r: i32) -> HexCoordinates {
        HexCoordinates {
            x: r,
            y: -(q + r),
            z: q,
        }
    }

    pub const DIRECTIONS: [Self; 6] = [
        Self { x: 1, y: -1, z: 0 },
        Self { x: 1, y: 0, z: -1 },
        Self { x: 0, y: 1, z: -1 },
        Self { x: -1, y: 1, z: 0 },
        Self { x: -1, y: 0, z: 1 },
        Self { x: 0, y: -1, z: 1 },
    ];

    pub fn neighbor(&self, direction: usize) -> Self {
        self.clone() + Self::DIRECTIONS[direction]
    }

    pub fn neighbors(&self) -> [Self; 6] {
        let mut dir = Self::DIRECTIONS.clone();
        for x in dir.iter_mut() {
            *x = *x + *self;
        }
        dir
    }

    pub fn origin() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    pub fn distance_to(&self, other: &Self) -> i32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) / 2
    }

    pub fn p(&self) -> i32 {
        self.z
    }

    pub fn q(&self) -> i32 {
        self.x
    }
}

impl Add for HexCoordinates {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Neg for HexCoordinates {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for HexCoordinates {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Orientation {
    //orientation matrix (used in the conversion to pixel point), row major
    m: [f32; 4],
    //inverse of orientation matrix
    minv: [f32; 4],
    //multiples of 60 degrees
    start_angle: f32,
}

const SQRT_3: f32 = 1.732050807568877;

impl Orientation {
    pub const POINTY: Self = Orientation {
        m: [SQRT_3, SQRT_3 / 2.0, 0.0, 3.0 / 2.0],
        minv: [SQRT_3 / 3.0, -1.0 / 3.0, 0.0, 2.0 / 3.0],
        start_angle: 0.5,
    };
    pub const FLAT: Self = Orientation {
        m: [3.0 / 2.0, 0.0, SQRT_3 / 2.0, SQRT_3],
        minv: [2.0 / 3.0, 0.0, -1.0 / 3.0, SQRT_3 / 3.0],
        start_angle: 0.0,
    };
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Layout {
    pub orientation: Orientation,
    pub size: Vector2f,
    pub origin: Vector2f,
}

impl Layout {
    pub fn corner_offset(&self, corner: u32) -> Vector2f {
        let angle =
            2.0 * std::f32::consts::PI * (self.orientation.start_angle + corner as f32) / 6.0;
        Vector2f {
            x: self.size.x * angle.cos(),
            y: self.size.y * angle.sin(),
        }
    }
}

pub fn hex_to_pixel(hex: HexCoordinates, layout: Layout) -> Vector2f {
    let m = &layout.orientation.m;
    let x: f32 = m[0] * (hex.q() as f32) + m[1] * (hex.p() as f32);
    let y: f32 = m[2] * (hex.q() as f32) + m[3] * (hex.p() as f32);

    Vector2f {
        x: x,
        y: y,
    } * layout.size
        + layout.origin
}

pub fn pixel_to_hex(point: Vector2f, layout: Layout) -> HexCoordinates {
    let pt = (point - layout.origin) / layout.size;
    let m = &layout.orientation.minv;
    let q: f32 = m[0] * pt.x + m[1] * pt.y;
    let p: f32 = m[2] * pt.x + m[3] * pt.y;

    let mut rx = q.round();
    let mut ry = (-q - p).round();
    let mut rz = p.round();

    let x_diff = (rx - q).abs();
    let y_diff = (ry - (-q - p)).abs();
    let z_diff = (rz - p).abs();

    if x_diff > y_diff && x_diff > z_diff {
        rx = -ry - rz;
    } else if y_diff > z_diff {
        ry = -rx - rz;
    } else {
        rz = -rx - ry;
    }

    HexCoordinates::new_cube(rx as i32, ry as i32, rz as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        let coord1 = HexCoordinates::new_axial(2, 3);
        let coord2 = HexCoordinates::new_cube(3, -5, 2);
        assert!(coord1 == coord2);
    }

    #[test]
    fn addition() {
        let coord1 = HexCoordinates::new_axial(2, 3);
        let coord2 = HexCoordinates::new_axial(1, 5);
        let coord3 = HexCoordinates::new_axial(3, 8);

        assert_eq!(coord1 + coord2, coord3);
    }

    #[test]
    fn origin() {
        let coord1 = HexCoordinates::origin();
        let coord2 = HexCoordinates::new_axial(0, 0);
        assert_eq!(coord1, coord2);
    }

    #[test]
    fn neighbors() {
        let coord = HexCoordinates::new_cube(2, -1, -1);
        let coord_n = coord.neighbor(3);
        let coord_n_correct = HexCoordinates::new_cube(1, 0, -1);
        assert_eq!(coord_n, coord_n_correct);
    }

    #[test]
    fn distance() {
        let c1 = HexCoordinates::new_cube(1, 0, -1);
        let c2 = HexCoordinates::new_cube(3, -2, -1);
        assert_eq!(c1.distance_to(&c2), 2);
    }

}
