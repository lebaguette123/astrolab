use std::ops::{Add, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vec3{
    pub fn new(x: f64, y: f64, z: f64) -> Vec3{
        Vec3 { x, y, z }
    }
    pub const ZERO: Self = Self{ x: 0.0, y: 0.0, z: 0.0 };

    pub fn dot(&self, other: Self) -> f64{
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn magnitude(&self) -> f64{
        self.dot(*self).sqrt()
    }

    pub fn normalize(&self) -> Self{
        let mag = self.magnitude();
        assert!(mag > 0.0, "Cannot normalize a zero-length vector");
        *self * (1.0 / mag)
    }

    pub fn cross(&self, other: Self) -> Self{
        Self{
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}
impl Add for Vec3{
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self{
        Self{ x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}
impl Sub for Vec3{
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self{
        Self{ x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}
impl Neg for Vec3{
    type Output = Vec3;
    fn neg(self) -> Self{
        Self{ x: -self.x, y: -self.y, z: -self.z }
    }
}
impl Mul<f64> for Vec3{
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self{
        Self{ x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_add(){
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let result = v1 + v2;
        assert_eq!(result, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_sub(){
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let result = v1 - v2;
        assert_eq!(result, Vec3::new(-3.0, -3.0, -3.0));
    }

    #[test]
    fn test_mul(){
        let v = Vec3::new(1.0, 2.0, 3.0);
        let result = v * 2.0;
        let result1 = v * -0.5;
        assert_eq!(result, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(result1, Vec3::new(-0.5, -1.0, -1.5));
    }

    #[test]
    fn test_neg(){
        let v = Vec3::new(1.0, -2.0, 3.0);
        let result = -v;
        assert_eq!(result, Vec3::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_dot(){
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let v3 = Vec3::new(1.0, 2.0, 0.0);
        let v4 = Vec3::new(-2.0, 1.0, 0.0);
        let result = v1.dot(v2);
        assert_eq!(v1.dot(v2), v2.dot(v1));
        assert_eq!(v1.dot(v1), 14.0);
        assert_eq!(result, 32.0);
        assert_eq!(v3.dot(v4), 0.0);
    }

    #[test]
    fn test_magnitude(){
        let v = Vec3::new(3.0, 4.0, 0.0);
        let v1 = Vec3::new(1.0, 2.0, 2.0);
        let result1 = v1.magnitude();
        assert_eq!(result1, 3.0);
        let result = v.magnitude();
        assert_eq!(result, 5.0);
        let n = Vec3::new(3.0, 4.0, 0.0).normalize();
        assert!((n.magnitude()-1.0).abs() < 1e-12);
    }

    #[test]
    fn test_normalize(){
        let v1 = Vec3::new(0.0,0.0,0.5);
        assert_eq!(v1.normalize(), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    #[should_panic(expected = "zero-length")]
    fn test_normalize_zero_panics(){
        Vec3::ZERO.normalize();
    }

    #[test]
    fn test_cross(){
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);
        let v3 = Vec3::new(0.0, 0.0, 1.0);

        assert_eq!(v1.cross(v2), v3);
        assert_eq!(v2.cross(v3), v1);
        assert_eq!(v3.cross(v1), v2);

        let v4 = Vec3::new(1.0, 2.0, 3.0);
        let v5 = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(v4.cross(v5), Vec3::ZERO);

        let v6 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v4.cross(v6), Vec3::new(-3.0, 6.0, -3.0));

        let radius = Vec3::new(7e6, 0.0, 0.0);
        let velocity = Vec3::new(0.0, 7.5e3, 0.0);
        assert_eq!(radius.cross(velocity), Vec3::new(0.0, 0.0, 5.25e10));
    }
}