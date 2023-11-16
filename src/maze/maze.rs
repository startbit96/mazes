use crate::maze::draw::*;
use crate::maze::generator::MazeGenerator;
use std::io::Write;

const MAZE_EDGE_LENGTH_MIN: usize = 11;
const MAZE_EDGE_LENGTH_MAX: usize = 201;

pub enum CellType {
    Path,
    Wall,
}

pub struct MazeCell {
    pub cell_type: CellType,
    pub was_visited: bool,
    pub is_on_current_path: bool,
}

pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<bool>>,
}

// What informations holds single cell?
// - wall / path
// - visited
// - on current path

// solution of the maze:
// Vec<(usize, usize)> route
// update route: check for first occurence of last value,

impl Maze {
    /// Creates a new maze instance with the given dimensions.
    /// The given dimensions will be checked and corrected if necessary by
    /// the function `Maze::check_edge_length`.
    /// Note that the maze will not be generated using this function, it will
    /// only provide an empty maze with only walls and no paths. To generate a
    /// maze, see `Maze::generate`.
    ///
    /// # Arguments
    ///
    /// * `width` - the width of the maze (needs to be odd and within `MAZE_EDGE_LENGTH_MIN` and `MAZE_EDGE_LENGTH_MAX`)
    /// * `height` - the height of the maze (needs to be odd and within `MAZE_EDGE_LENGTH_MIN` and `MAZE_EDGE_LENGTH_MAX`)
    ///
    /// # Returns
    ///
    /// A Maze instance.
    pub fn new(width: usize, height: usize) -> Self {
        let width = Maze::check_edge_length(width);
        let height = Maze::check_edge_length(height);
        Maze {
            width,
            height,
            data: vec![vec![false; width]; height],
        }
    }

    /// Checks the given edge length and corrects the input to be odd and within
    /// the given range if necessary.
    ///
    /// # Arguments
    ///
    /// * `edge_length` - the given edge length that needs to be checked
    ///
    /// # Returns
    ///
    /// The corrected edge length or - if it already fulfills the requirements - the
    /// given edge length.
    fn check_edge_length(edge_length: usize) -> usize {
        if edge_length < MAZE_EDGE_LENGTH_MIN {
            MAZE_EDGE_LENGTH_MIN
        } else if edge_length > MAZE_EDGE_LENGTH_MAX {
            MAZE_EDGE_LENGTH_MAX
        } else if edge_length % 2 == 0 {
            edge_length + 1
        } else {
            edge_length
        }
    }

    fn reset(&mut self) {
        for row in &mut self.data {
            for value in row {
                *value = false;
            }
        }
    }

    pub fn generate(&mut self, generator: &dyn MazeGenerator) {
        self.reset();
        generator.generate(self);
    }

    pub fn draw<W: Write>(&self, screen: &mut W) {
        draw_maze(screen, self);
    }
}
