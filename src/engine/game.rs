use crate::engine::{
    sand::Sand,
};

pub struct Game {
    pub sand: Vec<Sand>,
}

impl Game {
    pub fn new()->Self{
        Self{sand:vec![]}
    }

    pub fn add_sand(&mut self, s:Sand) {
        self.sand.push(s);
    }
}

