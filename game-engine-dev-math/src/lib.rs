
use std::ops::{Add, Sub, Mul,Div};

#[derive(Debug)]
struct Vector3D{
    x: f32,
    y: f32,
    z: f32,
}
impl Add<Vector3D> for Vector3D {
    type Output = Vector3D;
    fn add(self, v2: Vector3D) -> Vector3D{
        Vector3D{x: self.x + v2.x, y:self.y + v2.y, z:self.z + v2.z}
    }
}

impl Sub<Vector3D> for Vector3D {
    type Output = Vector3D;
    fn sub(self, v2: Vector3D) -> Vector3D{
        Vector3D{x: self.x - v2.x, y:self.y - v2.y, z:self.z - v2.z}
    }
}

impl Mul<Vector3D> for Vector3D {
    type Output = f32;
    fn mul(self, v2: Vector3D) -> f32{
        self.x * v2.x + self.y * v2.y + self.z * v2.z
    }
}

impl Mul<f32> for Vector3D {
    type Output = Vector3D;
    fn mul(self, s: f32) -> Vector3D{
        Vector3D{x: self.x * s, y:self.y * s, z:self.z * s}
    }
}
impl Div<f32> for Vector3D {
    type Output = Vector3D;
    fn div(self, s: f32) -> Vector3D{
        Vector3D{x: self.x / s, y:self.y /s, z:self.z /s}
    }
}


#[cfg(test)]

mod tests {
    use crate::Vector3D;
    const V1 : Vector3D = Vector3D{x:2.0,y:3.0,z:4.0};
    const V2 : Vector3D = Vector3D{x:5.0,y:6.0,z:7.0};
    #[test]
    fn test_add_two_vector3ds() {
        let sum = V1 + V2;
        assert_eq!(sum.x, 7.0);
        assert_eq!(sum.y, 9.0);
        assert_eq!(sum.z, 11.0);
    }
    #[test]
    fn test_sub_two_vector3ds() {
        let subt = V1 - V2;
        assert_eq!(subt.x, -3.0);
        assert_eq!(subt.y, -3.0);
        assert_eq!(subt.z, -3.0);
    }
    #[test]
    fn test_mul_two_vector3ds() {
        let dot = V1 * V2;
        assert_eq!(dot, 56.0);
    }
     #[test]
    fn test_mul_scalar() {
        let prod = V1 * 2.0;
        assert_eq!(prod.x, 4.0);
        assert_eq!(prod.y, 6.0);
        assert_eq!(prod.z, 8.0);
    }
     #[test]
    fn test_div_scalar() {
        let quot = V1 / 2.0;
        assert_eq!(quot.x, 1.0);
        assert_eq!(quot.y, 1.5);
        assert_eq!(quot.z, 2.0);
    }
}
