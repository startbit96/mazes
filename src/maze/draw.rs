use crate::maze::maze::Maze;
use std::io::Write;

const SYMBOL_MAZE_FIELD_ACCESSIBLE: char = ' ';
const SYMBOL_MAZE_FIELD_BLOCKED: char = '█';

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
