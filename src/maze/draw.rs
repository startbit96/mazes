use crate::maze::maze::*;
use crate::maze::path::*;
use std::io::Write;

pub const SYMBOL_MAZE_FIELD_ACCESSIBLE: char = ' ';
pub const SYMBOL_MAZE_FIELD_BLOCKED: char = '█';
const SYMBOL_MAZE_ERASED: char = ' ';

const SYMBOL_MAZE_GRAPH_NODE: char = '◉';
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

fn calculate_maze_position(maze: &Maze) -> (u16, u16) {
    let (terminal_width, terminal_height) = termion::terminal_size().unwrap();
    return (
        (terminal_width - maze.width as u16) / 2 + 1,
        (terminal_height - maze.height as u16) / 2 + 1,
    );
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

pub fn draw_maze(screen: &mut dyn Write, maze: &Maze, show_graph: bool) {
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
                    match (is_accessible, is_node, show_graph) {
                        (MAZE_VALUE_BLOCKED, _, _) => SYMBOL_MAZE_FIELD_BLOCKED,
                        (MAZE_VALUE_ACCESSIBLE, _, false) => SYMBOL_MAZE_FIELD_ACCESSIBLE,
                        (MAZE_VALUE_ACCESSIBLE, true, true) => SYMBOL_MAZE_GRAPH_NODE,
                        (MAZE_VALUE_ACCESSIBLE, false, true) => match maze.data[row][col - 1] {
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

pub fn show_binary_representation(screen: &mut dyn Write, maze: &Maze) {
    erase_maze(screen, maze);
    let (maze_pos_x, maze_pos_y) = calculate_maze_position(maze);
    for row in 0..maze.height {
        write!(
            screen,
            "{}{}",
            termion::cursor::Goto(maze_pos_x, maze_pos_y + row as u16),
            maze.data[row]
                .iter()
                .map(|is_accessible| match is_accessible {
                    true => format!(
                        "{}{}1",
                        termion::color::Fg(termion::color::Black),
                        termion::color::Bg(termion::color::White)
                    ),
                    false => format!(
                        "{}{}0",
                        termion::color::Fg(termion::color::White),
                        termion::color::Bg(termion::color::Black)
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

pub fn highlight_cell(screen: &mut dyn Write, maze: &Maze, pos: (usize, usize)) {
    let (maze_pos_x, maze_pos_y) = calculate_maze_position(maze);
    write!(
        screen,
        "{}{}{}{}",
        termion::cursor::Goto(maze_pos_x + pos.0 as u16, maze_pos_y + pos.1 as u16),
        termion::color::Bg(termion::color::LightGreen),
        SYMBOL_MAZE_FIELD_ACCESSIBLE,
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
            SYMBOL_MAZE_FIELD_ACCESSIBLE,
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
    highlight: bool,
) {
    let (maze_pos_x, maze_pos_y) = calculate_maze_position(maze);
    if highlight {
        write!(screen, "{}", termion::color::Bg(termion::color::LightGreen)).unwrap();
    }
    write!(
        screen,
        "{}{}",
        termion::cursor::Goto(maze_pos_x + pos.0 as u16, maze_pos_y + pos.1 as u16),
        character
    )
    .unwrap();
    if highlight {
        write!(screen, "{}", termion::color::Bg(termion::color::Reset)).unwrap();
    }
    screen.flush().unwrap();
}

pub fn draw_path(screen: &mut dyn Write, maze: &Maze, path: Vec<(usize, usize)>) {
    let path = complete_path(path);
    write!(
        screen,
        "{}{}",
        termion::color::Fg(termion::color::Black),
        termion::color::Bg(termion::color::LightBlue)
    )
    .unwrap();
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
            },
            false,
        );
    });
    write!(
        screen,
        "{}{}",
        termion::color::Fg(termion::color::Reset),
        termion::color::Bg(termion::color::Reset)
    )
    .unwrap();

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