use super::color::Color;
use super::depth_buffer::DepthBuffer;
use super::draw_buffer::DrawBuffer;
use super::vec4::Vec4;
use crate::custom_data_types::camera::Camera;
use crate::custom_data_types::matrices::Matrix4x4;
use crate::custom_data_types::scene::Scene;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

pub struct Rasterizer {
    pub draw_buffer: DrawBuffer,
    pub depth_buffer: DepthBuffer,
}

impl Rasterizer {
    pub fn new(draw_buffer: DrawBuffer, depth_buffer: DepthBuffer) -> Self {
        Self {
            draw_buffer,
            depth_buffer,
        }
    }

    fn viewport_transform(&self, point: Vec4) -> (i32, i32, f32) {
        let x = (point.x + 1.0) * 0.5 * self.draw_buffer.buffer_width() as f32;
        let y = (point.y + 1.0) * 0.5 * self.draw_buffer.buffer_height() as f32;
        let z = (point.z + 1.0) * 0.5;

        (x as i32, y as i32, z)
    }

    fn edge_function(a: (i32, i32, f32), b: (i32, i32, f32), c: (i32, i32, f32)) -> i32 {
        let result = (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0);
        result
    }

    fn in_triangle(
        &self,
        a: (i32, i32, f32),
        b: (i32, i32, f32),
        c: (i32, i32, f32),
        point: (i32, i32, f32),
    ) -> (bool, f32) {
        let abp = Self::edge_function(a, b, point) as f32;
        let bcp = Self::edge_function(b, c, point) as f32;
        let cap = Self::edge_function(c, a, point) as f32;
        let total_area = abp + bcp + cap;
        let weight_c = abp / total_area;
        let weight_a = bcp / total_area;
        let weight_b = cap / total_area;

        let z_coord = c.2 * weight_c + b.2 * weight_b + a.2 * weight_a;

        return (abp >= 0.0 && bcp >= 0.0 && cap >= 0.0, z_coord);
    }

    fn fill_triangle(
        &mut self,
        a: (i32, i32, f32),
        b: (i32, i32, f32),
        c: (i32, i32, f32),
        color: Color,
    ) {
        let min_x = a.0.min(b.0.min(c.0));
        let min_y = a.1.min(b.1.min(c.1));
        let max_x = a.0.max(b.0.max(c.0));
        let max_y = a.1.max(b.1.max(c.1));

        let min_x = min_x.max(0i32);
        let min_y = min_y.max(0i32);
        let max_x = max_x.min(self.draw_buffer.buffer_width() as i32);
        let max_y = max_y.min(self.draw_buffer.buffer_height() as i32);

        for i in min_y..max_y {
            for j in min_x..max_x {
                let (in_trig, z_coord) = Self::in_triangle(&self, a, b, c, (j, i, 0.0));
                let buffer_value = self.depth_buffer.get(i as usize, j as usize);

                if in_trig && z_coord < buffer_value {
                    self.depth_buffer.set(i as usize, j as usize, z_coord);
                    let depth_color = (255.0 * z_coord) as u8;

                    self.draw_buffer.set(
                        i as usize,
                        j as usize,
                        Color::new(depth_color, depth_color, depth_color, depth_color),
                    );
                }
            }
        }
    }

    pub fn world_to_screen(
        &self,
        v0: Vec4,
        v1: Vec4,
        v2: Vec4,
        mvp: &Matrix4x4,
        camera: &Camera,
    ) -> (
        Option<(i32, i32, f32)>,
        Option<(i32, i32, f32)>,
        Option<(i32, i32, f32)>,
    ) {
        let v0_clip = *mvp * v0;
        let v1_clip = *mvp * v1;
        let v2_clip = *mvp * v2;

        // TODO: actually split the triangles to handle ones behind the camera
        if v0_clip.w < camera.z_near || v1_clip.w < camera.z_near || v2_clip.w < camera.z_near {
            return (None, None, None);
        }

        let v0_ndc = v0_clip / v0_clip.w;
        let v1_ndc = v1_clip / v1_clip.w;
        let v2_ndc = v2_clip / v2_clip.w;

        let a = Self::viewport_transform(&self, v0_ndc);
        let b = Self::viewport_transform(&self, v1_ndc);
        let c = Self::viewport_transform(&self, v2_ndc);

        (Some(a), Some(b), Some(c))
    }
    pub fn draw_scene(&mut self, scene: &Scene, colors: &Vec<Color>) {
        let view_matrix = scene.camera.get_view_matrix();
        let projection_matrix = scene.camera.get_projection_matrix();
        let pv = projection_matrix * view_matrix;

        for obj in &scene.objects {
            let model_matrix = obj.model_matrix.get_model_matrix();
            let mvp = pv * model_matrix;

            let mesh = &scene.meshes[obj.object_id];

            for i in (0..mesh.indices.len()).step_by(3) {
                let idx0 = mesh.indices[i];
                let idx1 = mesh.indices[i + 1];
                let idx2 = mesh.indices[i + 2];

                let v0_local = mesh.vertices[idx0];
                let v1_local = mesh.vertices[idx1];
                let v2_local = mesh.vertices[idx2];
                if let (Some(p0), Some(p1), Some(p2)) =
                    self.world_to_screen(v0_local, v1_local, v2_local, &mvp, &scene.camera)
                {
                    let area = Self::edge_function(p0, p1, p2);
                    if area <= 0 {
                        continue;
                    }

                    self.fill_triangle(p0, p1, p2, colors[i % colors.len()]);
                }
            }
        }
    }
}
