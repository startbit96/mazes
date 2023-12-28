use crate::maze::animation::delay;
use crate::maze::draw::{draw_character, SYMBOL_MAZE_FIELD_ACCESSIBLE};
use crate::maze::generator::{MazeGenerator, GENERATION_DELAY};
use crate::maze::maze::Maze;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::Write;

pub struct Kruskal;

impl MazeGenerator for Kruskal {
    fn generate(&self, maze: &mut Maze, screen: &mut dyn Write, animate: bool) {
        // Draw the maze as empty as it is.
        maze.draw(screen, false);

        let mut forest: Vec<Vec<(usize, usize)>> = vec![];

        // Fill the forest with small trees. Each tree contains at the
        // beginning only one cell.
        for row in (1..maze.height - 1).step_by(2) {
            for col in (1..maze.width - 1).step_by(2) {
                forest.push(vec![(row, col)]);
                maze.data[row][col] = true;
                if animate {
                    draw_character(
                        screen,
                        maze,
                        (col as u16, row as u16),
                        SYMBOL_MAZE_FIELD_ACCESSIBLE,
                    );
                    delay(GENERATION_DELAY);
                    screen.flush().unwrap();
                }
            }
        }

        // Get all possible edges. We will not use all but only that much
        // until all our trees in the forest will be connected to one big tree.
        let mut edges: Vec<(usize, usize)> = vec![];
        for row in (2..maze.height - 1).step_by(2) {
            for col in (1..maze.width - 1).step_by(2) {
                edges.push((row, col));
            }
        }
        for row in (1..maze.height - 1).step_by(2) {
            for col in (2..maze.width - 1).step_by(2) {
                edges.push((row, col));
            }
        }
        // Shuffle them.
        edges.shuffle(&mut thread_rng());

        while forest.len() > 1 {
            let (ce_row, ce_col) = edges[0];
            edges.remove(0);
            let mut tree1: usize;
            let mut tree2: usize;

            if ce_row % 2 == 0 {
                tree1 = forest
                    .iter()
                    .enumerate()
                    .map(|(idx, tree)| {
                        if tree.contains(&(ce_row - 1, ce_col)) {
                            idx
                        } else {
                            0
                        }
                    })
                    .sum();

                tree2 = forest
                    .iter()
                    .enumerate()
                    .map(|(idx, tree)| {
                        if tree.contains(&(ce_row + 1, ce_col)) {
                            idx
                        } else {
                            0
                        }
                    })
                    .sum();
            } else {
                tree1 = forest
                    .iter()
                    .enumerate()
                    .map(|(idx, tree)| {
                        if tree.contains(&(ce_row, ce_col - 1)) {
                            idx
                        } else {
                            0
                        }
                    })
                    .sum();

                tree2 = forest
                    .iter()
                    .enumerate()
                    .map(|(idx, tree)| {
                        if tree.contains(&(ce_row, ce_col + 1)) {
                            idx
                        } else {
                            0
                        }
                    })
                    .sum();
            }

            if tree1 != tree2 {
                if tree1 > tree2 {
                    (tree2, tree1) = (tree1, tree2);
                }
                let mut new_tree = forest[tree1].clone();
                new_tree.append(&mut forest[tree2].clone());
                forest.remove(tree2);
                forest.remove(tree1);
                forest.push(new_tree);
                maze.data[ce_row][ce_col] = true;
                if animate {
                    draw_character(
                        screen,
                        maze,
                        (ce_col as u16, ce_row as u16),
                        SYMBOL_MAZE_FIELD_ACCESSIBLE,
                    );
                    delay(GENERATION_DELAY);
                    screen.flush().unwrap();
                }
            }
        }
    }
}
