use crate::maze::maze::Maze;
use std::io::Write;

const SYMBOL_MAZE_FIELD_ACCESSIBLE: char = ' ';
const SYMBOL_MAZE_FIELD_BLOCKED: char = '█';
const SYMBOL_MAZE_ERASED: char = ' ';

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

#[derive(Debug, PartialEq, Eq)]
enum PathOrientation {
    Horizontal,
    Vertical,
}

impl PathOrientation {
    fn from_points(pos1: (u16, u16), pos2: (u16, u16)) -> Self {
        match (pos1.0 == pos2.0, pos1.1 == pos2.1) {
            (true, false) => PathOrientation::Vertical,
            (false, true) => PathOrientation::Horizontal,
            _ => panic!("Diagonally is currently not supported."),
        }
    }
}

fn calculate_maze_position(maze: &Maze) -> (u16, u16) {
    let (terminal_width, terminal_height) = termion::terminal_size().unwrap();
    return (
        (terminal_width - maze.width as u16) / 2 + 1,
        (terminal_height - maze.height as u16) / 2 + 1,
    );
}

pub fn erase_maze<W: Write>(screen: &mut W, maze: &Maze) {
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

pub fn draw_maze<W: Write>(screen: &mut W, maze: &Maze) {
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
                .map(|(col, datum)| match (*datum, maze.is_node[row][col]) {
                    (true, false) => SYMBOL_MAZE_FIELD_ACCESSIBLE.to_string(),
                    (true, true) => format!(
                        "{}{}{}",
                        termion::color::Bg(termion::color::Green),
                        SYMBOL_MAZE_FIELD_ACCESSIBLE,
                        termion::color::Bg(termion::color::Reset)
                    ),
                    _ => SYMBOL_MAZE_FIELD_BLOCKED.to_string(),
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

fn complete_line(pos_from: (u16, u16), pos_to: (u16, u16)) -> Vec<(u16, u16)> {
    let orientation = PathOrientation::from_points(pos_from, pos_to);
    match orientation {
        PathOrientation::Vertical => {
            let mut line = (pos_from.1.min(pos_to.1)..=pos_from.1.max(pos_to.1))
                .map(|y| (pos_from.0, y))
                .collect::<Vec<(u16, u16)>>();
            if pos_from.1 > pos_to.1 {
                line.reverse();
            }
            return line;
        }
        PathOrientation::Horizontal => {
            let mut line = (pos_from.0.min(pos_to.0)..=pos_from.0.max(pos_to.0))
                .map(|x| (x, pos_from.1))
                .collect::<Vec<(u16, u16)>>();
            if pos_from.0 > pos_to.0 {
                line.reverse();
            }
            return line;
        }
    }
}

fn complete_path(path: Vec<(u16, u16)>) -> Vec<(u16, u16)> {
    // This function only implements straight lines, if more is needed, Bresenham will be implemented.
    path.windows(2)
        .enumerate()
        .flat_map(|(idx, window)| {
            let mut line = complete_line(window[0], window[1]);
            if idx > 0 {
                // Otherwise we would have the junctions twice.
                line.remove(0);
            }
            line
        })
        .collect()
}

pub fn draw_path<W: Write>(screen: &mut W, maze: &Maze, path: Vec<(u16, u16)>) {
    let path = complete_path(path);
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
