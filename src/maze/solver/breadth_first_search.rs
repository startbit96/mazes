use crate::maze::animation::delay;
use crate::maze::direction::{AbsoluteDirection, RelativeDirection};
use crate::maze::draw::{draw_path, highlight_cell, CellColorType};
use crate::maze::maze::Maze;
use crate::maze::solver::{MazeSolver, SOLVING_DELAY};
use std::collections::{HashSet, VecDeque};

pub struct BreadthFirstSearch;

impl MazeSolver for BreadthFirstSearch {
    fn solve(
        &self,
        maze: &mut Maze,
        screen: &mut dyn std::io::Write,
        animate: bool,
    ) -> (Vec<(usize, usize)>, usize) {
        let mut queue: VecDeque<(
            (usize, usize),
            Option<AbsoluteDirection>,
            Vec<(usize, usize)>,
        )> = VecDeque::new();
        // Add the start position.
        queue.push_back((maze.pos_start, None, vec![maze.pos_start]));

        // Count the inspected cells.
        let mut inspected_cells: HashSet<(usize, usize)> = HashSet::new();

        while !queue.is_empty() {
            let (pos, direction, path) = queue.pop_front().unwrap();
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
                    direction.add_relative_direction(RelativeDirection::Right),
                    direction.add_relative_direction(RelativeDirection::Forward),
                    direction.add_relative_direction(RelativeDirection::Left),
                ]
            } else {
                vec![
                    AbsoluteDirection::Up,
                    AbsoluteDirection::Left,
                    AbsoluteDirection::Down,
                    AbsoluteDirection::Right,
                ]
            };
            for next_direction in possible_directions.iter() {
                let pos_next = next_direction.apply(pos);
                if maze.is_accessible(pos_next) {
                    let mut path_next = path.clone();
                    path_next.push(pos_next.clone());
                    queue.push_back((pos_next, Some(*next_direction), path_next));
                }
            }
        }
        panic!()
    }

    fn to_string(&self) -> String {
        String::from("BFS")
    }
}
