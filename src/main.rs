use maze::draw::{draw_path, show_binary_representation};
use maze::generator::{MazeGenerator, GENERATION_DELAY};
use std::io::{stdin, stdout, Write};
use terminal_ui::{TERMINAL_HEIGHT_MIN, TERMINAL_WIDTH_MIN};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::{IntoAlternateScreen, ToAlternateScreen};

mod maze;
mod terminal_ui;

use maze::generator::{
    kruskal::Kruskal, recursive_backtracking::RecursiveBacktracking, wilson::Wilson,
    MazeGenerationAlgorithms,
};
use maze::maze::Maze;
use maze::path::{apply_solving_sequence, get_solving_sequence};
use maze::solver::{
    breadth_first_search::BreadthFirstSearch, depth_first_search::DepthFirstSearch,
    wall_follower::WallFollower, MazeSolvingAlgorithms,
};

fn main() {
    // Check if the terminal is large enough.
    let (terminal_width, terminal_height) = termion::terminal_size().unwrap();
    if terminal_width < TERMINAL_WIDTH_MIN || terminal_height < TERMINAL_HEIGHT_MIN {
        println!("The terminal is too small. Increase the size of the terminal and restart the application.");
        return;
    }

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
    let mut generation_algorithm = MazeGenerationAlgorithms::Kruskal;
    let mut solving_algorithm = MazeSolvingAlgorithms::BreadthFirstSearch;

    // Draw terminal ui and the maze.
    terminal_ui::intialize_terminal_ui(&mut screen);
    terminal_ui::print_informations(
        &mut screen,
        generation_algorithm.to_string(),
        solving_algorithm.to_string(),
        0,
        animate,
    );

    // Initialize the maze with the information about its max size.
    let (max_maze_width, max_maze_height) = terminal_ui::get_max_draw_size();
    let mut maze = Maze::new(max_maze_width, max_maze_height);

    // Generate the first maze.
    maze.generate(
        match generation_algorithm {
            MazeGenerationAlgorithms::Kruskal => &Kruskal,
            MazeGenerationAlgorithms::RecursiveBacktracking => &RecursiveBacktracking,
            MazeGenerationAlgorithms::Wilson => &Wilson,
        },
        &mut screen,
        animate,
    );
    maze.draw(&mut screen, show_graph);

    // The main loop that keeps the program alive. q breaks it.
    for c in stdin.keys() {
        // Process the input.
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('r') => {
                // Reset the informations in the UI.
                terminal_ui::print_informations(
                    &mut screen,
                    generation_algorithm.to_string(),
                    solving_algorithm.to_string(),
                    0,
                    animate,
                );
                terminal_ui::print_solving_sequence(&mut screen, String::new());
                // Recreate.
                maze.generate(
                    match generation_algorithm {
                        MazeGenerationAlgorithms::Kruskal => &Kruskal,
                        MazeGenerationAlgorithms::RecursiveBacktracking => &RecursiveBacktracking,
                        MazeGenerationAlgorithms::Wilson => &Wilson,
                    },
                    &mut screen,
                    animate,
                );
                maze.draw(&mut screen, show_graph);
            }
            Key::Up | Key::Char('k') => {
                // Increase size.
                if maze.change_size(maze.width + 2, maze.height + 2) {
                    maze.generate(
                        match generation_algorithm {
                            MazeGenerationAlgorithms::Kruskal => &Kruskal,
                            MazeGenerationAlgorithms::RecursiveBacktracking => {
                                &RecursiveBacktracking
                            }
                            MazeGenerationAlgorithms::Wilson => &Wilson,
                        },
                        &mut screen,
                        animate,
                    );
                    maze.draw(&mut screen, show_graph);
                    // Reset the informations in the UI.
                    terminal_ui::print_informations(
                        &mut screen,
                        generation_algorithm.to_string(),
                        solving_algorithm.to_string(),
                        0,
                        animate,
                    );
                    terminal_ui::print_solving_sequence(&mut screen, String::new());
                }
            }
            Key::Down | Key::Char('j') => {
                // Decrease size.
                if maze.change_size(maze.width - 2, maze.height - 2) {
                    terminal_ui::erase_draw_area(&mut screen);
                    maze.generate(
                        match generation_algorithm {
                            MazeGenerationAlgorithms::Kruskal => &Kruskal,
                            MazeGenerationAlgorithms::RecursiveBacktracking => {
                                &RecursiveBacktracking
                            }
                            MazeGenerationAlgorithms::Wilson => &Wilson,
                        },
                        &mut screen,
                        animate,
                    );
                    maze.draw(&mut screen, show_graph);
                    // Reset the informations in the UI.
                    terminal_ui::print_informations(
                        &mut screen,
                        generation_algorithm.to_string(),
                        solving_algorithm.to_string(),
                        0,
                        animate,
                    );
                    terminal_ui::print_solving_sequence(&mut screen, String::new());
                }
            }
            Key::Char('s') => {
                let (path, number_of_inspected_cells) = maze.solve(
                    match solving_algorithm {
                        MazeSolvingAlgorithms::BreadthFirstSearch => &BreadthFirstSearch,

                        MazeSolvingAlgorithms::DepthFirstSearch => &DepthFirstSearch,
                        MazeSolvingAlgorithms::WallFollower => &WallFollower,
                    },
                    &mut screen,
                    animate,
                );
                let solving_sequence = get_solving_sequence(&path);
                let mut solving_sequence: String = solving_sequence.iter().collect();
                if solving_sequence.len() == 0 {
                    solving_sequence = String::from("No solving sequence available.");
                }
                // Print the informations in the UI.
                terminal_ui::print_informations(
                    &mut screen,
                    generation_algorithm.to_string(),
                    solving_algorithm.to_string(),
                    number_of_inspected_cells,
                    animate,
                );
                terminal_ui::print_solving_sequence(&mut screen, solving_sequence);
            }
            Key::Char('h') => {
                // Next generation algorithm.
                generation_algorithm = generation_algorithm.next();
                // Print / reset the informations in the UI.
                terminal_ui::print_informations(
                    &mut screen,
                    generation_algorithm.to_string(),
                    solving_algorithm.to_string(),
                    0,
                    animate,
                );
                terminal_ui::print_solving_sequence(&mut screen, String::new());
                // Recreate.
                maze.generate(
                    match generation_algorithm {
                        MazeGenerationAlgorithms::Kruskal => &Kruskal,
                        MazeGenerationAlgorithms::RecursiveBacktracking => &RecursiveBacktracking,
                        MazeGenerationAlgorithms::Wilson => &Wilson,
                    },
                    &mut screen,
                    animate,
                );
                maze.draw(&mut screen, show_graph);
            }
            Key::Char('l') => {
                // Next solving algorithm.
                solving_algorithm = solving_algorithm.next();
                // Print / reset the informations in the UI.
                terminal_ui::print_informations(
                    &mut screen,
                    generation_algorithm.to_string(),
                    solving_algorithm.to_string(),
                    0,
                    animate,
                );
                terminal_ui::print_solving_sequence(&mut screen, String::new());
                // Redraw the maze but do not solve it yet (may trigger the animation).
                maze.draw(&mut screen, show_graph);
            }
            Key::Char('g') => {
                // Show / hide graph nodes.
                show_graph = !show_graph;
                maze.draw(&mut screen, show_graph);
                // Reset the informations in the UI.
                terminal_ui::print_informations(
                    &mut screen,
                    generation_algorithm.to_string(),
                    solving_algorithm.to_string(),
                    0,
                    animate,
                );
                terminal_ui::print_solving_sequence(&mut screen, String::new());
            }
            Key::Char('a') => {
                // Toggle animation on / off.
                animate = !animate;
                // Print the informations in the UI.
                terminal_ui::print_informations(
                    &mut screen,
                    generation_algorithm.to_string(),
                    solving_algorithm.to_string(),
                    0,
                    animate,
                );
            }
            Key::Char('b') => {
                // Show the binary representation.
                show_representation = !show_representation;
                if show_representation {
                    show_binary_representation(&mut screen, &maze);
                } else {
                    maze.draw(&mut screen, show_graph);
                }
                // Reset the informations in the UI.
                terminal_ui::print_informations(
                    &mut screen,
                    generation_algorithm.to_string(),
                    solving_algorithm.to_string(),
                    0,
                    animate,
                );
                terminal_ui::print_solving_sequence(&mut screen, String::new());
            }
            _ => {}
        }
        screen.flush().unwrap();
    }

    write!(screen, "{}", termion::cursor::Show).unwrap();
}
