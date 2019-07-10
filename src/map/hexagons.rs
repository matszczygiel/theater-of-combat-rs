//Based on: https://www.redblobgames.com/grids/hexagons/
//We use pointy top representation

extern crate sfml;

use std::ops::{Add, Neg, Sub};

use sfml::system::Vector2i;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
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

    pub fn new_axial(p: i32, q: i32) -> HexCoordinates {
        HexCoordinates {
            x: q,
            y: -(p + q),
            z: p,
        }
    }

    const DIRECTIONS: [Self; 6] = [
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

pub fn hex_to_pixel(hex: HexCoordinates, size: f32) -> Vector2i {
    let x: f32 =
        size * ((3.0f32).sqrt() * (hex.q() as f32) + (3.0f32).sqrt() / 2.0 * (hex.p() as f32));
    let y: f32 = size * (3.0 / 2.0 * (hex.p() as f32));
    Vector2i {
        x: x.round() as i32,
        y: y.round() as i32,
    }
}

pub fn pixel_to_hex(point: Vector2i, size: f32) -> HexCoordinates {
    let q: f32 =
        ((3.0f32).sqrt() / 3.0f32 * (point.x as f32) - 1.0f32 / 3.0f32 * (point.y as f32)) / size;
    let p: f32 = (2.0f32 / 3.0f32 * (point.y as f32)) / size;

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
