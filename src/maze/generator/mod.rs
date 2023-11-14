use crate::maze::maze::Maze;

pub trait MazeGenerator {
    fn generate(&self, maze: &mut Maze);
}

pub mod kruskal;
