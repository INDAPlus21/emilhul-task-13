use std::ops;
use rand::Rng;

/// ## Vector3
/// Representation of a 3-vector. Implements common 3-vector math functions
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    /// ## new
    /// Returns a Vector3 with x, y, z given as arguments
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 {
            x: x,
            y: y,
            z: z,
        }
    }

    /// ## normal
    /// Returns the normal of this Vector3 as a f32 value
    pub fn normal(&self) -> f32 {
        (self.x.powf(2.0) +
         self.y.powf(2.0) +
         self.z.powf(2.0)).sqrt()
    }

    /// ## unit_vec
    /// Returns a unit vector by normalizing this Vector3
    pub fn unit_vec(&self) -> Vector3 {
        let normal: f32 = self.normal();
        if normal == 0.0 {
            panic!("Can't divide by zero") // Ignores IEEE 754 standard for division with floating numbers
        }
        Vector3 {
            x: self.x / normal,
            y: self.y / normal,
            z: self.z / normal,
        }
    }

    /// ## cross
    /// Returns the cross product of this Vector3 and another given Vector3
    pub fn cross(&self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x,
        }
    }
    /// ## dot
    /// Returns the dot product as a f32 value of this Vector3 and another given Vector3
    pub fn dot(&self, other: Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// ## entrywise
    /// Returns the entrywise product of this Vector3 and another given Vector3
    pub fn entrywise(&self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    /// ## random_in_unit
    /// Returns a random vector withing a unit
    pub fn random_in_unit() -> Vector3 {
        let mut rng = rand::thread_rng();
        loop {
            let p: Vector3 = Vector3::new(rng.gen_range(-1.0..1.0),
                                          rng.gen_range(-1.0..1.0),
                                          rng.gen_range(-1.0..1.0)) * 2.0
                                 - Vector3::new(1.0, 1.0, 1.0);
            if p.normal().powf(2.0) < 1.0 {
                return p;
            }
        }
    }
}

// Operator overloading for Vector3 math
impl ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        *self = Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Self) -> Self {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, other: Vector3) {
        *self = Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f32) -> Vector3{
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, other: f32) {
        *self = Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

// To avoid unexpected issues IEEE 754 standard for floating point numbers will be ignored
impl ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, other: f32) -> Vector3 {
        if other == 0.0 {
            panic!("Can't divide by zero")
        }
        Vector3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl ops::DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, other: f32) {
        if other == 0.0 {
            panic!("Can't divide by zero")
        }
        *self = Vector3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

/// ## Color
/// Special Vector3 where x, y, z, represent r, g, b, of a color
pub type Color = Vector3;

/// Tests for Vector3 struct
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector3_new() {
        let a = Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        }; 
        assert_eq!(a, Vector3::new(1.0, 2.0, 3.0))
    }

    #[test]
    fn vector3_add() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(3.0, 2.0, 1.0);
        let c = Vector3::new(4.0, 4.0, 4.0);

        assert_eq!(c, a + b)
    }

    #[test]
    fn vector3_add_negative() {
        let a = Vector3::new(1.0, -2.0, 3.0);
        let b = Vector3::new(-3.0, 2.0, -1.0);
        let c = Vector3::new(-2.0, 0.0, 2.0);

        assert_eq!(c, a + b)
    }

    #[test]
    fn vector3_add_zero() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(0.0, 0.0, 0.0);
        let c = Vector3::new(1.0, 2.0, 3.0);

        assert_eq!(c, a + b)
    }

    #[test]
    fn vector3_add_assign() {
        let mut a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(3.0, 2.0, 1.0);
        let c = Vector3::new(4.0, 4.0, 4.0);
        a += b;

        assert_eq!(c, a)
    }

    #[test]
    fn vector3_sub() {
        let a = Vector3::new(4.0, 4.0, 4.0);
        let b = Vector3::new(3.0, 2.0, 1.0);
        let c = Vector3::new(1.0, 2.0, 3.0);

        assert_eq!(c, a - b)
    }

    #[test]
    fn vector3_sub_negative() {
        let a = Vector3::new(-4.0, 4.0, 4.0);
        let b = Vector3::new(3.0, -2.0, -1.0);
        let c = Vector3::new(-7.0, 6.0, 5.0);

        assert_eq!(c, a - b)
    }

    #[test]
    fn vector3_sub_zero() {
        let a = Vector3::new(4.0, 4.0, 4.0);
        let b = Vector3::new(0.0, 0.0, 0.0);
        let c = Vector3::new(4.0, 4.0, 4.0);

        assert_eq!(c, a - b)
    }

    #[test]
    fn vector3_sub_assign() {
        let mut a = Vector3::new(4.0, 4.0, 4.0);
        let b = Vector3::new(3.0, 2.0, 1.0);
        let c = Vector3::new(1.0, 2.0, 3.0);
        a -= b;

        assert_eq!(c, a)
    }

    #[test]
    fn vector3_scalar_mul() {
        let a = Vector3::new(4.0, 6.0, 2.0);
        let b: f32 = 0.5;
        let c = Vector3::new(2.0, 3.0, 1.0);

        assert_eq!(c, a * b)
    }

    #[test]
    fn vector3_scalar_mul_negative() {
        let a = Vector3::new(4.0, 6.0, 2.0);
        let b: f32 = -2.0;
        let c = Vector3::new(-8.0, -12.0, -4.0);

        assert_eq!(c, a * b)
    }

    #[test]
    fn vector3_scalar_mul_zero() {
        let a = Vector3::new(4.0, 6.0, 2.0);
        let b: f32 = 0.0;
        let c = Vector3::new(0.0, 0.0, 0.0);

        assert_eq!(c, a * b)
    }

    #[test]
    fn vector3_scalar_mul_vec_zero() {
        let a = Vector3::new(0.0, 0.0, 0.0);
        let b: f32 = 1.5;
        let c = Vector3::new(0.0, 0.0, 0.0);

        assert_eq!(c, a * b)
    }

    #[test]
    fn vector3_scalar_mul_assign() {
        let mut a = Vector3::new(4.0, 6.0, 2.0);
        let b: f32 = 0.5;
        let c = Vector3::new(2.0, 3.0, 1.0);
        a *= b;

        assert_eq!(c, a)
    }

    #[test]
    fn vector3_scalar_div() {
        let a = Vector3::new(4.0, 6.0, 2.0);
        let b: f32 = 0.5;
        let c = Vector3::new(8.0, 12.0, 4.0);

        assert_eq!(c, a / b)
    }

    #[test]
    fn vector3_scalar_div_negative() {
        let a = Vector3::new(4.0, 6.0, -2.0);
        let b: f32 = -2.0;
        let c = Vector3::new(-2.0, -3.0, 1.0);

        assert_eq!(c, a / b)
    }

    #[test]
    fn vector3_scalar_div_zero() {
        let a = Vector3::new(4.0, 6.0, 2.0);
        let b: f32 = 0.0;

        let result = std::panic::catch_unwind(|| a/b );
        assert!(result.is_err());
    }

    #[test]
    fn vector3_scalar_div_vec_zero() {
        let a = Vector3::new(0.0, 0.0, 0.0);
        let b: f32 = 2.0;
        let c = Vector3::new(0.0, 0.0, 0.0);

        assert_eq!(c, a/b);
    }

    #[test]
    fn vector3_scalar_div_assign() {
        let mut a = Vector3::new(4.0, 6.0, 2.0);
        let b: f32 = 0.5;
        let c = Vector3::new(8.0, 12.0, 4.0);
        a /= b;

        assert_eq!(c, a)
    }

    #[test]
    fn vector3_dot() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        let c: f32 = 32.0;

        assert_eq!(c, a.dot(b))
    }

    #[test]
    fn vector3_dot_negative() {
        let a = Vector3::new(-1.0, -2.0, -3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        let c: f32 = -32.0;

        assert_eq!(c, a.dot(b))
    }

    #[test]
    fn vector3_dot_zero() {
        let a = Vector3::new(0.0, 0.0, 0.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        let c: f32 = 0.0;

        assert_eq!(c, a.dot(b))
    }

    #[test]
    fn vector3_cross() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        let c = Vector3::new(-3.0, 6.0, -3.0);

        assert_eq!(c, a.cross(b));
    }

    #[test]
    fn vector3_cross_negative() {
        let a = Vector3::new(-1.0, 2.0, -3.0);
        let b = Vector3::new(4.0, -5.0, 6.0);
        let c = Vector3::new(-3.0, -6.0, -3.0);

        assert_eq!(c, a.cross(b));
    }

    #[test]
    fn vector3_cross_zero() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(0.0, 0.0, 0.0);
        let c = Vector3::new(0.0, 0.0, 0.0);

        assert_eq!(c, a.cross(b));
    }

    #[test]
    fn vector_entrywise() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(4.0, 5.0, 6.0);
        let c = Vector3::new(4.0, 10.0, 18.0);

        assert_eq!(c, a.entrywise(b));
    }

    #[test]
    fn vector_entrywise_negative() {
        let a = Vector3::new(-1.0, 2.0, -3.0);
        let b = Vector3::new(4.0, -5.0, 6.0);
        let c = Vector3::new(-4.0, -10.0, -18.0);

        assert_eq!(c, a.entrywise(b));
    }

    #[test]
    fn vector_entrywise_zero() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(0.0, 0.0, 0.0);
        let c = Vector3::new(0.0, 0.0, 0.0);

        assert_eq!(c, a.entrywise(b));
    }

    #[test]
    fn vector3_normal() {
        let a = Vector3::new(4.0, 4.0, 2.0);
        assert_eq!(a.normal(), 6.0);
    }

    #[test]
    fn vector3_normal_negative() {
        let a = Vector3::new(-4.0, -4.0, -2.0);
        assert_eq!(a.normal(), 6.0);
    }

    #[test]
    fn vector3_normal_zero() {
        let a = Vector3::new(0.0, 0.0, 0.0);
        assert_eq!(a.normal(), 0.0);
    }

    #[test]
    fn vector3_normalized() {
        let a = Vector3::new(2.0, 0.0, 0.0);
        let b = Vector3::new(1.0, 0.0, 0.0);
        let c = a.unit_vec();
        assert_eq!(c, b)
    }

    #[test]
    fn vector3_normalized_negative() {
        let a = Vector3::new(-2.0, 0.0, 0.0);
        let b = Vector3::new(-1.0, 0.0, 0.0);
        let c = a.unit_vec();
        assert_eq!(c, b)
    }

    #[test]
    /// In accordance to the IEEE 754 standard division of zero by zero (norm of zero vector is zero) results in NaN
    fn vector3_normalized_zero() {
        let a = Vector3::new(0.0, 0.0, 0.0);
        let result = std::panic::catch_unwind(|| a.unit_vec() );
        assert!(result.is_err());
    }
}