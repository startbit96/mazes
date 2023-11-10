use rand::seq::SliceRandom;
use rand::thread_rng;

extern crate termion;

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

const TERMION_OFFSET_X: u16 = 1;
const TERMION_OFFSET_Y: u16 = 1;
const BORDER_WIDTH_X: u16 = 11;
const BORDER_WIDTH_Y: u16 = 3;

#[derive(Debug, Default)]
enum MazeGenerationAlgorithm {
    #[default]
    Kruskal,
}

#[derive(Debug)]
enum MazeSolvingAlgorithm {}

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
        // Top border.
        print!(
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(TERMION_OFFSET_X, TERMION_OFFSET_Y)
        );
        print!("{}", SYMBOL_MAZE_BORDER_CORNER_TOP_LEFT);
        for _ in 1..=(self.width + 2 * (BORDER_WIDTH_X as usize - 1)) {
            print!("{}", SYMBOL_MAZE_BORDER_TOP);
        }
        println!("{}", SYMBOL_MAZE_BORDER_CORNER_TOP_RIGHT);

        // Extra space between the maze and the top border (if wanted).
        for extra_space in 1..BORDER_WIDTH_Y {
            println!(
                "{}{}{}",
                SYMBOL_MAZE_BORDER_LEFT,
                termion::cursor::Goto(
                    TERMION_OFFSET_X + 2 * BORDER_WIDTH_X + self.width as u16 - 1,
                    TERMION_OFFSET_Y + extra_space
                ),
                SYMBOL_MAZE_BORDER_RIGHT,
            );
        }

        // Maze together with the border on the left and right.
        for (idx_row, row) in self.data.iter().enumerate() {
            print!(
                "{}{}{}",
                termion::cursor::Goto(
                    TERMION_OFFSET_X,
                    TERMION_OFFSET_Y + BORDER_WIDTH_Y + idx_row as u16
                ),
                SYMBOL_MAZE_BORDER_LEFT,
                termion::cursor::Goto(
                    TERMION_OFFSET_X + BORDER_WIDTH_X,
                    TERMION_OFFSET_Y + BORDER_WIDTH_Y + idx_row as u16
                )
            );
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
            println!(
                "{}{}",
                termion::cursor::Goto(
                    TERMION_OFFSET_X + 2 * BORDER_WIDTH_X + self.width as u16 - 1,
                    TERMION_OFFSET_Y + BORDER_WIDTH_Y + idx_row as u16
                ),
                SYMBOL_MAZE_BORDER_RIGHT
            );
        }

        // Extra space between the maze and the bottom border (if wanted).
        for extra_space in 1..BORDER_WIDTH_Y {
            println!(
                "{}{}{}",
                SYMBOL_MAZE_BORDER_LEFT,
                termion::cursor::Goto(
                    TERMION_OFFSET_X + 2 * BORDER_WIDTH_X + self.width as u16 - 1,
                    TERMION_OFFSET_Y + BORDER_WIDTH_Y + self.height as u16 + extra_space - 1
                ),
                SYMBOL_MAZE_BORDER_RIGHT,
            );
        }

        // Bottom border.
        print!("{}", SYMBOL_MAZE_BORDER_CORNER_BOTTOM_LEFT);
        for _ in 1..=(self.width + 2 * (BORDER_WIDTH_X as usize - 1)) {
            print!("{}", SYMBOL_MAZE_BORDER_BOTTOM);
        }
        println!("{}", SYMBOL_MAZE_BORDER_CORNER_BOTTOM_RIGHT);
    }

    fn generate_maze(&mut self, algorithm: MazeGenerationAlgorithm) {
        match algorithm {
            MazeGenerationAlgorithm::Kruskal => self.generate_maze_kruskal(),
        }
    }

    fn generate_maze_kruskal(&mut self) {
        let mut forest: Vec<Vec<(usize, usize)>> = vec![];

        // Fill the forest with small trees. Each tree contains at the
        // beginning only one cell.
        for row in (1..self.height - 1).step_by(2) {
            for col in (1..self.width - 1).step_by(2) {
                forest.push(vec![(row, col)]);
                self.data[row][col] = true;
            }
        }

        // Get all possible edges. We will not use all but only that much
        // until all our trees in the forest will be connected to one big tree.
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
                self.data[ce_row][ce_col] = true;
            }
        }
    }
}

fn main() {
    let mut maze = Maze::new(41, 21);
    maze.generate_maze(MazeGenerationAlgorithm::default());
    maze.draw();
}
