use crate::maze::animation::delay;
use crate::maze::draw::{
    draw_character, get_unique_colors, highlight_cell, highlight_cells_by_rgb_color,
    SYMBOL_MAZE_FIELD_ACCESSIBLE,
};
use crate::maze::generator::{MazeGenerator, GENERATION_DELAY};
use crate::maze::maze::*;
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
                maze.data[row][col] = MAZE_VALUE_ACCESSIBLE;
                if animate {
                    draw_character(
                        screen,
                        maze,
                        (col, row),
                        SYMBOL_MAZE_FIELD_ACCESSIBLE,
                        false,
                    );
                    delay(GENERATION_DELAY);
                    screen.flush().unwrap();
                }
            }
        }

        // Now we have all the trees visualized. Give them all a (almost) unique color.
        let mut colors = if animate {
            get_unique_colors(forest.len())
        } else {
            Vec::new()
        };
        if animate {
            for (idx, tree) in forest.iter().enumerate() {
                highlight_cells_by_rgb_color(screen, maze, tree.clone(), colors[idx]);
            }
            screen.flush().unwrap();
            delay(GENERATION_DELAY);
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
                new_tree.push((ce_row, ce_col));
                forest.remove(tree2);
                forest.remove(tree1);
                forest.push(new_tree.clone());
                maze.data[ce_row][ce_col] = MAZE_VALUE_ACCESSIBLE;
                if animate {
                    colors.push(colors[tree2]);
                    colors.remove(tree2);
                    colors.remove(tree1);
                    highlight_cells_by_rgb_color(screen, maze, new_tree, colors[colors.len() - 1]);
                    delay(GENERATION_DELAY);
                    screen.flush().unwrap();
                }
            }
        }
        if animate {
            delay(GENERATION_DELAY);
        }
    }
}
