use crate::maze::animation::delay;
use crate::maze::direction::{AbsoluteDirection, RelativeDirection};
use crate::maze::draw::{draw_character, draw_path, highlight_cell};
use crate::maze::maze::Maze;
use crate::maze::solver::{MazeSolver, SOLVING_DELAY};

pub struct WallFollower;

impl MazeSolver for WallFollower {
    fn solve(
        &self,
        maze: &Maze,
        screen: &mut dyn std::io::Write,
        animate: bool,
    ) -> Vec<(usize, usize)> {
        let mut pos_current = (1, 1);
        let mut pos_prev;
        let mut direction = if maze.is_accessible((1, 2)) {
            AbsoluteDirection::Down
        } else {
            AbsoluteDirection::Right
        };
        let pos_end = (maze.width - 2, maze.height - 2);
        while pos_current != pos_end {
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
            highlight_cell(screen, maze, pos_prev);
            // Mark the current cell including the walking direction.
            draw_character(screen, maze, pos_current, direction.to_symbol(), true);
            if animate {
                delay(SOLVING_DELAY);
            }
        }
        Vec::new()
    }
}
