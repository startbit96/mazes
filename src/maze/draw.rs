use crate::maze::maze::Maze;
use std::io::Write;

const SYMBOL_MAZE_FIELD_ACCESSIBLE: char = ' ';
const SYMBOL_MAZE_FIELD_BLOCKED: char = '█';

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
        (terminal_width - maze.width as u16) / 2,
        (terminal_height - maze.height as u16) / 2,
    );
}

pub fn draw_maze<W: Write>(screen: &mut W, maze: &Maze) {
    let (maze_pos_x, maze_pos_y) = calculate_maze_position(maze);
    for row in 0..maze.height {
        write!(
            screen,
            "{}{}",
            termion::cursor::Goto(maze_pos_x, maze_pos_y + row as u16),
            maze.data[row]
                .iter()
                .map(|datum| if datum == &true {
                    SYMBOL_MAZE_FIELD_ACCESSIBLE
                } else {
                    SYMBOL_MAZE_FIELD_BLOCKED
                })
                .collect::<String>()
        )
        .unwrap();
    }
    screen.flush().unwrap();
}

pub fn draw_character<W: Write>(screen: &mut W, maze: &Maze, pos: (u16, u16), character: char) {
    let (maze_pos_x, maze_pos_y) = calculate_maze_position(maze);
    write!(
        screen,
        "{}{}",
        termion::cursor::Goto(maze_pos_x + pos.0, maze_pos_y + pos.1),
        character
    )
    .unwrap();
}

pub fn draw_path<W: Write>(screen: &mut W, maze: &Maze, path: Vec<(u16, u16)>) {
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
                pos_prev.0 as i16 - pos.0 as i16,
                pos_prev.1 as i16 - pos.1 as i16,
                pos_next.0 as i16 - pos.0 as i16,
                pos_next.1 as i16 - pos.1 as i16,
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
        );
    });
    screen.flush().unwrap();
}
