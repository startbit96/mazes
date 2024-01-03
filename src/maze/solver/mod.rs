use crate::maze::animation::Delay;
use crate::maze::maze::Maze;
use std::io::Write;

use self::breadth_first_search::BreadthFirstSearch;

pub const SOLVING_DELAY: Delay = Delay::Short;

pub mod breadth_first_search;
pub mod depth_first_search;

pub trait MazeSolver {
    fn solve(&self, maze: &Maze, screen: &mut dyn Write, animate: bool);
}

#[derive(Debug)]
pub enum MazeSolvingAlgorithms {
    BreadthFirstSearch,
    DepthFirstSearch,
}

impl MazeSolvingAlgorithms {
    pub fn next(&self) -> Self {
        match self {
            Self::BreadthFirstSearch => Self::DepthFirstSearch,
            Self::DepthFirstSearch => Self::BreadthFirstSearch,
        }
    }
}
