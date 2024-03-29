use crate::maze::animation::Delay;
use crate::maze::maze::Maze;
use std::io::Write;

pub const GENERATION_DELAY: Delay = Delay::Long;

pub mod kruskal;
pub mod recursive_backtracking;
pub mod wilson;

pub use kruskal::Kruskal;
pub use recursive_backtracking::RecursiveBacktracking;
pub use wilson::Wilson;

pub trait MazeGenerator: Send + Sync {
    fn generate(&self, maze: &mut Maze, screen: &mut dyn Write, animate: bool);

    fn to_string(&self) -> String;
}

#[derive(Debug)]
pub enum MazeGenerationAlgorithms {
    Kruskal,
    RecursiveBacktracking,
    Wilson,
}

impl MazeGenerationAlgorithms {
    pub fn next(&self) -> Self {
        match self {
            Self::Kruskal => Self::RecursiveBacktracking,
            Self::RecursiveBacktracking => Self::Wilson,
            Self::Wilson => Self::Kruskal,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Self::Kruskal => "kruskal",
            Self::RecursiveBacktracking => "recursive backtracking",
            Self::Wilson => "wilson",
        }
    }
}
