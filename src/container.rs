#[derive(Debug, Clone)]
pub struct Container {
    pub top: Point,
    pub bottom: Point,

    pub padding_vertical: usize,
    pub padding_horizontal: usize,
}

impl Container {
    pub fn new(top: Point, bottom: Point) -> Self {
        Self {
            top,
            bottom,
            padding_horizontal: 0,
            padding_vertical: 0,
        }
    }
    pub fn get_width(&self) -> usize {
        self.bottom.x - self.top.x
    }

    pub fn get_height(&self) -> usize {
        self.bottom.y - self.top.y
    }
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}
