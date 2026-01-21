// custom modules
mod custom_data_types;

use custom_data_types::color::Color;
use custom_data_types::draw_buffer::DrawBuffer;
use custom_data_types::matrices::{Matrix4x4, RotationMatrices};
use custom_data_types::point::Point;
use custom_data_types::triangle::Triangle;
use custom_data_types::vec4::Vec4;

// external dependencies
use minifb::{Key, Window, WindowOptions};

fn load_model(path: &str) -> Vec<Triangle> {
    let mut triangles: Vec<Triangle> = vec![];

    let load_options = tobj::LoadOptions {
        triangulate: true,
        single_index: true,
        ..Default::default()
    };

    let (models, _materials) = tobj::load_obj(path, &load_options).expect("Failed to load models");

    for m in 0..models.len() {
        let mesh = &models[m].mesh;

        for i in (0..mesh.indices.len()).step_by(3) {
            let idx0 = mesh.indices[i] as usize;
            let idx1 = mesh.indices[i + 1] as usize;
            let idx2 = mesh.indices[i + 2] as usize;

            let v0_x = mesh.positions[3 * idx0];
            let v0_y = mesh.positions[3 * idx0 + 1];
            let v0_z = mesh.positions[3 * idx0 + 2];

            let v1_x = mesh.positions[3 * idx1];
            let v1_y = mesh.positions[3 * idx1 + 1];
            let v1_z = mesh.positions[3 * idx1 + 2];

            let v2_x = mesh.positions[3 * idx2];
            let v2_y = mesh.positions[3 * idx2 + 1];
            let v2_z = mesh.positions[3 * idx2 + 2];

            let vec0 = Vec4::new(v0_x as f64, v0_y as f64, v0_z as f64, 1.0);
            let vec1 = Vec4::new(v1_x as f64, v1_y as f64, v1_z as f64, 1.0);
            let vec2 = Vec4::new(v2_x as f64, v2_y as f64, v2_z as f64, 1.0);

            let triangle = Triangle::new(vec0, vec1, vec2);

            triangles.push(triangle);
        }
    }

    triangles
}

fn edge_function(a_x: f64, a_y: f64, b_x: f64, b_y: f64, c_x: f64, c_y: f64) -> f64 {
    let result = (b_x - a_x) * (c_y - a_y) - (b_y - a_y) * (c_x - a_x);
    result
}

fn in_triangle(
    a_x: f64,
    a_y: f64,
    b_x: f64,
    b_y: f64,
    c_x: f64,
    c_y: f64,
    point_x: f64,
    point_y: f64,
) -> bool {
    let abp = edge_function(a_x, a_y, b_x, b_y, point_x, point_y);
    let bcp = edge_function(b_x, b_y, c_x, c_y, point_x, point_y);
    let cap = edge_function(c_x, c_y, a_x, a_y, point_x, point_y);

    abp >= 0.0 && bcp >= 0.0 && cap >= 0.0
}

fn draw_triangle(draw_buffer: &mut DrawBuffer, triangle: &Triangle, color: Color) -> () {
    let a_x = triangle.a().x;
    let a_y = triangle.a().y;
    let b_x = triangle.b().x;
    let b_y = triangle.b().y;
    let c_x = triangle.c().x;
    let c_y = triangle.c().y;

    let area = edge_function(a_x, a_y, b_x, b_y, c_x, c_y);

    if area <= 0.0 {
        return;
    }

    let triangle_min_x = a_x.min(b_x.min(c_x));
    let triangle_min_y = a_y.min(b_y.min(c_y));
    let triangle_max_x = a_x.max(b_x.max(c_x));
    let triangle_max_y = a_y.max(b_y.max(c_y));

    let min_x = (triangle_min_x as i32).max(0i32);
    let min_y = (triangle_min_y as i32).max(0i32);
    let max_x = (triangle_max_x as i32).min(draw_buffer.buffer_width() as i32);
    let max_y = (triangle_max_y as i32).min(draw_buffer.buffer_height() as i32);

    for i in min_y..max_y {
        for j in min_x..max_x {
            if in_triangle(a_x, a_y, b_x, b_y, c_x, c_y, j as f64, i as f64) {
                draw_buffer.set(i as usize, j as usize, color);
            }
        }
    }
}

fn handle_clear(draw_buffer: &mut DrawBuffer, window: &Window) -> () {
    let (new_width, new_height) = window.get_size();

    if draw_buffer.buffer_width() != new_width || draw_buffer.buffer_height() != new_height {
        draw_buffer.resize(new_width, new_height);
    } else {
        draw_buffer.clear(Color::new(0, 0, 0, 255));
    }
}

fn draw(draw_buffer: &mut DrawBuffer, window: &mut Window) -> () {
    window
        .update_with_buffer(
            draw_buffer.buffer(),
            draw_buffer.buffer_width(),
            draw_buffer.buffer_height(),
        )
        .unwrap();
}

fn main() {
    let mut draw_buffer = DrawBuffer::new(vec![0; 1280 * 720], 1280, 720);

    let mut window = Window::new(
        "Haywire Rasterizer",
        draw_buffer.buffer_width(),
        draw_buffer.buffer_height(),
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open Window");

    window.set_target_fps(60);

    let color: Color = Color::new(0, 0, 0, 255);

    let triangles: Vec<Triangle> = load_model("./assets/cube.obj");

    let mut y_angle = 0.0;
    let mut x_angle = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        handle_clear(&mut draw_buffer, &window);

        y_angle += 0.01;
        x_angle += 0.02;

        let scale_mat = Matrix4x4::scaling(200.0);

        let rot_struct = RotationMatrices::new(x_angle , y_angle, 0.0);
        let rot_mat = rot_struct.get_rotation();

        let trans_mat = Matrix4x4::translation(360.0, 180.0, 0.0);

        let world_matrix = trans_mat * rot_mat * scale_mat;

        for i in 0..triangles.len() {
            let v0_trans = world_matrix * triangles[i].a();
            let v1_trans = world_matrix * triangles[i].b();
            let v2_trans = world_matrix * triangles[i].c();

            let tri_to_draw = Triangle::new(v0_trans, v1_trans, v2_trans);

            let r = color.r().wrapping_add((i * 10) as u8);
            let g = color.g().wrapping_add((i * 30) as u8);
            let b = color.b().wrapping_add((i * 50) as u8);

            draw_triangle(&mut draw_buffer, &tri_to_draw, Color::new(r, g, b, 255));
        }

        draw(&mut draw_buffer, &mut window);
    }
}
