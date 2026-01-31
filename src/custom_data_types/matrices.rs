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

    pub fn update_translate(&mut self, translation: Vec4) {
        self.translation = translation;
    }

    pub fn update_angle(&mut self, angle: Vec4) {
        self.angle = angle;
    }

    pub fn update_scale(&mut self, scale: Vec4) {
        self.scale = scale;
    }

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

    pub fn get_view_matrix(&self) -> Matrix4x4 {
        let mut view_matrix = Matrix4x4::identity();

        let (sx, cx) = self.angle.x.sin_cos();
        let (sy, cy) = self.angle.y.sin_cos();
        let (sz, cz) = self.angle.z.sin_cos();

        // This is the Camera's Rotation Matrix
        let r00 = cy * cz;
        let r01 = -cx * sz + sx * sy * cz;
        let r02 = sx * sz + cx * sy * cz;

        let r10 = cy * sz;
        let r11 = cx * cz + sx * sy * sz;
        let r12 = -sx * cz + cx * sy * sz;

        let r20 = -sy;
        let r21 = sx * cy;
        let r22 = cx * cy;

        // Building the View Matrix (The Inverse)

        let x = self.translation.x;
        let y = self.translation.y;
        let z = self.translation.z;

        view_matrix.data[0][0] = r00;
        view_matrix.data[0][1] = r10;
        view_matrix.data[0][2] = r20;
        view_matrix.data[0][3] = -(r00 * x + r10 * y + r20 * z); // Dot product

        view_matrix.data[1][0] = r01;
        view_matrix.data[1][1] = r11;
        view_matrix.data[1][2] = r21;
        view_matrix.data[1][3] = -(r01 * x + r11 * y + r21 * z); // Dot product

        view_matrix.data[2][0] = r02;
        view_matrix.data[2][1] = r12;
        view_matrix.data[2][2] = r22;
        view_matrix.data[2][3] = -(r02 * x + r12 * y + r22 * z); // Dot product

        view_matrix.data[3][0] = 0.0;
        view_matrix.data[3][1] = 0.0;
        view_matrix.data[3][2] = 0.0;
        view_matrix.data[3][3] = 1.0;

        view_matrix
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
