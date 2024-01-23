use crate::maze::animation::delay;
use crate::maze::direction::AbsoluteDirection;
use crate::maze::draw::{draw_character, SYMBOL_MAZE_FIELD_ACCESSIBLE};
use crate::maze::generator::{MazeGenerator, GENERATION_DELAY};
use crate::maze::maze::*;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::io::Write;

pub struct RecursiveBacktracking;

impl MazeGenerator for RecursiveBacktracking {
    fn generate(&self, maze: &mut Maze, screen: &mut dyn Write, animate: bool) {
        /*
        Algorithm:

        1. Randomly choose a starting cell.
        2. Randomly choose a wall at the current cell and open a passage through to any random adjacent
            cell, that has not been visited yet. This is now the current cell.
        3. If all adjacent cells have been visited, back up to the previous and repeat step 2.
        4. Stop when the algorithm has backed all the way up to the starting cell.
        */

        // Get all unvisited cells.
        let mut unvisited_cells: HashSet<(usize, usize)> = HashSet::new();
        for row in (1..maze.height - 1).step_by(2) {
            for col in (1..maze.width - 1).step_by(2) {
                unvisited_cells.insert((row, col));
            }
        }

        // Choose a random starting cell.
        let mut current_cell = unvisited_cells
            .clone()
            .into_iter()
            .collect::<Vec<(usize, usize)>>()
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone();
        maze.data[current_cell.1][current_cell.0] = MAZE_VALUE_ACCESSIBLE;
        if animate {
            draw_character(
                screen,
                maze,
                current_cell,
                SYMBOL_MAZE_FIELD_ACCESSIBLE,
                None,
            );
            delay(GENERATION_DELAY);
        }

        // Remove the starting cell from the unvisited cells.
        unvisited_cells.remove(&current_cell);
        // Keep track of the path you walked.
        let mut path: Vec<(usize, usize)> = Vec::new();
        while !unvisited_cells.is_empty() {
            // Determine the possible directions you can choose from.
            let mut possible_directions: Vec<AbsoluteDirection> = Vec::new();
            // Look to the left.
            if current_cell.0 > 1 && unvisited_cells.contains(&(current_cell.0 - 2, current_cell.1))
            {
                possible_directions.push(AbsoluteDirection::Left);
            }
            // Look to the right.
            if current_cell.0 < maze.width - 2
                && unvisited_cells.contains(&(current_cell.0 + 2, current_cell.1))
            {
                possible_directions.push(AbsoluteDirection::Right);
            }
            // Look to the top.
            if current_cell.1 > 1 && unvisited_cells.contains(&(current_cell.0, current_cell.1 - 2))
            {
                possible_directions.push(AbsoluteDirection::Up);
            }
            // Look to the bottom.
            if current_cell.1 < maze.height - 2
                && unvisited_cells.contains(&(current_cell.0, current_cell.1 + 2))
            {
                possible_directions.push(AbsoluteDirection::Down);
            }
            // If there is no further direction to choose from, go back to the last cell.
            if possible_directions.is_empty() {
                current_cell = path.pop().unwrap();
                continue;
            }
            // Choose a direction to walk to.
            let direction = possible_directions.choose(&mut rand::thread_rng()).unwrap();
            // Remember the current position, add it to the path.
            path.push(current_cell);
            // Go to the next cell. Therefore we need to take two steps.
            for _ in 0..2 {
                current_cell = direction.apply(current_cell);
                maze.data[current_cell.1][current_cell.0] = MAZE_VALUE_ACCESSIBLE;
                if animate {
                    draw_character(
                        screen,
                        maze,
                        current_cell,
                        SYMBOL_MAZE_FIELD_ACCESSIBLE,
                        None,
                    );
                    delay(GENERATION_DELAY);
                }
            }
            unvisited_cells.remove(&current_cell);
        }
    }

    fn to_string(&self) -> String {
        String::from("recursive backtracking")
    }
}
