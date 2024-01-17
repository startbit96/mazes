use crate::maze::animation::delay;
use crate::maze::direction::AbsoluteDirection;
use crate::maze::draw::{draw_character, SYMBOL_MAZE_FIELD_ACCESSIBLE};
use crate::maze::generator::{MazeGenerator, GENERATION_DELAY};
use crate::maze::maze::*;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::io::Write;

pub struct Wilson;

impl MazeGenerator for Wilson {
    fn generate(&self, maze: &mut Maze, screen: &mut dyn Write, animate: bool) {
        /*
        Algorithm:
        1. Choose a random cell and add it to the Uniform Spanning Tree (UST).
        2. Select any cell that is not in the UST and perform a random walk until you find a cell that is.
        3. Add the cells and walls visited in the random walk to the UST.
        4. Repeat steps 2 and 3 until all cells have been added to the UST.
        */

        // Get all unvisited cells.
        let mut unvisited_cells: HashSet<(usize, usize)> = HashSet::new();
        for row in (1..maze.height - 1).step_by(2) {
            for col in (1..maze.width - 1).step_by(2) {
                unvisited_cells.insert((row, col));
            }
        }

        // Choose a random starting cell.
        let start_cell = unvisited_cells
            .clone()
            .into_iter()
            .collect::<Vec<(usize, usize)>>()
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone();
        maze.data[start_cell.1][start_cell.0] = MAZE_VALUE_ACCESSIBLE;
        // Remove the starting cell from the unvisited cells.
        unvisited_cells.remove(&start_cell);
        if animate {
            draw_character(
                screen,
                maze,
                start_cell,
                SYMBOL_MAZE_FIELD_ACCESSIBLE,
                false,
            );
            delay(GENERATION_DELAY);
            screen.flush().unwrap();
        }

        // Choose a random cell to start the random walk from.
        let mut current_cell = unvisited_cells
            .clone()
            .into_iter()
            .collect::<Vec<(usize, usize)>>()
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone();

        // Keep track of the path you walked.
        let mut path: Vec<(usize, usize)> = vec![current_cell];

        loop {
            // Determine the possible directions you can choose from.
            let mut possible_directions: Vec<AbsoluteDirection> = Vec::new();
            // Look to the left.
            if current_cell.0 > 1 && !path.contains(&(current_cell.0 - 2, current_cell.1)) {
                possible_directions.push(AbsoluteDirection::Left);
            }
            // Look to the right.
            if current_cell.0 < maze.width - 2
                && !path.contains(&(current_cell.0 + 2, current_cell.1))
            {
                possible_directions.push(AbsoluteDirection::Right);
            }
            // Look to the top.
            if current_cell.1 > 1 && !path.contains(&(current_cell.0, current_cell.1 - 2)) {
                possible_directions.push(AbsoluteDirection::Up);
            }
            // Look to the bottom.
            if current_cell.1 < maze.height - 2
                && !path.contains(&(current_cell.0, current_cell.1 + 2))
            {
                possible_directions.push(AbsoluteDirection::Down);
            }
            // There should always be a direction to choose from. But if we end up
            // in a corner and cannot go anywhere, drop this random walk.
            if possible_directions.is_empty() {
                current_cell = unvisited_cells
                    .clone()
                    .into_iter()
                    .collect::<Vec<(usize, usize)>>()
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .clone();
                path.clear();
                path.push(current_cell);
                continue;
            }
            // Choose a direction to walk to.
            let direction = possible_directions.choose(&mut rand::thread_rng()).unwrap();
            // Go to the next cell. Therefore we need to take two steps.
            for _ in 0..2 {
                current_cell = direction.apply(current_cell);
                path.push(current_cell);
            }
            // If this is part of the UST, get a new random cell to start a random walk from.
            if !unvisited_cells.contains(&current_cell) {
                for pos in path.iter() {
                    maze.data[pos.1][pos.0] = MAZE_VALUE_ACCESSIBLE;
                    if animate {
                        draw_character(screen, maze, *pos, SYMBOL_MAZE_FIELD_ACCESSIBLE, false);
                        delay(GENERATION_DELAY);
                        screen.flush().unwrap();
                    }
                }
                unvisited_cells.retain(|&x| !path.contains(&x));
                if unvisited_cells.is_empty() {
                    break;
                }
                current_cell = unvisited_cells
                    .clone()
                    .into_iter()
                    .collect::<Vec<(usize, usize)>>()
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .clone();
                path.clear();
                path.push(current_cell);
            }
        }
    }
}
