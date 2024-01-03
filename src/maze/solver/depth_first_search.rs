use crate::maze::animation::delay;
use crate::maze::direction::{AbsoluteDirection, RelativeDirection};
use crate::maze::draw::{draw_path, highlight_cell};
use crate::maze::maze::Maze;
use crate::maze::solver::{MazeSolver, SOLVING_DELAY};
use std::collections::VecDeque;

pub struct DepthFirstSearch;

impl MazeSolver for DepthFirstSearch {
    fn solve(&self, maze: &Maze, screen: &mut dyn std::io::Write, animate: bool) {
        let pos_start = (1, 1);
        let pos_end = (maze.width - 2, maze.height - 2);
        let mut queue: VecDeque<((usize, usize), AbsoluteDirection, Vec<(usize, usize)>)> =
            VecDeque::new();
        // Add the start position.
        queue.push_back((pos_start, AbsoluteDirection::Down, vec![pos_start]));

        while !queue.is_empty() {
            let (pos, direction, path) = queue.pop_front().unwrap();
            highlight_cell(screen, maze, pos);
            if pos == pos_end {
                draw_path(screen, maze, path);
                break;
            }
            if animate {
                delay(SOLVING_DELAY);
            }
            // From your current direction, first look right, forward and then left.
            // Never go back.
            // In order to first look right and also in the next run to first look right,
            // we need to insert the right one last.
            let possible_directions: Vec<AbsoluteDirection> = vec![
                direction.add_relative_direction(RelativeDirection::Left),
                direction.add_relative_direction(RelativeDirection::Forward),
                direction.add_relative_direction(RelativeDirection::Right),
            ];
            for next_direction in possible_directions.iter() {
                let pos_next = next_direction.apply(pos);
                if maze.is_accessible(pos_next) {
                    let mut path_next = path.clone();
                    path_next.push(pos_next.clone());
                    queue.push_front((pos_next, *next_direction, path_next));
                }
            }
        }
    }
}
