use crate::maze::maze::*;
use crate::maze::path::*;
use std::io::Write;

pub const SYMBOL_MAZE_FIELD_ACCESSIBLE: char = ' ';
pub const SYMBOL_MAZE_FIELD_BLOCKED: char = '█';
const SYMBOL_MAZE_ERASED: char = ' ';
pub const SYMBOL_MAZE_POS_START: char = 'S';
pub const SYMBOL_MAZE_POS_END: char = 'E';
pub const SYMBOL_MAZE_POS_CURRENT: char = '◈';

const SYMBOL_MAZE_GRAPH_NODE: char = '⊚';
const SYMBOL_MAZE_GRAPH_CONNECTION_HORIZONTAL: char = '─';
const SYMBOL_MAZE_GRAPH_CONNECTION_VERTICAL: char = '│';

const SYMBOL_MAZE_PATH_HORIZONTAL: char = '─';
const SYMBOL_MAZE_PATH_VERTICAL: char = '│';
const SYMBOL_MAZE_PATH_CURVE_LEFT_UP: char = '╯';
const SYMBOL_MAZE_PATH_CURVE_LEFT_DOWN: char = '╮';
const SYMBOL_MAZE_PATH_CURVE_RIGHT_UP: char = '╰';
const SYMBOL_MAZE_PATH_CURVE_RIGHT_DOWN: char = '╭';
const SYMBOL_MAZE_PATH_DEAD_END_TOP: char = '╿';
const SYMBOL_MAZE_PATH_DEAD_END_BOTTOM: char = '╽';
const SYMBOL_MAZE_PATH_DEAD_END_LEFT: char = '╾';
const SYMBOL_MAZE_PATH_DEAD_END_RIGHT: char = '╼';
const SYMBOL_MAZE_PATH_SINGLE_POSITION: char = '╳';

const CHAR_MAZE_ACCESSIBLE: char = '0';
const CHAR_MAZE_BLOCKED: char = '1';

const MULTIPLE_MAZES_DRAW_DISTANCE: usize = 2;

#[derive(Clone, Copy)]
pub enum CellColorType {
    InspectedCell,
    CurrentCell,
    Path,
}

impl CellColorType {
    // https://coolors.co/palettes/trending
    pub fn to_termion_color(&self) -> String {
        match self {
            Self::InspectedCell => {
                let (r, g, b) = Self::termion_rgb_from_string(String::from("ffd166"));
                format!("{}", termion::color::Bg(termion::color::Rgb(r, g, b)))
            }
            Self::CurrentCell => {
                let (r, g, b) = Self::termion_rgb_from_string(String::from("ef476f"));
                format!("{}", termion::color::Bg(termion::color::Rgb(r, g, b)))
            }
            Self::Path => {
                let (r, g, b) = Self::termion_rgb_from_string(String::from("06d6a0"));
                format!("{}", termion::color::Bg(termion::color::Rgb(r, g, b)))
            }
        }
    }

    pub fn termion_rgb_from_string(color_string: String) -> (u8, u8, u8) {
        if color_string.len() != 6 {
            panic!()
        }
        let r = u8::from_str_radix(&color_string[0..=1], 16).unwrap();
        let g = u8::from_str_radix(&color_string[2..=3], 16).unwrap();
        let b = u8::from_str_radix(&color_string[4..=5], 16).unwrap();
        (r, g, b)
    }
}

fn calculate_maze_position(maze: &Maze) -> (u16, u16) {
    let (terminal_width, terminal_height) = termion::terminal_size().unwrap();
    let y = (terminal_height - maze.height as u16) / 2 + 1;
    let x = if maze.collection_position.1 % 2 == 1 {
        // Odd number of mazes in this collection.
        let idx_middle_maze = (maze.collection_position.1 + 1) / 2;
        let pos_center = (terminal_width - maze.width as u16) / 2 + 1;
        let idx_difference: isize =
            -(idx_middle_maze as isize - maze.collection_position.0 as isize);
        (pos_center as isize
            + (idx_difference * (maze.width as isize + MULTIPLE_MAZES_DRAW_DISTANCE as isize)))
            as u16
    } else {
        // Even number of mazes in this collection
        let idx_middle_maze = (maze.collection_position.1 / 2) + 1;
        let pos_center = terminal_width / 2 + 1;
        let idx_difference: isize =
            -(idx_middle_maze as isize - maze.collection_position.0 as isize);
        (pos_center as isize
            + (idx_difference * (maze.width as isize + MULTIPLE_MAZES_DRAW_DISTANCE as isize)))
            as u16
    };
    return (x, y);
}

