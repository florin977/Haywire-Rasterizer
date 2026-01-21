use super::vec4::Vec4;

#[derive(Clone, Copy)]
pub struct Triangle {
    a: Vec4,
    b: Vec4,
    c: Vec4,
}

impl Triangle {
    pub fn new(a: Vec4, b: Vec4, c: Vec4) -> Self {
        Self { a, b, c }
    }

    pub fn a(&self) -> Vec4 { self.a }
    pub fn b(&self) -> Vec4 { self.b }
    pub fn c(&self) -> Vec4 { self.c }

    pub fn set_a(&mut self, point: Vec4) { self.a = point; }
    pub fn set_b(&mut self, point: Vec4) { self.b = point; }
    pub fn set_c(&mut self, point: Vec4) { self.c = point; }
}