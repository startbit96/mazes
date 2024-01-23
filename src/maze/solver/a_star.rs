use crate::maze::animation::delay;
use crate::maze::direction::{AbsoluteDirection, RelativeDirection};
use crate::maze::draw::{draw_path, highlight_cell, CellColorType};
use crate::maze::maze::Maze;
use crate::maze::path::calculate_manhattan_distance;
use crate::maze::solver::{MazeSolver, SOLVING_DELAY};
use std::collections::{BTreeSet, HashSet};

pub struct AStar;

impl MazeSolver for AStar {
    fn solve(
        &self,
        maze: &mut Maze,
        screen: &mut dyn std::io::Write,
        animate: bool,
    ) -> (Vec<(usize, usize)>, usize) {
        let mut queue: BTreeSet<(
            usize,
            (usize, usize),
            Option<AbsoluteDirection>,
            Vec<(usize, usize)>,
        )> = BTreeSet::new();
        // Add the start position.
        queue.insert((
            calculate_manhattan_distance(maze.pos_start, maze.pos_end),
            maze.pos_start,
            None,
            vec![maze.pos_start],
        ));

        // Count the inspected cells.
        let mut inspected_cells: HashSet<(usize, usize)> = HashSet::new();

        while !queue.is_empty() {
            let (_, pos, direction, path) = queue.pop_first().unwrap();
            inspected_cells.insert(pos);
            highlight_cell(screen, maze, pos, CellColorType::CurrentCell);
            if pos == maze.pos_end {
                draw_path(screen, maze, path.clone(), Some(CellColorType::Path));
                return (path, inspected_cells.len());
            }
            if animate {
                delay(SOLVING_DELAY);
            }
            highlight_cell(screen, maze, pos, CellColorType::InspectedCell);
            // From your current direction, first look right, forward and then left.
            // Never go back.
            let possible_directions: Vec<AbsoluteDirection> = if let Some(direction) = direction {
                vec![
                    direction.add_relative_direction(RelativeDirection::Left),
                    direction.add_relative_direction(RelativeDirection::Forward),
                    direction.add_relative_direction(RelativeDirection::Right),
                ]
            } else {
                vec![
                    AbsoluteDirection::Right,
                    AbsoluteDirection::Down,
                    AbsoluteDirection::Left,
                    AbsoluteDirection::Up,
                ]
            };

            for next_direction in possible_directions.iter() {
                let pos_next = next_direction.apply(pos);
                if maze.is_accessible(pos_next) {
                    let mut path_next = path.clone();
                    path_next.push(pos_next.clone());
                    queue.insert((
                        (path_next.len() - 1)
                            + calculate_manhattan_distance(pos_next, maze.pos_end),
                        pos_next,
                        Some(*next_direction),
                        path_next,
                    ));
                }
            }
        }
        panic!()
    }

    fn to_string(&self) -> String {
        String::from("A*")
    }
}