pub fn erase_maze(screen: &mut dyn Write, maze: &Maze) {
    let (maze_pos_x, maze_pos_y) = calculate_maze_position(maze);
    for row in 0..maze.height {
        write!(
            screen,
            "{}{}",
            termion::cursor::Goto(maze_pos_x, maze_pos_y + row as u16),
            std::iter::repeat(SYMBOL_MAZE_ERASED)
                .take(maze.width)
                .collect::<String>()
        )
        .unwrap();
    }
}

pub fn draw_maze(screen: &mut dyn Write, maze: &Maze) {
    erase_maze(screen, maze);
    let (maze_pos_x, maze_pos_y) = calculate_maze_position(maze);
    for row in 0..maze.height {
        write!(
            screen,
            "{}{}",
            termion::cursor::Goto(maze_pos_x, maze_pos_y + row as u16),
            maze.data[row]
                .iter()
                .enumerate()
                .map(|(col, &is_accessible)| {
                    match is_accessible {
                        MAZE_VALUE_BLOCKED => SYMBOL_MAZE_FIELD_BLOCKED,
                        MAZE_VALUE_ACCESSIBLE => {
                            if (col, row) == maze.pos_start {
                                SYMBOL_MAZE_POS_START
                            } else if (col, row) == maze.pos_end {
                                SYMBOL_MAZE_POS_END
                            } else {
                                SYMBOL_MAZE_FIELD_ACCESSIBLE
                            }
                        }
                    }
                })
                .collect::<String>()
        )
        .unwrap();
    }
    screen.flush().unwrap();
}

pub fn draw_graph_representation(screen: &mut dyn Write, maze: &Maze, show_background: bool) {
    erase_maze(screen, maze);
    let (maze_pos_x, maze_pos_y) = calculate_maze_position(maze);
    for row in 0..maze.height {
        write!(
            screen,
            "{}{}",
            termion::cursor::Goto(maze_pos_x, maze_pos_y + row as u16),
            maze.data[row]
                .iter()
                .zip(maze.is_node[row].iter())
                .enumerate()
                .map(|(col, (&is_accessible, &is_node))| {
                    match (is_accessible, is_node) {
                        (MAZE_VALUE_BLOCKED, _) => {
                            if show_background {
                                SYMBOL_MAZE_FIELD_BLOCKED
                            } else {
                                SYMBOL_MAZE_FIELD_ACCESSIBLE
                            }
                        }
                        (MAZE_VALUE_ACCESSIBLE, true) => SYMBOL_MAZE_GRAPH_NODE,
                        (MAZE_VALUE_ACCESSIBLE, false) => match maze.data[row][col - 1] {
                            MAZE_VALUE_ACCESSIBLE => SYMBOL_MAZE_GRAPH_CONNECTION_HORIZONTAL,
                            MAZE_VALUE_BLOCKED => SYMBOL_MAZE_GRAPH_CONNECTION_VERTICAL,
                        },
                    }
                })
                .collect::<String>()
        )
        .unwrap();
    }
    screen.flush().unwrap();
}

pub fn draw_binary_representation(screen: &mut dyn Write, maze: &Maze, show_background: bool) {
    erase_maze(screen, maze);
    let (maze_pos_x, maze_pos_y) = calculate_maze_position(maze);
    for row in 0..maze.height {
        write!(
            screen,
            "{}{}",
            termion::cursor::Goto(maze_pos_x, maze_pos_y + row as u16),
            maze.data[row]
                .iter()
                .map(|is_accessible| match *is_accessible {
                    MAZE_VALUE_ACCESSIBLE => format!(
                        "{}{}{}",
                        termion::color::Fg(termion::color::Black),
                        match show_background {
                            true => format!("{}", termion::color::Bg(termion::color::White)),
                            false => format!("{}", termion::color::Bg(termion::color::Reset)),
                        },
                        CHAR_MAZE_ACCESSIBLE
                    ),
                    MAZE_VALUE_BLOCKED => format!(
                        "{}{}{}",
                        match show_background {
                            true => format!("{}", termion::color::Fg(termion::color::White)),
                            false => format!("{}", termion::color::Fg(termion::color::Black)),
                        },
                        match show_background {
                            true => format!("{}", termion::color::Bg(termion::color::Black)),
                            false => format!("{}", termion::color::Bg(termion::color::Reset)),
                        },
                        CHAR_MAZE_BLOCKED
                    ),
                })
                .collect::<String>()
        )
        .unwrap();
    }
    write!(
        screen,
        "{}{}",
        termion::color::Fg(termion::color::Reset),
        termion::color::Bg(termion::color::Reset)
    )
    .unwrap();
    screen.flush().unwrap();
}

