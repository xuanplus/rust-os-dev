#[derive(Debug, Clone, Copy)]
pub struct Point(usize, usize);

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self(x, y)
    }
}

impl Into<(usize, usize)> for Point {
    fn into(self) -> (usize, usize) {
        (self.0, self.1)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color(u8, u8, u8);

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    pub fn r(&self) -> u8 {
        self.0
    }

    pub fn g(&self) -> u8 {
        self.1
    }

    pub fn b(&self) -> u8 {
        self.2
    }
}

impl Into<[u8; 3]> for Color {
    fn into(self) -> [u8; 3] {
        [self.0, self.1, self.2]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pixel(Point, Color);

impl Pixel {
    pub fn new(point: Point, color: Color) -> Self {
        Self(point, color)
    } 

    pub fn point(&self) -> Point {
        self.0
    }

    pub fn color(&self) -> Color {
        self.1
    }
}