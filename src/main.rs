use maze::draw::show_binary_representation;
use maze::generator::GENERATION_DELAY;
use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::{IntoAlternateScreen, ToAlternateScreen};

mod maze;
mod terminal_ui;

use maze::generator::kruskal::Kruskal;
use maze::maze::Maze;
use maze::path::get_solving_sequence;
use maze::solver::{
    breadth_first_search::BreadthFirstSearch, depth_first_search::DepthFirstSearch,
    MazeSolvingAlgorithms,
};

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
    let mut show_representation: bool = false;
    let mut animate: bool = false;

    // Selected algorithms.
    let mut solving_algorithm = MazeSolvingAlgorithms::BreadthFirstSearch;

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
            Key::Char('s') => {
                let path = match solving_algorithm {
                    MazeSolvingAlgorithms::BreadthFirstSearch => {
                        maze.solve(&BreadthFirstSearch, &mut screen, animate)
                    }
                    MazeSolvingAlgorithms::DepthFirstSearch => {
                        maze.solve(&DepthFirstSearch, &mut screen, animate)
                    }
                };
                let solving_sequence = get_solving_sequence(&path);
                let solving_sequence: String = solving_sequence.iter().collect();
                write!(
                    screen,
                    "{}{:?}",
                    termion::cursor::Goto(5, 5),
                    solving_sequence
                )
                .unwrap();
            }
            Key::Char('l') => {
                // Next solving algorithm.
                solving_algorithm = solving_algorithm.next();
            }
            Key::Char('c') => {
                // Redraw the maze (remove the solved path).
                maze.draw(&mut screen, show_graph);
            }
            Key::Char('b') => {
                show_representation = !show_representation;
                if show_representation {
                    show_binary_representation(&mut screen, &maze);
                } else {
                    maze.draw(&mut screen, show_graph);
                }
            }
            _ => {}
        }
        screen.flush().unwrap();
    }

    write!(screen, "{}", termion::cursor::Show).unwrap();
}
