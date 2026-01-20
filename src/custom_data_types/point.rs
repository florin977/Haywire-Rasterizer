#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn x(&self) -> i32 { self.x }
    pub fn y(&self) -> i32 { self.y }
    pub fn z(&self) -> i32 { self.z }

    pub fn set_x(&mut self, x: i32) { self.x = x; }
    pub fn set_y(&mut self, y: i32) { self.y = y; }
    pub fn set_z(&mut self, z: i32) { self.z = z; }
}