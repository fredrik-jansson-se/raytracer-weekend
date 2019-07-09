use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, Neg, Sub};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn zero() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn r(&self) -> f32 {
        self.e[0]
    }
    pub fn g(&self) -> f32 {
        self.e[1]
    }
    pub fn b(&self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2])
    }

    pub fn squared_length(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn make_unit_vector(&mut self) {
        let k = 1.0 / self.length();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Self::Output {
        Vec3 {
            e: [self.e[0] / other, self.e[1] / other, self.e[2] / other],
        }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        self.e[0] /= other;
        self.e[1] /= other;
        self.e[2] /= other;
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, idx: usize) -> &f32 {
        &self.e[idx]
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Self::Output {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            e: [self.e[0] * other, self.e[1] * other, self.e[2] * other],
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3 {
            e: [other.e[0] * self, other.e[1] * self, other.e[2] * self],
        }
    }
}

impl Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Self::Output {
        Vec3 {
            e: [other.e[0] * self, other.e[1] * self, other.e[2] * self],
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Vec3>>(iter: I) -> Vec3 {
        iter.fold(Vec3::zero(), |acc, v| acc + v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(
            Vec3::new(0.0, 0.0, 0.0) + Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(1.0, 1.0, 1.0)
        );
    }

    #[test]
    fn mul() {
        assert_eq!(
            Vec3::new(2.0, 2.0, 2.0) * Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(2.0, 4.0, 6.0)
        );
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) * 2.0, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn div() {
        assert_eq!(Vec3::new(2.0, 4.0, 6.0) / 2.0, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn neg() {
        assert_eq!(-Vec3::new(2.0, 2.0, 2.0), Vec3::new(-2.0, -2.0, -2.0));
    }
}
