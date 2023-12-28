use crate::maze::{draw::*, generator::MazeGenerator};
use std::io::Write;

const MAZE_EDGE_LENGTH_MIN: usize = 11;
const MAZE_DEFAULT_WIDTH: usize = 21;
const MAZE_DEFAULT_HEIGHT: usize = 21;
const FORCE_SQUARE_MAZES: bool = true;

pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub max_width: usize,
    pub max_height: usize,
    pub data: Vec<Vec<bool>>,
    pub is_node: Vec<Vec<bool>>,
}

impl Maze {
    pub fn new(max_width: usize, max_height: usize) -> Self {
        // The max width and max height are were given by the terminal ui.
        // Make sure these numbers are odd.
        let max_width = match max_width % 2 {
            0 => max_width - 1,
            1 => max_width,
            _ => unreachable!(),
        };
        let max_height = match max_height % 2 {
            0 => max_height - 1,
            1 => max_height,
            _ => unreachable!(),
        };
        let mut width = Maze::check_edge_length(MAZE_DEFAULT_WIDTH, max_width);
        let mut height = Maze::check_edge_length(MAZE_DEFAULT_HEIGHT, max_height);
        if FORCE_SQUARE_MAZES {
            width = width.min(height);
            height = width;
        }
        Maze {
            width,
            height,
            max_width,
            max_height,
            data: vec![vec![false; height]; height],
            is_node: vec![vec![false; width]; height],
        }
    }

    fn check_edge_length(edge_length: usize, max_length: usize) -> usize {
        if edge_length < MAZE_EDGE_LENGTH_MIN {
            MAZE_EDGE_LENGTH_MIN
        } else if edge_length > max_length {
            max_length
        } else if edge_length % 2 == 0 {
            edge_length + 1
        } else {
            edge_length
        }
    }

    pub fn change_size(&mut self, width: usize, height: usize) -> bool {
        let mut width = Maze::check_edge_length(width, self.max_width);
        let mut height = Maze::check_edge_length(height, self.max_height);
        if FORCE_SQUARE_MAZES {
            width = width.min(height);
            height = width;
        }
        if width == self.width && height == self.height {
            // Cannot make the mazer smaller / bigger.
            return false;
        }
        self.width = width;
        self.height = height;
        self.data = vec![vec![false; width]; height];
        self.is_node = vec![vec![false; width]; height];
        return true;
    }

    fn reset(&mut self) {
        for row in &mut self.data {
            for value in row {
                *value = false;
            }
        }
    }

    pub fn generate(
        &mut self,
        generator: &dyn MazeGenerator,
        screen: &mut dyn Write,
        animate: bool,
    ) {
        self.reset();
        generator.generate(self, screen, animate);
        self.generate_graph();
    }

    pub fn draw(&self, screen: &mut dyn Write, show_graph: bool) {
        draw_maze(screen, self, show_graph);
    }

    pub fn erase(&self, screen: &mut dyn Write) {
        erase_maze(screen, self);
    }

    pub fn generate_graph(&mut self) {
        for (row, data_row) in self.data.iter().enumerate() {
            for (col, datum) in data_row.iter().enumerate() {
                if datum == &false {
                    self.is_node[row][col] = false;
                } else {
                    self.is_node[row][col] = match (
                        self.data[row - 1][col],
                        self.data[row + 1][col],
                        self.data[row][col - 1],
                        self.data[row][col + 1],
                    ) {
                        // (left, right, up, down)
                        // curve.
                        (false, true, false, true) => true,
                        (true, false, false, true) => true,
                        (false, true, true, false) => true,
                        (true, false, true, false) => true,
                        // dead end.
                        (false, false, false, true) => true,
                        (false, false, true, false) => true,
                        (false, true, false, false) => true,
                        (true, false, false, false) => true,
                        // crossway.
                        (true, true, true, false) => true,
                        (true, true, false, true) => true,
                        (true, false, true, true) => true,
                        (false, true, true, true) => true,
                        (true, true, true, true) => true,
                        _ => false,
                    };
                }
            }
        }
    }
}
