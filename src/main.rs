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

    // To toggle visualization options.
    let mut show_graph: bool = false;
    let mut animate: bool = true;

    // Draw terminal ui and the maze.
    terminal_ui::intialize_terminal_ui(&mut screen);
    let (max_maze_width, max_maze_height) = terminal_ui::get_max_draw_size();
    let mut maze = Maze::new(max_maze_width, max_maze_height);
    maze.generate(&Kruskal, &mut screen, animate);
    maze.draw(&mut screen, show_graph);

    // The main loop that keeps the program alive. q breaks it.
    for c in stdin.keys() {
        // Process the input.
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('r') => {
                // Recreate.
                maze.generate(&Kruskal, &mut screen, animate);
                maze.draw(&mut screen, show_graph);
            }
            Key::Up | Key::Char('k') => {
                // Increase size.
                if maze.change_size(maze.width + 2, maze.height + 2) {
                    maze.generate(&Kruskal, &mut screen, animate);
                    maze.draw(&mut screen, show_graph);
                }
            }
            Key::Down | Key::Char('j') => {
                // Decrease size.
                if maze.change_size(maze.width - 2, maze.height - 2) {
                    terminal_ui::erase_draw_area(&mut screen);
                    maze.generate(&Kruskal, &mut screen, animate);
                    maze.draw(&mut screen, show_graph);
                }
            }
            Key::Char('g') => {
                // Show / hide graph nodes.
                show_graph = !show_graph;
                maze.draw(&mut screen, show_graph);
            }
            Key::Char('a') => {
                // Toggle animation on / off.
                animate = !animate;
            }
            _ => {}
        }
        screen.flush().unwrap();
    }

    write!(screen, "{}", termion::cursor::Show).unwrap();
}
