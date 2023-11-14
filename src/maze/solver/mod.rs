use crate::maze::maze::Maze;

pub trait MazeSolver {
    fn solve(&self, maze: &mut Maze);
}
