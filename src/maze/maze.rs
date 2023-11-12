use crate::maze::generator::MazeGenerator;

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

pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Vec<bool>>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        Maze {
            width,
            height,
            data: vec![vec![false; width]; height],
        }
    }

    pub fn draw(&self) {
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

    pub fn generate(&mut self, generator: &dyn MazeGenerator) {
        generator.generate(self);
    }
}
