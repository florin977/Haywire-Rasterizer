use super::vec4::Vec4;
use std::ops::Mul;

#[derive(Debug, Copy, Clone)]
pub struct Matrix4x4 {
    pub data: [[f32; 4]; 4],
}

pub struct ModelMatrix {
    translation: Vec4,
    angle: Vec4,
    scale: Vec4,
}

impl ModelMatrix {
    pub fn new(translation: Vec4, angle: Vec4, scale: Vec4) -> Self {
        Self {
            translation,
            angle,
            scale,
        }
    }

    // If there was a non-uniform scaling applied, then the inverse
    // must be calculated for the normals.
    pub fn inverse_needed_for_normals(&self) -> bool {
        return !(self.scale.x == self.scale.y && self.scale.y == self.scale.z);
    }

    pub fn angle(&self) -> Vec4 {
        self.angle
    }

    pub fn translation(&self) -> Vec4 {
        self.translation
    }

    pub fn update_translate(&mut self, translation: Vec4) {
        self.translation = translation;
    }

    pub fn update_angle(&mut self, angle: Vec4) {
        self.angle = angle;
    }

    pub fn update_scale(&mut self, scale: Vec4) {
        self.scale = scale;
    }

    // Assumes only T, R and S operations were performed,
    // then computed the M matrix based on them
    pub fn get_model_matrix(&self) -> Matrix4x4 {
        let mut model_matrix = Matrix4x4::identity();

        let (sx, cx) = self.angle.x.sin_cos();
        let (sy, cy) = self.angle.y.sin_cos();
        let (sz, cz) = self.angle.z.sin_cos();

        // The Rotation * Scale Matrix (RS)
        // Row 1
        model_matrix.data[0][0] = (cz * cy) * self.scale.x;
        model_matrix.data[0][1] = (-sz * cx + cz * sy * sx) * self.scale.y;
        model_matrix.data[0][2] = (sz * sx + cz * sy * cx) * self.scale.z;

        // Row 2
        model_matrix.data[1][0] = (sz * cy) * self.scale.x;
        model_matrix.data[1][1] = (cz * cx + sz * sy * sx) * self.scale.y;
        model_matrix.data[1][2] = (-cz * sx + sz * sy * cx) * self.scale.z;

        // Row 3
        model_matrix.data[2][0] = (-sy) * self.scale.x;
        model_matrix.data[2][1] = (cy * sx) * self.scale.y;
        model_matrix.data[2][2] = (cy * cx) * self.scale.z;

        // Appending the translation
        model_matrix.data[0][3] = self.translation.x;
        model_matrix.data[1][3] = self.translation.y;
        model_matrix.data[2][3] = self.translation.z;

        model_matrix
    }
}

impl Matrix4x4 {
    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn zero() -> Self {
        Self {
            data: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        }
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
            self.data[0][0] * rhs.x
                + self.data[0][1] * rhs.y
                + self.data[0][2] * rhs.z
                + self.data[0][3] * rhs.w,
            self.data[1][0] * rhs.x
                + self.data[1][1] * rhs.y
                + self.data[1][2] * rhs.z
                + self.data[1][3] * rhs.w,
            self.data[2][0] * rhs.x
                + self.data[2][1] * rhs.y
                + self.data[2][2] * rhs.z
                + self.data[2][3] * rhs.w,
            self.data[3][0] * rhs.x
                + self.data[3][1] * rhs.y
                + self.data[3][2] * rhs.z
                + self.data[3][3] * rhs.w,
        )
    }
}