pub fn draw_grid_representation(screen: &mut dyn Write, maze: &Maze) {
    // We only need the maze to calculate the position to draw to and to get the size.
    erase_maze(screen, maze);
    let (maze_pos_x, maze_pos_y) = calculate_maze_position(maze);
    let symbol_accessible = format!("{} ", termion::color::Bg(termion::color::Reset));
    let symbol_not_accessible = format!("{} ", termion::color::Bg(termion::color::LightBlack));
    let symbol_passage_horizontal = format!(
        "{}{}{}",
        termion::color::Bg(termion::color::Black),
        termion::color::Fg(termion::color::White),
        SYMBOL_MAZE_GRAPH_CONNECTION_HORIZONTAL
    );
    let symbol_passage_vertical = format!(
        "{}{}{}",
        termion::color::Bg(termion::color::Black),
        termion::color::Fg(termion::color::White),
        SYMBOL_MAZE_GRAPH_CONNECTION_VERTICAL
    );
    for row in 0..maze.height {
        write!(
            screen,
            "{}{}",
            termion::cursor::Goto(maze_pos_x, maze_pos_y + row as u16),
            if row == 0 || row == maze.height - 1 {
                symbol_not_accessible.repeat(maze.width)
            } else if row % 2 == 1 {
                format!(
                    "{}{}{}{}",
                    symbol_not_accessible,
                    symbol_accessible,
                    format!("{}{}", symbol_passage_horizontal, symbol_accessible)
                        .repeat((maze.width - 3) / 2),
                    symbol_not_accessible
                )
            } else {
                format!(
                    "{}{}",
                    symbol_not_accessible,
                    format!("{}{}", symbol_passage_vertical, symbol_not_accessible)
                        .repeat((maze.width - 1) / 2)
                )
            }
        )
        .unwrap();
    }
    write!(
        screen,
        "{}{}",
        termion::color::Fg(termion::color::Reset),
        termion::color::Bg(termion::color::Reset)
    )
    .unwrap();
    screen.flush().unwrap();
}

pub fn highlight_cell(
    screen: &mut dyn Write,
    maze: &Maze,
    pos: (usize, usize),
    color_type: CellColorType,
) {
    let (maze_pos_x, maze_pos_y) = calculate_maze_position(maze);
    write!(
        screen,
        "{}{}{}{}",
        termion::cursor::Goto(maze_pos_x + pos.0 as u16, maze_pos_y + pos.1 as u16),
        color_type.to_termion_color(),
        if pos == maze.pos_start {
            SYMBOL_MAZE_POS_START
        } else if pos == maze.pos_end {
            SYMBOL_MAZE_POS_END
        } else {
            SYMBOL_MAZE_FIELD_ACCESSIBLE
        },
        termion::color::Bg(termion::color::Reset),
    )
    .unwrap();
    screen.flush().unwrap();
}

pub fn highlight_cells_by_rgb_color(
    screen: &mut dyn Write,
    maze: &Maze,
    pos: Vec<(usize, usize)>,
    color: (u8, u8, u8),
) {
    let (maze_pos_x, maze_pos_y) = calculate_maze_position(maze);
    for (y, x) in pos.iter() {
        write!(
            screen,
            "{}{}{}{}",
            termion::cursor::Goto(maze_pos_x + *x as u16, maze_pos_y + *y as u16),
            termion::color::Bg(termion::color::Rgb(color.0, color.1, color.2)),
            if (*x, *y) == maze.pos_start {
                SYMBOL_MAZE_POS_START
            } else if (*x, *y) == maze.pos_end {
                SYMBOL_MAZE_POS_END
            } else {
                SYMBOL_MAZE_FIELD_ACCESSIBLE
            },
            termion::color::Bg(termion::color::Reset),
        )
        .unwrap();
    }
    screen.flush().unwrap();
}

pub fn draw_character(
    screen: &mut dyn Write,
    maze: &Maze,
    pos: (usize, usize),
    character: char,
    highlight: Option<CellColorType>,
) {
    let (maze_pos_x, maze_pos_y) = calculate_maze_position(maze);
    if let Some(highlight_color) = highlight {
        write!(screen, "{}", highlight_color.to_termion_color()).unwrap();
    }
    write!(
        screen,
        "{}{}",
        termion::cursor::Goto(maze_pos_x + pos.0 as u16, maze_pos_y + pos.1 as u16),
        character
    )
    .unwrap();
    if let Some(_) = highlight {
        write!(screen, "{}", termion::color::Bg(termion::color::Reset)).unwrap();
    }
    screen.flush().unwrap();
}

