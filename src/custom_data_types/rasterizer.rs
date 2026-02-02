use super::color::Color;
use super::depth_buffer::DepthBuffer;
use super::draw_buffer::DrawBuffer;
use super::vec4::Vec4;
use crate::custom_data_types::camera::{self, Camera};
use crate::custom_data_types::matrices::Matrix4x4;
use crate::custom_data_types::scene::Scene;

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
                        i as usize, j as usize,
                        color,
                        //Color::new(depth_color, depth_color, depth_color, depth_color),
                    );
                }
            }
        }
    }

    fn find_t(a: Vec4, b: Vec4, camera: &Camera) -> f32 {
        (-a.z - camera.z_near) / (b.z - a.z)
    }

    fn lerp(a: Vec4, b: Vec4, t: f32) -> Vec4 {
        a + (b - a) * t
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
        Option<(i32, i32, f32)>,
    ) {
        let v0_clip = *mvp * v0;
        let v1_clip = *mvp * v1;
        let v2_clip = *mvp * v2;

        let to_screen = |v: Vec4| -> (i32, i32, f32) {
            let ndc = v / v.w; // Perspective Divide
            Self::viewport_transform(self, ndc)
        };

        let mut behind_camera = (false, false, false);
        let mut total_behind_camera = 0;
        if v0_clip.w < camera.z_near {
            behind_camera.0 = true;
            total_behind_camera += 1;
        }
        if v1_clip.w < camera.z_near {
            behind_camera.1 = true;
            total_behind_camera += 1;
        }
        if v2_clip.w < camera.z_near {
            behind_camera.2 = true;
            total_behind_camera += 1;
        }

        if total_behind_camera == 3 {
            return (None, None, None, None);
        } else if total_behind_camera == 2 {
            if !behind_camera.0 {
                let t1 = Self::find_t(v0_clip, v1_clip, camera);
                let t2 = Self::find_t(v0_clip, v2_clip, camera);

                let new_v1 = Self::lerp(v0_clip, v1_clip, t1);
                let new_v2 = Self::lerp(v0_clip, v2_clip, t2);

                return (
                    Some(to_screen(v0_clip)),
                    Some(to_screen(new_v1)),
                    Some(to_screen(new_v2)),
                    None,
                );
            } else if !behind_camera.1 {
                let t2 = Self::find_t(v1_clip, v2_clip, camera);
                let t0 = Self::find_t(v1_clip, v0_clip, camera);

                let new_v2 = Self::lerp(v1_clip, v2_clip, t2);
                let new_v0 = Self::lerp(v1_clip, v0_clip, t0);

                return (
                    Some(to_screen(v1_clip)),
                    Some(to_screen(new_v2)),
                    Some(to_screen(new_v0)),
                    None,
                );
            } else if !behind_camera.2 {
                let t0 = Self::find_t(v2_clip, v0_clip, camera);
                let t1 = Self::find_t(v2_clip, v1_clip, camera);

                let new_v0 = Self::lerp(v2_clip, v0_clip, t0);
                let new_v1 = Self::lerp(v2_clip, v1_clip, t1);

                return (
                    Some(to_screen(v2_clip)),
                    Some(to_screen(new_v0)),
                    Some(to_screen(new_v1)),
                    None,
                );
            }
        } else if total_behind_camera == 1 {
            if behind_camera.0 {
                let t_20 = Self::find_t(v2_clip, v0_clip, camera); // Cut on V2->V0
                let t_10 = Self::find_t(v1_clip, v0_clip, camera); // Cut on V1->V0

                let new_v2 = Self::lerp(v2_clip, v0_clip, t_20);
                let new_v1 = Self::lerp(v1_clip, v0_clip, t_10);

                return (
                    Some(to_screen(v1_clip)),
                    Some(to_screen(v2_clip)),
                    Some(to_screen(new_v2)),
                    Some(to_screen(new_v1)),
                );
            } else if behind_camera.1 {
                let t_01 = Self::find_t(v0_clip, v1_clip, camera);
                let t_21 = Self::find_t(v2_clip, v1_clip, camera);

                let new_v0 = Self::lerp(v0_clip, v1_clip, t_01);
                let new_v2 = Self::lerp(v2_clip, v1_clip, t_21);

                return (
                    Some(to_screen(v2_clip)),
                    Some(to_screen(v0_clip)),
                    Some(to_screen(new_v0)),
                    Some(to_screen(new_v2)),
                );
            } else if behind_camera.2 {
                let t_12 = Self::find_t(v1_clip, v2_clip, camera);
                let t_02 = Self::find_t(v0_clip, v2_clip, camera);

                let new_v1 = Self::lerp(v1_clip, v2_clip, t_12);
                let new_v0 = Self::lerp(v0_clip, v2_clip, t_02);

                return (
                    Some(to_screen(v0_clip)),
                    Some(to_screen(v1_clip)),
                    Some(to_screen(new_v1)),
                    Some(to_screen(new_v0)),
                );
            }
        }

        (
            Some(to_screen(v0_clip)),
            Some(to_screen(v1_clip)),
            Some(to_screen(v2_clip)),
            None,
        )
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
                let (opt0, opt1, opt2, opt3) =
                    self.world_to_screen(v0_local, v1_local, v2_local, &mvp, &scene.camera);

                if let (Some(p0), Some(p1), Some(p2)) = (opt0, opt1, opt2) {
                    let area = Self::edge_function(p0, p1, p2);
                    if area <= 0 {
                        continue;
                    }

                    self.fill_triangle(p0, p1, p2, colors[i % colors.len()]);

                    if let Some(p3) = opt3 {
                        let area = Self::edge_function(p0, p2, p3);
                        if area <= 0 {
                            continue;
                        }

                        self.fill_triangle(p0, p2, p3, colors[i % colors.len()]);
                    }
                }
            }
        }
    }
}
