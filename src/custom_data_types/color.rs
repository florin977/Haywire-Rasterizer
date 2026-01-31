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

    pub fn r(&self) -> u8 {
        self.r
    }
    pub fn g(&self) -> u8 {
        self.g
    }
    pub fn b(&self) -> u8 {
        self.b
    }
    pub fn a(&self) -> u8 {
        self.a
    }

    pub fn mix(color1: Color, color2: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);

        let r = (color1.r as f32 * (1.0 - t) + color2.r as f32 * t) as u8;
        let g = (color1.g as f32 * (1.0 - t) + color2.g as f32 * t) as u8;
        let b = (color1.b as f32 * (1.0 - t) + color2.b as f32 * t) as u8;

        let a = color1.a;

        Color::new(r, g, b, a)
    }

    pub fn format_as_u32(&self) -> u32 {
        0 | ((self.r() as u32) << 16) | ((self.g() as u32) << 8) | self.b() as u32
    }
}
