use crate::maze::animation::Delay;
use crate::maze::maze::Maze;
use std::io::Write;

pub const GENERATION_DELAY: Delay = Delay::Middle;

pub mod kruskal;
pub mod recursive_backtracking;

pub trait MazeGenerator {
    fn generate(&self, maze: &mut Maze, screen: &mut dyn Write, animate: bool);
}

#[derive(Debug)]
pub enum MazeGenerationAlgorithms {
    Kruskal,
    RecursiveBacktracking,
}

impl MazeGenerationAlgorithms {
    pub fn next(&self) -> Self {
        match self {
            Self::Kruskal => Self::RecursiveBacktracking,
            Self::RecursiveBacktracking => Self::Kruskal,
        }
    }
}
