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

    // Initialize our maze struct.
    let mut maze = Maze::new(61, 31);

    // Draw terminal ui and the maze.
    terminal_ui::intialize_terminal_ui(&mut screen);
    maze.generate(&Kruskal);
    maze.draw(&mut screen);

    // Save the current terminal size so that we can react if the size changes.
    let mut terminal_size = termion::terminal_size().unwrap();

    // The main loop that keeps the program alive. q breaks it.
    for c in stdin.keys() {
        // Everytime an input event happens, we need also to check if the
        // terminal got resized. An event for this would be nicer.
        // THIS DOES CURRENTLY NOT WORK!
        if terminal_size != termion::terminal_size().unwrap() {
            terminal_size = termion::terminal_size().unwrap();
            terminal_ui::intialize_terminal_ui(&mut screen);
        }

        // Process the input.
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('r') => {
                maze.generate(&Kruskal);
                maze.draw(&mut screen);
            }
            _ => {}
        }
        screen.flush().unwrap();
    }

    write!(screen, "{}", termion::cursor::Show).unwrap();
}
