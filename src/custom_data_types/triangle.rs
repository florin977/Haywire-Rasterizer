use super::point::Point;

#[derive(Clone, Copy)]
pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Self { a, b, c }
    }

    pub fn a(&self) -> Point { self.a }
    pub fn b(&self) -> Point { self.b }
    pub fn c(&self) -> Point { self.c }

    pub fn set_a(&mut self, point: Point) { self.a = point; }
    pub fn set_b(&mut self, point: Point) { self.b = point; }
    pub fn set_c(&mut self, point: Point) { self.c = point; }
}