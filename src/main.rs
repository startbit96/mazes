use rand::seq::SliceRandom;
use rand::thread_rng;

const SYMBOL_MAZE_BORDER_CORNER_TOP_LEFT: char = '\u{2554}';
const SYMBOL_MAZE_BORDER_CORNER_TOP_RIGHT: char = '\u{2557}';
const SYMBOL_MAZE_BORDER_CORNER_BOTTOM_LEFT: char = '\u{255A}';
const SYMBOL_MAZE_BORDER_CORNER_BOTTOM_RIGHT: char = '\u{255D}';
const SYMBOL_MAZE_BORDER_TOP: char = '\u{2550}';
const SYMBOL_MAZE_BORDER_BOTTOM: char = '\u{2550}';
const SYMBOL_MAZE_BORDER_LEFT: char = '\u{2551}';
const SYMBOL_MAZE_BORDER_RIGHT: char = '\u{2551}';
const SYMBOL_MAZE_FIELD_ACCESSIBLE: char = ' ';
const SYMBOL_MAZE_FIELD_BLOCKED: char = '\u{2588}';

struct Maze {
    width: usize,
    height: usize,
    data: Vec<Vec<bool>>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        Maze {
            width,
            height,
            data: vec![vec![false; width]; height],
        }
    }

    fn draw(&self) {
        // Draw the maze and also add a border around it.
        print!("{}", SYMBOL_MAZE_BORDER_CORNER_TOP_LEFT);
        for _ in &self.data[0] {
            print!("{}", SYMBOL_MAZE_BORDER_TOP);
        }
        println!("{}", SYMBOL_MAZE_BORDER_CORNER_TOP_RIGHT);
        for row in &self.data {
            print!("{}", SYMBOL_MAZE_BORDER_LEFT);
            for &bit in row {
                print!(
                    "{}",
                    if bit {
                        SYMBOL_MAZE_FIELD_ACCESSIBLE
                    } else {
                        SYMBOL_MAZE_FIELD_BLOCKED
                    }
                );
            }
            println!("{}", SYMBOL_MAZE_BORDER_RIGHT);
        }
        print!("{}", SYMBOL_MAZE_BORDER_CORNER_BOTTOM_LEFT);
        for _ in &self.data[0] {
            print!("{}", SYMBOL_MAZE_BORDER_BOTTOM);
        }
        println!("{}", SYMBOL_MAZE_BORDER_CORNER_BOTTOM_RIGHT);
    }

    fn generate_maze(&mut self) {
        let mut forest: Vec<Vec<(usize, usize)>> = vec![];
        for row in (1..self.height - 1).step_by(2) {
            for col in (1..self.width - 1).step_by(2) {
                forest.push(vec![(row, col)]);
                self.data[row][col] = true;
            }
        }

        let mut edges: Vec<(usize, usize)> = vec![];
        for row in (2..self.height - 1).step_by(2) {
            for col in (1..self.width - 1).step_by(2) {
                edges.push((row, col));
            }
        }

        for row in (1..self.height - 1).step_by(2) {
            for col in (2..self.width - 1).step_by(2) {
                edges.push((row, col));
            }
        }
        edges.shuffle(&mut thread_rng());

        while forest.len() > 1 {
            let (ce_row, ce_col) = edges[0];
            edges.remove(0);
            let mut tree1: usize;
            let mut tree2: usize;

            if ce_row % 2 == 0 {
                tree1 = forest
                    .clone()
                    .into_iter()
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
                    .clone()
                    .into_iter()
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
                    .clone()
                    .into_iter()
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
                    .clone()
                    .into_iter()
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
                self.data[ce_row][ce_col] = true;
            }
        }
    }
}

fn main() {
    let mut maze = Maze::new(81, 41);
    maze.generate_maze();
    maze.draw();
}
