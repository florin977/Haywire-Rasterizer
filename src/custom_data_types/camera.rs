use super::matrices::{Matrix4x4, ModelMatrix};

pub struct Camera {
    pub model: ModelMatrix,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Camera {
    pub fn new(model: ModelMatrix, fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        Self {
            model,
            fov,
            aspect_ratio,
            z_near,
            z_far,
        }
    }

    pub fn get_view_matrix(&self) -> Matrix4x4 {
        let mut view_matrix = Matrix4x4::identity();

        let (sx, cx) = self.model.angle().x.sin_cos();
        let (sy, cy) = self.model.angle().y.sin_cos();
        let (sz, cz) = self.model.angle().z.sin_cos();

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

        let x = self.model.translation().x;
        let y = self.model.translation().y;
        let z = self.model.translation().z;

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

    pub fn get_projection_matrix(&self) -> Matrix4x4 {
        let mut projection_matrix = Matrix4x4::zero();
        let tangent_fov = (self.fov / 2.0).tan();

        projection_matrix.data[0][0] = 1.0 / (self.aspect_ratio * tangent_fov);
        projection_matrix.data[1][1] = 1.0 / tangent_fov;

        projection_matrix.data[2][2] = -(self.z_far + self.z_near) / (self.z_far - self.z_near);
        projection_matrix.data[2][3] =
            -(2.0 * self.z_far * self.z_near) / (self.z_far - self.z_near);

        projection_matrix.data[3][2] = -1.0;

        projection_matrix
    }
}
