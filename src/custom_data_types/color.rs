#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn r(&self) -> u8 { self.r }
    pub fn g(&self) -> u8 { self.g }
    pub fn b(&self) -> u8 { self.b }
    pub fn a(&self) -> u8 { self.a }

    pub fn set_r(&mut self, r: u8) { self.r = r; }
    pub fn set_g(&mut self, g: u8) { self.g = g; }
    pub fn set_b(&mut self, b: u8) { self.b = b; }
    pub fn set_a(&mut self, a: u8) { self.a = a; }

    pub fn format_as_u32(&self) -> u32 {
        0 | ((self.r() as u32) << 16) | ((self.g() as u32) << 8) | self.b() as u32
    }
}