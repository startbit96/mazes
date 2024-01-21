use crate::maze::generator::MazeGenerator;
use crate::maze::maze::*;
use crate::maze::solver::MazeSolver;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct MazeCollection {
    pub mazes: Vec<Maze>,
    pub number_of_mazes: usize,
}

impl MazeCollection {
    pub fn new(max_width: usize, max_height: usize, number_of_mazes: usize) -> Self {
        let mut mazes: Vec<Maze> = Vec::new();
        for idx in 1..=number_of_mazes {
            mazes.push(Maze::new(max_width, max_height, (idx, number_of_mazes)));
        }
        Self {
            mazes,
            number_of_mazes,
        }
    }

    pub fn change_size(&mut self, width: usize, height: usize) -> bool {
        self.mazes
            .iter_mut()
            .all(|maze| maze.change_size(width, height))
    }

    pub fn generate(
        &mut self,
        generator: &dyn MazeGenerator,
        screen: &mut dyn Write,
        animate: bool,
    ) {
        self.mazes
            .iter_mut()
            .for_each(|maze| maze.generate(generator, screen, animate));
    }

    pub fn solve(
        &self,
        solver: &dyn MazeSolver,
        screen: &mut dyn Write,
        animate: bool,
    ) -> (Vec<(usize, usize)>, usize) {
        (
            Vec::new(),
            self.mazes
                .iter()
                .map(|maze| maze.solve(solver, screen, animate))
                .map(|(_, number_of_inspected_cells)| number_of_inspected_cells)
                .sum(),
        )
    }

    pub fn draw(&self, screen: &mut dyn Write, show_graph: bool) {
        self.mazes
            .iter()
            .for_each(|maze| maze.draw(screen, show_graph));
    }

    pub fn show_binary_representation(&self, screen: &mut dyn Write) {
        self.mazes
            .iter()
            .for_each(|maze| maze.show_binary_representation(screen));
    }
}
