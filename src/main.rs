// custom modules
mod custom_data_types;

use custom_data_types::draw_buffer::DrawBuffer;
use custom_data_types::game_object::GameObject;
use custom_data_types::matrices::ModelMatrix;
use custom_data_types::mesh::Mesh;
use custom_data_types::vec4::Vec4;

// external dependencies
use minifb::{Key, MouseMode, Window, WindowOptions};
use rand::Rng;

use crate::custom_data_types::camera::Camera;
use crate::custom_data_types::color::Color;
use crate::custom_data_types::depth_buffer::DepthBuffer;
use crate::custom_data_types::rasterizer::Rasterizer;
use crate::custom_data_types::scene::Scene;

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
    let mut rasterizer = Rasterizer::new(
        DrawBuffer::new(vec![0; 1280 * 720], 1280, 720),
        DepthBuffer::new(vec![1.0; 1280 * 720], 1280, 720),
    );
    let camera = Camera::new(
        ModelMatrix::new(
            Vec4::new(0.0, 0.0, 0.0, 1.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
            Vec4::new(1.0, 1.0, 1.0, 1.0),
        ),
        90.0f32.to_radians(),
        16.0 / 9.0,
        1.0,
        3000.0,
    );

    let mut window = Window::new(
        "Haywire Rasterizer",
        rasterizer.draw_buffer.buffer_width(),
        rasterizer.draw_buffer.buffer_height(),
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open Window");

    window.set_target_fps(60);

    let mut meshes: Vec<Mesh> = vec![];
    meshes.push(Mesh::new("./assets/cube.obj", ".obj"));
    let mut obj: Vec<GameObject> = vec![];
    obj.push(GameObject::new(
        0,
        ModelMatrix::new(
            Vec4::new(0.0, 0.0, -10.0, 1.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
            Vec4::new(7.0, 7.0, 7.0, 1.0),
        ),
    ));

    let mut scene = Scene::new(meshes, obj, camera);

    let mut pos = Vec4::new(0.0, 0.0, 0.0, 1.0);
    let mut angle = Vec4::new(0.0, 0.0, 0.0, 1.0);
    let mut last_mouse_pos = (0.0f32, 0.0f32);
    let mut random_colors: Vec<Color> = vec![];

    for _i in 0..100 {
        let r = rand::thread_rng().gen_range(50..255);
        let g = rand::thread_rng().gen_range(50..255);
        let b = rand::thread_rng().gen_range(50..255);

        let color = Color::new(r, g, b, 255);
        random_colors.push(color);
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        rasterizer.draw_buffer.handle_clear(&window);
        rasterizer.depth_buffer.handle_clear(&window);

        //y_angle += 0.02;
        //x_angle += 0.00;

        let current_mouse_pos = window
            .get_mouse_pos(MouseMode::Pass)
            .unwrap_or(last_mouse_pos);

        if window.get_mouse_down(minifb::MouseButton::Left) {
            let sensitivity = 0.005;
            let dy = (current_mouse_pos.0 - last_mouse_pos.0) * sensitivity;
            let dx = (current_mouse_pos.1 - last_mouse_pos.1) * sensitivity;

            angle.x += dx;
            angle.y -= dy;
        }

        let yaw = angle.y;
        let forward_x = -yaw.sin();
        let forward_z = -yaw.cos();
        let right_x = forward_z;
        let right_z = -forward_x;
        let speed = 0.1;

        if window.is_key_down(Key::A) {
            pos.x += right_x * speed;
            pos.z += right_z * speed;
        } else if window.is_key_down(Key::D) {
            pos.x -= right_x * speed;
            pos.z -= right_z * speed;
        }

        if window.is_key_down(Key::W) {
            pos.x += forward_x * speed;
            pos.z += forward_z * speed;
        } else if window.is_key_down(Key::S) {
            pos.x -= forward_x * speed;
            pos.z -= forward_z * speed;
        }

        if window.is_key_down(Key::Space) {
            pos.y += speed;
        } else if window.is_key_down(Key::LeftCtrl) {
            pos.y -= speed;
        }

        scene.camera.model.update_translate(pos);
        scene.camera.model.update_angle(angle);

        rasterizer.draw_scene(&scene, &random_colors);
        draw(&mut rasterizer.draw_buffer, &mut window);

        last_mouse_pos = current_mouse_pos;
    }
}
