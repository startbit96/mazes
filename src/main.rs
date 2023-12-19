use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::{IntoAlternateScreen, ToAlternateScreen};

mod maze;
mod terminal_ui;

use maze::generator::kruskal::Kruskal;
use maze::maze::Maze;

fn main() {
    // Initialize the alternate screen.
    let stdin = stdin();
    let mut screen = stdout()
        .into_raw_mode()
        .unwrap()
        .into_alternate_screen()
        .unwrap();
    write!(screen, "{}{}", termion::cursor::Hide, ToAlternateScreen).unwrap();
    screen.flush().unwrap();

    // Draw terminal ui and the maze.
    terminal_ui::intialize_terminal_ui(&mut screen);
    let (max_maze_width, max_maze_height) = terminal_ui::get_max_draw_size();
    let mut maze = Maze::new(max_maze_width, max_maze_height);
    maze.generate(&Kruskal);
    maze.draw(&mut screen);

    // The main loop that keeps the program alive. q breaks it.
    for c in stdin.keys() {
        // Process the input.
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('r') => {
                maze.generate(&Kruskal);
                maze.draw(&mut screen);
            }
            Key::Up | Key::Char('k') => {
                if maze.change_size(maze.width + 2, maze.height + 2) {
                    maze.generate(&Kruskal);
                    maze.draw(&mut screen);
                }
            }
            Key::Down | Key::Char('j') => {
                if maze.change_size(maze.width - 2, maze.height - 2) {
                    terminal_ui::erase_draw_area(&mut screen);
                    maze.generate(&Kruskal);
                    maze.draw(&mut screen);
                }
            }
            Key::Char('g') => {}
            _ => {}
        }
        screen.flush().unwrap();
    }

    write!(screen, "{}", termion::cursor::Show).unwrap();
}
