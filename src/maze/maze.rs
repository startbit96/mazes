use crate::maze::draw::draw_maze;
use crate::maze::generator::MazeGenerator;

pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<bool>>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        Maze {
            width,
            height,
            data: vec![vec![false; width]; height],
        }
    }

    pub fn generate(&mut self, generator: &dyn MazeGenerator) {
        generator.generate(self);
    }

    pub fn draw(&self) {
        draw_maze(self);
    }
}