pub fn draw_path(
    screen: &mut dyn Write,
    maze: &Maze,
    path: Vec<(usize, usize)>,
    highlight: Option<CellColorType>,
) {
    let path = complete_path(path);
    if let Some(highlight_color) = highlight {
        write!(
            screen,
            "{}{}",
            termion::color::Fg(termion::color::Black),
            highlight_color.to_termion_color()
        )
        .unwrap();
    }
    path.iter().enumerate().for_each(|(idx, pos)| {
        let pos_prev = if idx > 0 { path[idx - 1] } else { *pos };
        let pos_next = if idx < path.len() - 1 {
            path[idx + 1]
        } else {
            *pos
        };
        draw_character(
            screen,
            maze,
            *pos,
            if *pos == maze.pos_start {
                SYMBOL_MAZE_POS_START
            } else if *pos == maze.pos_end {
                SYMBOL_MAZE_POS_END
            } else {
                match (
                    pos_prev.0 as isize - pos.0 as isize,
                    pos_prev.1 as isize - pos.1 as isize,
                    pos_next.0 as isize - pos.0 as isize,
                    pos_next.1 as isize - pos.1 as isize,
                ) {
                    // single position.
                    (0, 0, 0, 0) => SYMBOL_MAZE_PATH_SINGLE_POSITION,
                    // dead end to the right.
                    (-1, 0, 0, 0) => SYMBOL_MAZE_PATH_DEAD_END_RIGHT,
                    (0, 0, -1, 0) => SYMBOL_MAZE_PATH_DEAD_END_RIGHT,
                    // dead end to the left.
                    (1, 0, 0, 0) => SYMBOL_MAZE_PATH_DEAD_END_LEFT,
                    (0, 0, 1, 0) => SYMBOL_MAZE_PATH_DEAD_END_LEFT,
                    // dead end to the top.
                    (0, 1, 0, 0) => SYMBOL_MAZE_PATH_DEAD_END_TOP,
                    (0, 0, 0, 1) => SYMBOL_MAZE_PATH_DEAD_END_TOP,
                    // dead end to the bottom.
                    (0, -1, 0, 0) => SYMBOL_MAZE_PATH_DEAD_END_BOTTOM,
                    (0, 0, 0, -1) => SYMBOL_MAZE_PATH_DEAD_END_BOTTOM,
                    // horizontal.
                    (-1, 0, 1, 0) => SYMBOL_MAZE_PATH_HORIZONTAL,
                    (1, 0, -1, 0) => SYMBOL_MAZE_PATH_HORIZONTAL,
                    // vertical.
                    (0, -1, 0, 1) => SYMBOL_MAZE_PATH_VERTICAL,
                    (0, 1, 0, -1) => SYMBOL_MAZE_PATH_VERTICAL,
                    // curve left and up.
                    (-1, 0, 0, -1) => SYMBOL_MAZE_PATH_CURVE_LEFT_UP,
                    (0, -1, -1, 0) => SYMBOL_MAZE_PATH_CURVE_LEFT_UP,
                    // curve left and down.
                    (-1, 0, 0, 1) => SYMBOL_MAZE_PATH_CURVE_LEFT_DOWN,
                    (0, 1, -1, 0) => SYMBOL_MAZE_PATH_CURVE_LEFT_DOWN,
                    // curve right and up.
                    (1, 0, 0, -1) => SYMBOL_MAZE_PATH_CURVE_RIGHT_UP,
                    (0, -1, 1, 0) => SYMBOL_MAZE_PATH_CURVE_RIGHT_UP,
                    // curve right and down.
                    (1, 0, 0, 1) => SYMBOL_MAZE_PATH_CURVE_RIGHT_DOWN,
                    (0, 1, 1, 0) => SYMBOL_MAZE_PATH_CURVE_RIGHT_DOWN,
                    // hopefully I forgot nothing.
                    _ => '?',
                }
            },
            None,
        );
    });
    if let Some(_) = highlight {
        write!(
            screen,
            "{}{}",
            termion::color::Fg(termion::color::Reset),
            termion::color::Bg(termion::color::Reset)
        )
        .unwrap();
    }
    screen.flush().unwrap();
}

pub fn get_unique_colors(n: usize) -> Vec<(u8, u8, u8)> {
    // Returns n unique colors or repeats the unique colors if n is too large.
    // There is an algorithm to implement this:
    // https://stackoverflow.com/questions/309149/generate-distinctly-different-rgb-colors-in-graphs
    // Currently we will just use a list.
    let colors: Vec<(u8, u8, u8)> = vec![
        (255, 0, 0),
        (0, 255, 0),
        (0, 0, 255),
        (255, 255, 0),
        (0, 255, 255),
        (255, 0, 255),
        (128, 128, 128),
        (255, 128, 128),
        (128, 255, 128),
        (128, 128, 255),
        (0, 128, 128),
        (128, 0, 128),
        (128, 128, 0),
        (255, 255, 128),
        (128, 255, 255),
        (255, 128, 255),
        (255, 0, 128),
        (128, 255, 0),
        (0, 128, 255),
        (0, 255, 128),
        (128, 0, 255),
        (255, 128, 0),
    ];
    colors.iter().cycle().take(n).cloned().collect()
}
