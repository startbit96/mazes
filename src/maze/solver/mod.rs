use crate::maze::animation::Delay;
use crate::maze::maze::Maze;
use std::io::Write;

pub const SOLVING_DELAY: Delay = Delay::Middle;

pub mod a_star;
pub mod a_star_weighted;
pub mod breadth_first_search;
pub mod depth_first_search;
pub mod greedy_best_first_search;
pub mod wall_follower;

pub use a_star::AStar;
pub use a_star_weighted::AStarWeighted;
pub use breadth_first_search::BreadthFirstSearch;
pub use depth_first_search::DepthFirstSearch;
pub use greedy_best_first_search::GreedyBestFirstSearch;
pub use wall_follower::WallFollower;

pub trait MazeSolver {
    fn solve(
        &self,
        maze: &mut Maze,
        screen: &mut dyn Write,
        animate: bool,
    ) -> (Vec<(usize, usize)>, usize);

    fn to_string(&self) -> String;
}

#[derive(Debug, PartialEq, Eq)]
pub enum MazeSolvingAlgorithms {
    AStar,
    AStarWeighted,
    BreadthFirstSearch,
    DepthFirstSearch,
    GreedyBestFirstSearch,
    WallFollower,
}

impl MazeSolvingAlgorithms {
    pub fn next(&self) -> Self {
        match self {
            Self::AStar => Self::AStarWeighted,
            Self::AStarWeighted => Self::BreadthFirstSearch,
            Self::BreadthFirstSearch => Self::DepthFirstSearch,
            Self::DepthFirstSearch => Self::GreedyBestFirstSearch,
            Self::GreedyBestFirstSearch => Self::WallFollower,
            Self::WallFollower => Self::AStar,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Self::AStar => "A*",
            Self::AStarWeighted => "A* weighted",
            Self::BreadthFirstSearch => "BFS",
            Self::DepthFirstSearch => "DFS",
            Self::GreedyBestFirstSearch => "greedy best-first search",
            Self::WallFollower => "wall follower",
        }
    }
}
