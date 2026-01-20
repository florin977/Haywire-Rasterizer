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

impl RotationMatrices {
    pub fn new(x_angle: f64, y_angle: f64, z_angle: f64) -> Self {
        let (sin_x, cos_x) = x_angle.sin_cos();
        let mut x_mat =  Matrix4x4::identity();
        x_mat.data[1][1] = cos_x;
        x_mat.data[1][2] = -sin_x;
        x_mat.data[2][1] = sin_x;
        x_mat.data[2][2] = cos_x;

        let (sin_y, cos_y) = y_angle.sin_cos();
        let mut y_mat = Matrix4x4::identity();
        y_mat.data[0][0] = cos_y;
        y_mat.data[0][2] = sin_y;
        y_mat.data[2][0] = -sin_y;
        y_mat.data[2][2] = cos_y;

        let (sin_z, cos_z) = z_angle.sin_cos();
        let mut z_mat = Matrix4x4::identity();
        z_mat.data[0][0] = cos_z;
        z_mat.data[0][1] = -sin_z;
        z_mat.data[1][0] = sin_z;
        z_mat.data[1][1] = cos_z;

        Self {
            xRotation: x_mat,
            yRotation: y_mat,
            zRotation: z_mat,
        } 
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