use crate::container::{Container, Point};

#[derive(Debug, Clone)]
pub struct GameObject {
    container: Container,
    symbol: String,
    destroyed: bool,
}

impl GameObject {
    pub fn new(container: Container, symbol: &str) -> Self {
        Self {
            container,
            symbol: symbol.to_string(),
            destroyed: false,
        }
    }

    pub fn get_container(&self) -> Container {
        self.container.clone()
    }

    pub fn get_width(&self) -> usize {
        self.container.bottom.x - self.container.top.x
    }

    pub fn get_height(&self) -> usize {
        self.container.bottom.y - self.container.top.y
    }

    pub fn destroy(&mut self) {
        self.destroyed = true;
    }
    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }

    pub fn set_pos(&mut self, point: &Point) {
        self.container.top = point.clone();
        self.container.bottom = point.clone();
    }

    pub fn get_pos(&self) -> Point {
        self.container.top.clone()
    }
}
