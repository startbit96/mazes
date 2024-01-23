use crate::maze::generator::MazeGenerator;
use crate::maze::maze::Maze;
use crate::maze::maze_collection::MazeCollection;
use crate::maze::solver::MazeSolver;
use std::io::Write;

pub enum MazeContainer {
    SingleMaze(Maze),
    MultipleMazes(MazeCollection),
}

impl MazeContainer {
    pub fn generate(
        &mut self,
        generator: &dyn MazeGenerator,
        screen: &mut dyn Write,
        animate: bool,
    ) {
        if let MazeContainer::SingleMaze(ref mut maze) = self {
            maze.generate(generator, screen, animate);
        } else if let MazeContainer::MultipleMazes(ref mut maze_collection) = self {
            maze_collection.generate(generator, screen, animate);
        }
    }

    pub fn solve(
        &mut self,
        solver: &dyn MazeSolver,
        screen: &mut dyn Write,
        animate: bool,
    ) -> (Vec<(usize, usize)>, usize) {
        if let MazeContainer::SingleMaze(ref mut maze) = self {
            maze.solve(solver, screen, animate)
        } else if let MazeContainer::MultipleMazes(ref mut maze_collection) = self {
            maze_collection.solve(solver, screen, animate)
        } else {
            panic!()
        }
    }

    pub fn draw(
        &mut self,
        screen: &mut dyn Write,
        show_graph: bool,
        show_background_graph: bool,
        show_binary_representation: bool,
        show_background_binary_representation: bool,
    ) {
        if let MazeContainer::SingleMaze(ref mut maze) = self {
            maze.draw(
                screen,
                show_graph,
                show_background_graph,
                show_binary_representation,
                show_background_binary_representation,
            );
        } else if let MazeContainer::MultipleMazes(ref mut maze_collection) = self {
            maze_collection.draw(
                screen,
                show_graph,
                show_background_graph,
                show_binary_representation,
                show_background_binary_representation,
            );
        }
    }

    pub fn change_size(&mut self, change_in_width: isize, change_in_height: isize) -> bool {
        if let MazeContainer::SingleMaze(ref mut maze) = self {
            maze.change_size(
                (maze.width as isize + change_in_width) as usize,
                (maze.height as isize + change_in_height) as usize,
            )
        } else if let MazeContainer::MultipleMazes(ref mut maze_collection) = self {
            maze_collection.change_size(
                (maze_collection.mazes[0].width as isize + change_in_width) as usize,
                (maze_collection.mazes[0].height as isize + change_in_height) as usize,
            )
        } else {
            panic!()
        }
    }

    pub fn reorder(&mut self) {
        if let MazeContainer::MultipleMazes(ref mut maze_collection) = self {
            maze_collection.reorder();
        }
    }
}
