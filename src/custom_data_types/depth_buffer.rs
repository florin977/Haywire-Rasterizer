use minifb::Window;

use crate::custom_data_types::draw_buffer::DrawBuffer;

pub struct DepthBuffer {
    buffer: Vec<f32>,
    buffer_width: usize,
    buffer_height: usize,
}
impl DepthBuffer {
    pub fn new(buffer: Vec<f32>, buffer_width: usize, buffer_height: usize) -> Self {
        if buffer_width == 0 || buffer_height == 0 {
            return Self {
                buffer,
                buffer_width,
                buffer_height,
            };
        }
        Self {
            buffer,
            buffer_width,
            buffer_height,
        }
    }

    pub fn buffer_width(&self) -> usize {
        self.buffer_width
    }
    pub fn buffer_height(&self) -> usize {
        self.buffer_height
    }
    pub fn buffer(&self) -> &Vec<f32> {
        &self.buffer
    }

    fn clear(&mut self) {
        self.buffer.fill(1.0f32);
    }

    pub fn handle_clear(&mut self, window: &Window) -> () {
        let (new_width, new_height) = window.get_size();

        if self.buffer_width() != new_width || self.buffer_height() != new_height {
            self.resize(new_width, new_height);
        } else {
            self.clear();
        }
    }

    pub fn resize(&mut self, buffer_width: usize, buffer_height: usize) {
        self.buffer_width = buffer_width;
        self.buffer_height = buffer_height;
        self.buffer = vec![1.0f32; self.buffer_width() * self.buffer_height()];
    }

    pub fn set(&mut self, x: usize, y: usize, depth: f32) {
        self.buffer[(self.buffer_height - x - 1) * self.buffer_width + y] = depth;
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.buffer[(self.buffer_height - x - 1) * self.buffer_width + y]
    }
}
