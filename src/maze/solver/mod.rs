use crate::maze::animation::Delay;
use crate::maze::maze::Maze;
use std::io::Write;

pub const SOLVING_DELAY: Delay = Delay::Long;

pub mod breadth_first_search;
pub mod depth_first_search;
pub mod wall_follower;

pub trait MazeSolver {
    fn solve(&self, maze: &Maze, screen: &mut dyn Write, animate: bool) -> Vec<(usize, usize)>;
}

#[derive(Debug)]
pub enum MazeSolvingAlgorithms {
    BreadthFirstSearch,
    DepthFirstSearch,
    WallFollower,
}

impl MazeSolvingAlgorithms {
    pub fn next(&self) -> Self {
        match self {
            Self::BreadthFirstSearch => Self::DepthFirstSearch,
            Self::DepthFirstSearch => Self::WallFollower,
            Self::WallFollower => Self::BreadthFirstSearch,
        }
    }
}
