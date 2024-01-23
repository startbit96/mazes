use crate::maze::animation::delay;
use crate::maze::direction::{AbsoluteDirection, RelativeDirection};
use crate::maze::draw::{draw_character, highlight_cell, CellColorType};
use crate::maze::maze::Maze;
use crate::maze::solver::{MazeSolver, SOLVING_DELAY};
use std::collections::HashSet;

pub struct WallFollower;

impl MazeSolver for WallFollower {
    fn solve(
        &self,
        maze: &mut Maze,
        screen: &mut dyn std::io::Write,
        animate: bool,
    ) -> (Vec<(usize, usize)>, usize) {
        let mut pos_current = maze.pos_start;
        let mut pos_prev;
        let mut direction = match (
            maze.is_accessible((pos_current.0, pos_current.1 - 1)),
            maze.is_accessible((pos_current.0 - 1, pos_current.1)),
            maze.is_accessible((pos_current.0, pos_current.1 + 1)),
            maze.is_accessible((pos_current.0 + 1, pos_current.1)),
        ) {
            (true, _, _, _) => AbsoluteDirection::Up,
            (_, true, _, _) => AbsoluteDirection::Left,
            (_, _, true, _) => AbsoluteDirection::Down,
            (_, _, _, true) => AbsoluteDirection::Right,
            _ => panic!(),
        };

        // Count the inspected cells.
        let mut inspected_cells: HashSet<(usize, usize)> = HashSet::new();

        while pos_current != maze.pos_end {
            inspected_cells.insert(pos_current);
            pos_prev = pos_current;
            // Follow the right wall.
            direction = direction.add_relative_direction(RelativeDirection::Right);
            loop {
                if maze.is_accessible(direction.apply(pos_current)) {
                    break;
                }
                direction = direction.add_relative_direction(RelativeDirection::Left);
            }
            pos_current = direction.apply(pos_current);
            // Highlight the previous cell.
            highlight_cell(screen, maze, pos_prev, CellColorType::InspectedCell);
            // Mark the current cell including the walking direction.
            draw_character(
                screen,
                maze,
                pos_current,
                direction.to_symbol(),
                Some(CellColorType::CurrentCell),
            );
            if animate {
                delay(SOLVING_DELAY);
            }
        }
        // Catch the last one.
        inspected_cells.insert(pos_current);

        (Vec::new(), inspected_cells.len())
    }

    fn to_string(&self) -> String {
        String::from("wall follower")
    }
}
