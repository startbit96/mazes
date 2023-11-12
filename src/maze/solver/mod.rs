use crate::maze::Maze;

pub trait MazeSolver {
    fn solve(&self, maze: &mut Maze);
}
