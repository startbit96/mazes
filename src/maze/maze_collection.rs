use crate::maze::animation::*;
use crate::maze::direction::AbsoluteDirection;
use crate::maze::draw::*;
use crate::maze::generator::MazeGenerator;
use crate::maze::maze::*;
use crate::maze::path::apply_step;
use crate::maze::path::get_solving_sequence;
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
        self.mazes.iter_mut().for_each(|maze| maze.erase(screen));
        self.mazes
            .iter_mut()
            .for_each(|maze| maze.generate(generator, screen, animate));
    }

    pub fn solve(
        &mut self,
        solver: &dyn MazeSolver,
        screen: &mut dyn Write,
        animate: bool,
    ) -> (Vec<(usize, usize)>, usize) {
        // Reset the start positions and redraw.
        self.mazes
            .iter_mut()
            .for_each(|maze| maze.reset_start_end_position());
        self.draw(screen, false, false, false);
        // Returns the path and the number of inspected cells.
        let mut path: Vec<(usize, usize)> = Vec::new();
        let mut number_of_inspected_cells = 0;
        let mut pos_shift = (0, 0);
        let mut current_positions: Vec<(usize, usize)> =
            self.mazes.iter().map(|maze| maze.pos_start).collect();
        for idx in 0..self.mazes.len() {
            if idx > 0 {
                // Redraw the last maze.
                self.mazes[idx - 1].draw(screen, false, false, false);
            }
            // Solve the maze.
            let (mut sub_path, sub_inspected_cells) =
                self.mazes[idx].solve(solver, screen, animate);
            // Wait a little bit and then redraw the maze.
            if animate {
                delay(Delay::VeryLong);
            }
            self.mazes[idx].draw(screen, false, false, false);
            // Apply the solving sequence to all previous mazes and update their resulting position.
            // Apply the solving sequence to all upcoming mazes and update their start position.
            let solving_sequence = get_solving_sequence(&sub_path);
            for c in solving_sequence.iter() {
                for idx_other in 0..self.mazes.len() {
                    if idx == idx_other {
                        draw_path(screen, &self.mazes[idx], sub_path.clone(), None);
                    }
                    let direction = AbsoluteDirection::from_char(*c);
                    let pos_next =
                        apply_step(&self.mazes[idx_other], current_positions[idx_other], *c);
                    // Erase the last positions marker and draw the current marker.
                    draw_character(
                        screen,
                        &self.mazes[idx_other],
                        current_positions[idx_other],
                        if current_positions[idx_other] == self.mazes[idx_other].pos_start {
                            SYMBOL_MAZE_POS_START
                        } else if current_positions[idx_other] == self.mazes[idx_other].pos_end {
                            SYMBOL_MAZE_POS_END
                        } else {
                            SYMBOL_MAZE_FIELD_ACCESSIBLE
                        },
                        None,
                    );
                    draw_character(
                        screen,
                        &self.mazes[idx_other],
                        pos_next,
                        direction.to_symbol(),
                        None,
                    );
                    current_positions[idx_other] = pos_next;
                    if animate {
                        delay(Delay::Long);
                    }
                }
            }
            // Update the start position of the upcoming mazes.
            for idx_other in (idx + 1)..self.mazes.len() {
                self.mazes[idx_other].pos_start = current_positions[idx_other];
                self.mazes[idx_other].draw(screen, false, false, false);
            }
            // In order to later create the solving sequence, the consecutive positions
            // are only allowed to be one step away from eachother. So we need to move
            // the points from this path a little bit.
            if idx > 0 {
                pos_shift = (
                    pos_shift.0 + (self.mazes[idx - 1].pos_end.0 - self.mazes[idx].pos_start.0),
                    pos_shift.1 + (self.mazes[idx - 1].pos_end.1 - self.mazes[idx].pos_start.1),
                );
                // Also remove the first one.
                sub_path.remove(0);
            }
            sub_path
                .iter_mut()
                .for_each(|pos| *pos = (pos.0 + pos_shift.0, pos.1 + pos_shift.1));
            path.append(&mut sub_path.clone());
            number_of_inspected_cells += sub_inspected_cells;
        }
        // At the end, redraw all mazes and mark the final position.
        self.draw(screen, false, false, false);
        for idx in 0..self.mazes.len() {
            draw_character(
                screen,
                &self.mazes[idx],
                current_positions[idx],
                SYMBOL_MAZE_POS_CURRENT,
                None,
            );
        }
        (path, number_of_inspected_cells)
    }

    pub fn draw(
        &self,
        screen: &mut dyn Write,
        show_graph: bool,
        show_binary_presentation: bool,
        highlight_binary_presentation: bool,
    ) {
        self.mazes.iter().for_each(|maze| {
            maze.draw(
                screen,
                show_graph,
                show_binary_presentation,
                highlight_binary_presentation,
            )
        });
    }

    pub fn reorder(&mut self) {
        self.mazes
            .iter_mut()
            .for_each(|maze| maze.reset_start_end_position());
        self.mazes.rotate_left(1);
        self.mazes
            .iter_mut()
            .enumerate()
            .for_each(|(idx, maze)| maze.collection_position = (idx + 1, self.number_of_mazes));
    }
}
