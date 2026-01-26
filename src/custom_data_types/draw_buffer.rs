use super::color::Color;

pub struct DrawBuffer {
    buffer: Vec<u32>,
    buffer_width: usize,
    buffer_height: usize,
    pub step_x: f64,
    pub step_y: f64,
}

impl DrawBuffer {
    pub fn new(buffer: Vec<u32>, buffer_width: usize, buffer_height: usize) -> Self {
        if buffer_width == 0 || buffer_height == 0 {
            return Self { buffer, buffer_width, buffer_height , step_x: 0.1, step_y: 0.1 };
        }
        Self { buffer, buffer_width, buffer_height , step_x: 2.0 / (buffer_width as f64), step_y: 2.0 / (buffer_height as f64)}
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

        if (buffer_height == 0 || buffer_width == 0) {
            self.step_x = 0.1;
            self.step_y = 0.1;
        }
        else {
            self.step_x = 2.0 / (buffer_width as f64);
            self.step_y = 2.0 / (buffer_height as f64);
        }
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