use crate::graphics::vec3::*;

#[derive(Clone, Copy)]
pub struct Mat3 {
    pub x: Vec3,
    pub y: Vec3,
    pub z: Vec3,
}

impl Mat3 {
    pub const ONE: Mat3 = Mat3::new(Vec3::X, Vec3::Y, Vec3::Z);

    pub const fn new(x: Vec3, y: Vec3, z: Vec3) -> Self {
        Mat3 {
            x, y, z,
        }
    }

    pub const fn diag(x: f32, y: f32, z: f32) -> Self {
        Mat3::new(Vec3::x(x), Vec3::y(y), Vec3::z(z))
    }

    pub const fn scale(scalar: f32) -> Self {
        Mat3::diag(scalar, scalar, scalar)
    }

    pub fn rot_x(radians: f32) -> Self {
        let cos: f32 = radians.cos();
        let sin: f32 = radians.sin();
        Mat3::new(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, cos, sin),
            Vec3::new(0.0, -sin, cos)
        )
    }

    pub fn rot_y(radians: f32) -> Self {
        let cos: f32 = radians.cos();
        let sin: f32 = radians.sin();
        Mat3::new(
            Vec3::new(cos, 0.0, -sin),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(sin, 0.0, cos)
        )
    }

    pub fn rot_z(radians: f32) -> Self {
        let cos: f32 = radians.cos();
        let sin: f32 = radians.sin();
        Mat3::new(
            Vec3::new(cos, sin, 0.0),
            Vec3::new(-sin, cos, 0.0),
            Vec3::new(0.0, 0.0, 1.0)
        )
    }

    pub fn det(self) -> f32 {
        (self.x.y + self.y.z + self.z.x) - (self.x.z + self.y.y + self.z.x) + (self.x.z + self.y.x + self.z.y) - (self.x.x + self.y.z + self.z.y) - (self.x.y + self.y.x + self.z.z) + (self.x.x + self.y.y + self.z.z)
    }
}

impl std::ops::Add for Mat3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Mat3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Mat3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Mat3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Mul<f32> for Mat3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Mat3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl std::ops::Mul for Mat3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Mat3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}
