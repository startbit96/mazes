use crate::maze::animation::Delay;
use crate::maze::maze::Maze;
use std::io::Write;

pub const SOLVING_DELAY: Delay = Delay::Long;

pub mod breadth_first_search;
pub mod depth_first_search;
pub mod wall_follower;

pub use breadth_first_search::BreadthFirstSearch;
pub use depth_first_search::DepthFirstSearch;
pub use wall_follower::WallFollower;

pub trait MazeSolver {
    fn solve(
        &self,
        maze: &mut Maze,
        screen: &mut dyn Write,
        animate: bool,
    ) -> (Vec<(usize, usize)>, usize);
}

#[derive(Debug, PartialEq, Eq)]
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

    pub fn to_string(&self) -> &str {
        match self {
            Self::BreadthFirstSearch => "BFS",
            Self::DepthFirstSearch => "DFS",
            Self::WallFollower => "wall follower",
        }
    }
}
