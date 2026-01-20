use super::color::Color;

pub struct DrawBuffer {
    buffer: Vec<u32>,
    buffer_width: usize,
    buffer_height: usize,
}

impl DrawBuffer {
    pub fn new(buffer: Vec<u32>, buffer_width: usize, buffer_height: usize) -> Self {
        Self { buffer, buffer_width, buffer_height }
    }

    pub fn buffer_width(&self) -> usize { self.buffer_width }
    pub fn buffer_height(&self) -> usize { self.buffer_height }
    pub fn buffer(&self) -> &Vec<u32> { &self.buffer }

    pub fn clear(&mut self, clear_color: Color) {
        self.buffer.fill(clear_color.format_as_u32());
    }
    
    pub fn resize(&mut self, buffer_width: usize, buffer_height: usize) {
        self.buffer_width = buffer_width;
        self.buffer_height = buffer_height;

        self.buffer = vec![0; self.buffer_width() * self.buffer_height()];
    }

    pub fn get(&self, x: usize, y: usize) -> u32 {
        self.buffer
                    [(self.buffer_height - x - 1) * self.buffer_width + y]
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        self.buffer
                    [(self.buffer_height - x - 1) * self.buffer_width + y] = color.format_as_u32();
    }
}