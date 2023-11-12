use crate::maze::Maze;

pub trait MazeGenerator {
    fn generate(&self, maze: &mut Maze);
}

pub mod kruskal;

pub use kruskal::Kruskal;
