use crate::maze::draw::*;
use crate::maze::generator::MazeGenerator;
use crate::maze::solver::MazeSolver;
use rand::seq::SliceRandom;
use std::io::Write;

const MAZE_EDGE_LENGTH_MIN: usize = 11;
const MAZE_DEFAULT_WIDTH: usize = 21;
const MAZE_DEFAULT_HEIGHT: usize = 21;
const FORCE_SQUARE_MAZES: bool = true;

pub const MAZE_VALUE_ACCESSIBLE: bool = false;
pub const MAZE_VALUE_BLOCKED: bool = true;

#[derive(Debug, Clone)]
pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub pos_start: (usize, usize),
    pub pos_end: (usize, usize),
    pub max_width: usize,
    pub max_height: usize,
    pub data: Vec<Vec<bool>>,
    pub is_node: Vec<Vec<bool>>,
    pub collection_position: (usize, usize), // (pos, number of mazes), pos starts at 1
    pub is_generated: bool,
}

impl Maze {
    pub fn new(max_width: usize, max_height: usize, collection_position: (usize, usize)) -> Self {
        // The max width and max height are were given by the terminal ui.
        // If there is more than one maze, reduce the max width.
        let max_width = match collection_position.1 {
            0 => panic!(),
            1 => max_width,
            _ => max_width / collection_position.1 - (collection_position.1 - 1),
        };
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
        if collection_position.0 < 1 || collection_position.0 > collection_position.1 {
            panic!();
        }
        Maze {
            width,
            height,
            pos_start: (1, 1),
            pos_end: (width - 2, height - 2),
            max_width,
            max_height,
            data: vec![vec![MAZE_VALUE_BLOCKED; height]; height],
            is_node: vec![vec![false; width]; height],
            collection_position,
            is_generated: false,
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
        self.pos_start = (1, 1);
        self.pos_end = (width - 2, height - 2);
        self.data = vec![vec![MAZE_VALUE_BLOCKED; width]; height];
        self.is_node = vec![vec![false; width]; height];
        return true;
    }

    pub fn is_accessible(&self, pos: (usize, usize)) -> bool {
        if pos.0 >= self.width || pos.1 >= self.height {
            panic!();
        }
        self.data[pos.1][pos.0] == MAZE_VALUE_ACCESSIBLE
    }

    pub fn is_blocked(&self, pos: (usize, usize)) -> bool {
        if pos.0 >= self.width || pos.1 >= self.height {
            panic!();
        }
        self.data[pos.1][pos.0] == MAZE_VALUE_BLOCKED
    }

    pub fn reset(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                self.data[row][col] = MAZE_VALUE_BLOCKED;
                self.is_node[row][col] = false;
            }
        }
        self.is_generated = false;
    }

    pub fn reset_start_end_position(&mut self) {
        self.pos_start = (1, 1);
        self.pos_end = (self.width - 2, self.height - 2);
    }

    pub fn set_start_end_position(
        &mut self,
        pos_start: Option<(usize, usize)>,
        pos_end: Option<(usize, usize)>,
    ) {
        if let Some(pos) = pos_start {
            self.pos_start = pos;
        }
        if let Some(pos) = pos_end {
            self.pos_end = pos;
        }
    }

    fn get_random_accessible_position(&self) -> (usize, usize) {
        self.data
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|&(_, &value)| value == MAZE_VALUE_ACCESSIBLE)
                    .map(move |(x, _)| (x, y))
            })
            .collect::<Vec<(usize, usize)>>()
            .choose(&mut rand::thread_rng())
            .cloned()
            .unwrap()
    }

    pub fn set_random_start_end_position(&mut self) {
        // The two positions need to be accessible and also not the same.
        self.pos_start = self.get_random_accessible_position();
        self.pos_end = self.get_random_accessible_position();
        while self.pos_end == self.pos_start {
            self.pos_end = self.get_random_accessible_position();
        }
    }

    pub fn generate(
        &mut self,
        generator: &dyn MazeGenerator,
        screen: &mut dyn Write,
        animate: bool,
    ) {
        // Draw the maze as empty as it is and draw it empty.
        self.reset();
        self.reset_start_end_position();
        self.draw(screen, false, false, false, false, false);
        // Generate the maze.
        generator.generate(self, screen, animate);
        // Generate the graph once.
        self.generate_graph();
        self.is_generated = true;
    }

    pub fn solve(
        &mut self,
        solver: &dyn MazeSolver,
        screen: &mut dyn Write,
        animate: bool,
    ) -> (Vec<(usize, usize)>, usize) {
        if self.is_generated {
            // Draw the maze again (this may delete the path from the previous solving).
            self.draw(screen, false, false, false, false, false);
            solver.solve(self, screen, animate)
        } else {
            (Vec::new(), 0)
        }
    }

    pub fn draw(
        &self,
        screen: &mut dyn Write,
        show_graph: bool,
        show_background_graph: bool,
        show_binary_representation: bool,
        show_background_binary_representation: bool,
        show_grid_representation: bool,
    ) {
        if (show_graph || show_background_graph)
            && ((show_binary_representation || show_background_binary_representation)
                || show_grid_representation)
        {
            panic!();
        }
        if show_graph {
            draw_graph_representation(screen, self, show_background_graph);
        } else if show_binary_representation {
            draw_binary_representation(screen, self, show_background_binary_representation);
        } else if show_grid_representation {
            draw_grid_representation(screen, self);
        } else {
            draw_maze(screen, self);
        }
    }

    pub fn erase(&self, screen: &mut dyn Write) {
        erase_maze(screen, self);
    }

    pub fn generate_graph(&mut self) {
        for (row, data_row) in self.data.iter().enumerate() {
            for (col, &datum) in data_row.iter().enumerate() {
                if datum == MAZE_VALUE_BLOCKED {
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
                        (
                            MAZE_VALUE_BLOCKED,
                            MAZE_VALUE_ACCESSIBLE,
                            MAZE_VALUE_BLOCKED,
                            MAZE_VALUE_ACCESSIBLE,
                        ) => true,
                        (
                            MAZE_VALUE_ACCESSIBLE,
                            MAZE_VALUE_BLOCKED,
                            MAZE_VALUE_BLOCKED,
                            MAZE_VALUE_ACCESSIBLE,
                        ) => true,
                        (
                            MAZE_VALUE_BLOCKED,
                            MAZE_VALUE_ACCESSIBLE,
                            MAZE_VALUE_ACCESSIBLE,
                            MAZE_VALUE_BLOCKED,
                        ) => true,
                        (
                            MAZE_VALUE_ACCESSIBLE,
                            MAZE_VALUE_BLOCKED,
                            MAZE_VALUE_ACCESSIBLE,
                            MAZE_VALUE_BLOCKED,
                        ) => true,
                        // dead end.
                        (
                            MAZE_VALUE_BLOCKED,
                            MAZE_VALUE_BLOCKED,
                            MAZE_VALUE_BLOCKED,
                            MAZE_VALUE_ACCESSIBLE,
                        ) => true,
                        (
                            MAZE_VALUE_BLOCKED,
                            MAZE_VALUE_BLOCKED,
                            MAZE_VALUE_ACCESSIBLE,
                            MAZE_VALUE_BLOCKED,
                        ) => true,
                        (
                            MAZE_VALUE_BLOCKED,
                            MAZE_VALUE_ACCESSIBLE,
                            MAZE_VALUE_BLOCKED,
                            MAZE_VALUE_BLOCKED,
                        ) => true,
                        (
                            MAZE_VALUE_ACCESSIBLE,
                            MAZE_VALUE_BLOCKED,
                            MAZE_VALUE_BLOCKED,
                            MAZE_VALUE_BLOCKED,
                        ) => true,
                        // crossway.
                        (
                            MAZE_VALUE_ACCESSIBLE,
                            MAZE_VALUE_ACCESSIBLE,
                            MAZE_VALUE_ACCESSIBLE,
                            MAZE_VALUE_BLOCKED,
                        ) => true,
                        (
                            MAZE_VALUE_ACCESSIBLE,
                            MAZE_VALUE_ACCESSIBLE,
                            MAZE_VALUE_BLOCKED,
                            MAZE_VALUE_ACCESSIBLE,
                        ) => true,
                        (
                            MAZE_VALUE_ACCESSIBLE,
                            MAZE_VALUE_BLOCKED,
                            MAZE_VALUE_ACCESSIBLE,
                            MAZE_VALUE_ACCESSIBLE,
                        ) => true,
                        (
                            MAZE_VALUE_BLOCKED,
                            MAZE_VALUE_ACCESSIBLE,
                            MAZE_VALUE_ACCESSIBLE,
                            MAZE_VALUE_ACCESSIBLE,
                        ) => true,
                        (
                            MAZE_VALUE_ACCESSIBLE,
                            MAZE_VALUE_ACCESSIBLE,
                            MAZE_VALUE_ACCESSIBLE,
                            MAZE_VALUE_ACCESSIBLE,
                        ) => true,
                        _ => false,
                    };
                }
            }
        }
    }
}
