use super::vec4::Vec4;
use std::ops::Mul;

#[derive(Debug, Copy, Clone)]
pub struct Matrix4x4 {
    pub data: [[f64; 4]; 4],
}

pub struct Matrix3x3 {
    pub data: [[f64; 3]; 3]
}
pub struct RotationMatrices {
    xRotation: Matrix4x4,
    yRotation: Matrix4x4,
    zRotation: Matrix4x4,
    /* === xRotation ===
    [
        [1,  0,    0,  0],
        [0, cos, -sin, 0],
        [0, sin,  cos, 0],
        [0,  0,    0,  1],
    ] */
   /* === yRotation ===
    [
        [cos,  0, sin,  0],
        [ 0,   1, -0,   0],
        [-sin, 0, cos,  0],
        [  0,  0,  0,   1],
    ] */
   /* === zRotation ===
    [
        [cos,  -sin,    0,  0],
        [sin,   cos,    0,  0],
        [ 0,     0,     1,  0],
        [ 0,     0,     0,  1],
    ] */
}

impl Matrix4x4 {
    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        }
    }

    pub fn zero() -> Self {
        Self {
            data: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ]
        }
    }

    pub fn scaling(factor: f64) -> Self {
        let mut scaling_mat = Matrix4x4::identity();
        scaling_mat.data[0][0] = factor;
        scaling_mat.data[1][1] = factor;
        scaling_mat.data[2][2] = factor;
        scaling_mat.data[3][3] = 1.0;

        scaling_mat
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        let mut translation_mat = Matrix4x4::identity();
        translation_mat.data[0][3] = x;
        translation_mat.data[1][3] = y;
        translation_mat.data[2][3] = z;

        translation_mat
    }
}

impl Default for Matrix4x4 {
    fn default() -> Self {
        Self::identity()
    }
}

impl Mul for Matrix4x4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = Matrix4x4::zero();

        for row in 0..4 {
            for col in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.data[row][k] * rhs.data[k][col];
                }
                result.data[row][col] = sum;
            }
        }

        result
    }
}

impl Mul<Vec4> for Matrix4x4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Vec4 {
        Vec4::new(
            self.data[0][0] * rhs.x + self.data[0][1] * rhs.y + self.data[0][2] * rhs.z + self.data[0][3] * rhs.w,
            self.data[1][0] * rhs.x + self.data[1][1] * rhs.y + self.data[1][2] * rhs.z + self.data[1][3] * rhs.w,
            self.data[2][0] * rhs.x + self.data[2][1] * rhs.y + self.data[2][2] * rhs.z + self.data[2][3] * rhs.w,
            self.data[3][0] * rhs.x + self.data[3][1] * rhs.y + self.data[3][2] * rhs.z + self.data[3][3] * rhs.w,
        )
    }
}

impl RotationMatrices {
    pub fn new(x_angle: f64, y_angle: f64, z_angle: f64) -> Self {
        let mut rotations = Self {
            xRotation: Matrix4x4::identity(),
            yRotation: Matrix4x4::identity(),
            zRotation: Matrix4x4::identity(),
        };

        rotations.update_angles(x_angle, y_angle, z_angle);

        rotations
    }

    pub fn get_rotation(&self) -> Matrix4x4 {
        let combined = self.zRotation * self.yRotation * self.xRotation;
        combined
    }

    pub fn update_angles(&mut self, x_angle: f64, y_angle: f64, z_angle: f64)  {
        let (sin_x, cos_x) = x_angle.sin_cos();
        let (sin_y, cos_y) = y_angle.sin_cos();
        let (sin_z, cos_z) = z_angle.sin_cos();

        self.xRotation.data[1][1] = cos_x;
        self.xRotation.data[1][2] = -sin_x;
        self.xRotation.data[2][1] = sin_x;
        self.xRotation.data[2][2] = cos_x;

        self.yRotation.data[0][0] = cos_y;
        self.yRotation.data[0][2] = sin_y;
        self.yRotation.data[2][0] = -sin_y;
        self.yRotation.data[2][2] = cos_y;

        self.zRotation.data[0][0] = cos_z;
        self.zRotation.data[0][1] = -sin_z;
        self.zRotation.data[1][0] = sin_z;
        self.zRotation.data[1][1] = cos_z;
    }
}