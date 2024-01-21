use maze::generator::{
    kruskal::Kruskal, recursive_backtracking::RecursiveBacktracking, wilson::Wilson,
    MazeGenerationAlgorithms,
};
use maze::maze::Maze;
use maze::maze_collection::MazeCollection;
use maze::path::get_solving_sequence;
use maze::solver::{
    breadth_first_search::BreadthFirstSearch, depth_first_search::DepthFirstSearch,
    wall_follower::WallFollower, MazeSolvingAlgorithms,
};
use std::io::{stdin, stdout, Write};
use terminal_ui::{TERMINAL_HEIGHT_MIN, TERMINAL_WIDTH_MIN};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::{IntoAlternateScreen, ToAlternateScreen};

mod maze;
mod terminal_ui;

pub enum MazeType {
    SingleMaze(Maze),
    MultipleMazes(MazeCollection),
}

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
    let mut maze_container =
        MazeType::SingleMaze(Maze::new(max_maze_width, max_maze_height, (1, 1)));

    // Generate the first maze.
    if let MazeType::SingleMaze(ref mut maze) = maze_container {
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
    } else if let MazeType::MultipleMazes(ref mut maze_collection) = maze_container {
        maze_collection.generate(
            match generation_algorithm {
                MazeGenerationAlgorithms::Kruskal => &Kruskal,
                MazeGenerationAlgorithms::RecursiveBacktracking => &RecursiveBacktracking,
                MazeGenerationAlgorithms::Wilson => &Wilson,
            },
            &mut screen,
            animate,
        );
        maze_collection.draw(&mut screen, show_graph);
    }

    // The main loop that keeps the program alive. q breaks it.
    for c in stdin.keys() {
        // Process the input.
        let key = c.unwrap();
        match key {
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
                if let MazeType::SingleMaze(ref mut maze) = maze_container {
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
                } else if let MazeType::MultipleMazes(ref mut maze_collection) = maze_container {
                    maze_collection.generate(
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
                    maze_collection.draw(&mut screen, show_graph);
                }
            }
            Key::Up | Key::Char('k') => {
                // Increase size.
                if let MazeType::SingleMaze(ref mut maze) = maze_container {
                    if maze.change_size(maze.width + 2, maze.height + 2) {
                        terminal_ui::erase_draw_area(&mut screen);
                        // Generate without animation.
                        maze.generate(
                            match generation_algorithm {
                                MazeGenerationAlgorithms::Kruskal => &Kruskal,
                                MazeGenerationAlgorithms::RecursiveBacktracking => {
                                    &RecursiveBacktracking
                                }
                                MazeGenerationAlgorithms::Wilson => &Wilson,
                            },
                            &mut screen,
                            false,
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
                } else if let MazeType::MultipleMazes(ref mut maze_collection) = maze_container {
                    if maze_collection.change_size(
                        maze_collection.mazes[0].width + 2,
                        maze_collection.mazes[0].height + 2,
                    ) {
                        terminal_ui::erase_draw_area(&mut screen);
                        // Generate without animation.
                        maze_collection.generate(
                            match generation_algorithm {
                                MazeGenerationAlgorithms::Kruskal => &Kruskal,
                                MazeGenerationAlgorithms::RecursiveBacktracking => {
                                    &RecursiveBacktracking
                                }
                                MazeGenerationAlgorithms::Wilson => &Wilson,
                            },
                            &mut screen,
                            false,
                        );
                        maze_collection.draw(&mut screen, show_graph);
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
            }
            Key::Down | Key::Char('j') => {
                // Decrease size.
                if let MazeType::SingleMaze(ref mut maze) = maze_container {
                    if maze.change_size(maze.width - 2, maze.height - 2) {
                        terminal_ui::erase_draw_area(&mut screen);
                        // Generate without animation.
                        maze.generate(
                            match generation_algorithm {
                                MazeGenerationAlgorithms::Kruskal => &Kruskal,
                                MazeGenerationAlgorithms::RecursiveBacktracking => {
                                    &RecursiveBacktracking
                                }
                                MazeGenerationAlgorithms::Wilson => &Wilson,
                            },
                            &mut screen,
                            false,
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
                } else if let MazeType::MultipleMazes(ref mut maze_collection) = maze_container {
                    if maze_collection.change_size(
                        maze_collection.mazes[0].width - 2,
                        maze_collection.mazes[0].height - 2,
                    ) {
                        terminal_ui::erase_draw_area(&mut screen);
                        // Generate without animation.
                        maze_collection.generate(
                            match generation_algorithm {
                                MazeGenerationAlgorithms::Kruskal => &Kruskal,
                                MazeGenerationAlgorithms::RecursiveBacktracking => {
                                    &RecursiveBacktracking
                                }
                                MazeGenerationAlgorithms::Wilson => &Wilson,
                            },
                            &mut screen,
                            false,
                        );
                        maze_collection.draw(&mut screen, show_graph);
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
            }
            Key::Char('n') => {
                // Set random start and end position for the maze. (only for single maze)
                if let MazeType::SingleMaze(ref mut maze) = maze_container {
                    // Reset the informations in the UI.
                    terminal_ui::print_informations(
                        &mut screen,
                        generation_algorithm.to_string(),
                        solving_algorithm.to_string(),
                        0,
                        animate,
                    );
                    terminal_ui::print_solving_sequence(&mut screen, String::new());
                    // Make the start and end position random.
                    maze.set_random_start_end_position();
                    // Redraw.
                    maze.draw(&mut screen, show_graph);
                }
            }
            Key::Char('m') => {
                // Reset start and end position for the maze. (only for single maze)
                if let MazeType::SingleMaze(ref mut maze) = maze_container {
                    // Reset the informations in the UI.
                    terminal_ui::print_informations(
                        &mut screen,
                        generation_algorithm.to_string(),
                        solving_algorithm.to_string(),
                        0,
                        animate,
                    );
                    terminal_ui::print_solving_sequence(&mut screen, String::new());
                    // Reset the start and end position.
                    maze.reset_start_end_position();
                    // Redraw.
                    maze.draw(&mut screen, show_graph);
                }
            }
            Key::Char('s') => {
                let (path, number_of_inspected_cells) =
                    if let MazeType::SingleMaze(ref maze) = maze_container {
                        maze.solve(
                            match solving_algorithm {
                                MazeSolvingAlgorithms::BreadthFirstSearch => &BreadthFirstSearch,

                                MazeSolvingAlgorithms::DepthFirstSearch => &DepthFirstSearch,
                                MazeSolvingAlgorithms::WallFollower => &WallFollower,
                            },
                            &mut screen,
                            animate,
                        )
                    } else if let MazeType::MultipleMazes(ref maze_collection) = maze_container {
                        maze_collection.solve(
                            match solving_algorithm {
                                MazeSolvingAlgorithms::BreadthFirstSearch => &BreadthFirstSearch,

                                MazeSolvingAlgorithms::DepthFirstSearch => &DepthFirstSearch,
                                MazeSolvingAlgorithms::WallFollower => &WallFollower,
                            },
                            &mut screen,
                            animate,
                        )
                    } else {
                        panic!()
                    };
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
                // Recreate (without animation).
                if let MazeType::SingleMaze(ref mut maze) = maze_container {
                    maze.generate(
                        match generation_algorithm {
                            MazeGenerationAlgorithms::Kruskal => &Kruskal,
                            MazeGenerationAlgorithms::RecursiveBacktracking => {
                                &RecursiveBacktracking
                            }
                            MazeGenerationAlgorithms::Wilson => &Wilson,
                        },
                        &mut screen,
                        false,
                    );
                    maze.draw(&mut screen, show_graph);
                } else if let MazeType::MultipleMazes(ref mut maze_collection) = maze_container {
                    maze_collection.generate(
                        match generation_algorithm {
                            MazeGenerationAlgorithms::Kruskal => &Kruskal,
                            MazeGenerationAlgorithms::RecursiveBacktracking => {
                                &RecursiveBacktracking
                            }
                            MazeGenerationAlgorithms::Wilson => &Wilson,
                        },
                        &mut screen,
                        false,
                    );
                    maze_collection.draw(&mut screen, show_graph);
                }
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
                if let MazeType::SingleMaze(ref maze) = maze_container {
                    maze.draw(&mut screen, show_graph);
                } else if let MazeType::MultipleMazes(ref maze_collection) = maze_container {
                    maze_collection.draw(&mut screen, show_graph);
                }
            }
            Key::Char('g') => {
                // Show / hide graph nodes.
                show_graph = !show_graph;
                if let MazeType::SingleMaze(ref maze) = maze_container {
                    maze.draw(&mut screen, show_graph);
                } else if let MazeType::MultipleMazes(ref maze_collection) = maze_container {
                    maze_collection.draw(&mut screen, show_graph);
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
                if let MazeType::SingleMaze(ref maze) = maze_container {
                    if show_representation {
                        maze.show_binary_representation(&mut screen);
                    } else {
                        maze.draw(&mut screen, show_graph);
                    }
                } else if let MazeType::MultipleMazes(ref maze_collection) = maze_container {
                    if show_representation {
                        maze_collection.show_binary_representation(&mut screen);
                    } else {
                        maze_collection.draw(&mut screen, show_graph);
                    }
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
            Key::Char('1') => {
                // Switch to a single maze.
                if let MazeType::MultipleMazes(_) = maze_container {
                    // Reset the informations in the UI.
                    terminal_ui::erase_draw_area(&mut screen);
                    terminal_ui::print_informations(
                        &mut screen,
                        generation_algorithm.to_string(),
                        solving_algorithm.to_string(),
                        0,
                        animate,
                    );
                    terminal_ui::print_solving_sequence(&mut screen, String::new());
                    // Generate the maze and print it.
                    let mut maze = Maze::new(max_maze_width, max_maze_height, (1, 1));
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
                    maze_container = MazeType::SingleMaze(maze);
                }
            }
            Key::Char('2') | Key::Char('3') | Key::Char('4') | Key::Char('5') => {
                if let Key::Char(number_of_mazes) = key {
                    let number_of_mazes = (number_of_mazes as u8 - b'0') as usize;
                    let maze_collection: Option<MazeCollection> =
                        if let MazeType::SingleMaze(_) = maze_container {
                            Some(MazeCollection::new(
                                max_maze_width,
                                max_maze_height,
                                number_of_mazes,
                            ))
                        } else if let MazeType::MultipleMazes(ref mut old_maze_collection) =
                            maze_container
                        {
                            if old_maze_collection.number_of_mazes == number_of_mazes {
                                None
                            } else {
                                Some(MazeCollection::new(
                                    max_maze_width,
                                    max_maze_height,
                                    number_of_mazes,
                                ))
                            }
                        } else {
                            panic!()
                        };
                    if let Some(mut maze_collection) = maze_collection {
                        // Reset the informations in the UI.
                        terminal_ui::erase_draw_area(&mut screen);
                        terminal_ui::print_informations(
                            &mut screen,
                            generation_algorithm.to_string(),
                            solving_algorithm.to_string(),
                            0,
                            animate,
                        );
                        terminal_ui::print_solving_sequence(&mut screen, String::new());
                        // Create the mazes and draw them.
                        maze_collection.generate(
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
                        maze_collection.draw(&mut screen, show_graph);
                        maze_container = MazeType::MultipleMazes(maze_collection);
                    }
                }
            }
            _ => {}
        }
        screen.flush().unwrap();
    }

    write!(screen, "{}", termion::cursor::Show).unwrap();
}
