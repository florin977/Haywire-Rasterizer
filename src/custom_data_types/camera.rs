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
