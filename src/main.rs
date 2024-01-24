use maze::benchmark::*;
use maze::generator::*;
use maze::maze::Maze;
use maze::maze_collection::MazeCollection;
use maze::maze_container::MazeContainer;
use maze::path::get_solving_sequence;
use maze::solver::*;
use std::io::{stdin, stdout, Write};
use std::time::Instant;
use terminal_ui::{TERMINAL_HEIGHT_MIN, TERMINAL_WIDTH_MIN};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::{IntoAlternateScreen, ToAlternateScreen};

mod maze;
mod terminal_ui;

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
    let mut show_background_graph: bool = false;
    let mut show_binary_representation: bool = false;
    let mut show_background_binary_representation: bool = false;
    let mut animate: bool = false;

    // Selected algorithms.
    let mut generation_algorithm = MazeGenerationAlgorithms::Kruskal;
    let mut solving_algorithm = MazeSolvingAlgorithms::DepthFirstSearch;

    // Initialize the maze with the information about its max size.
    let (max_maze_width, max_maze_height) = terminal_ui::get_max_draw_size();
    let mut maze_container =
        MazeContainer::SingleMaze(Maze::new(max_maze_width, max_maze_height, (1, 1)));

    // Draw terminal ui and the maze.
    terminal_ui::intialize_terminal_ui(&mut screen);
    terminal_ui::print_informations(
        &mut screen,
        maze_container.get_size(),
        generation_algorithm.to_string(),
        solving_algorithm.to_string(),
        0,
        animate,
    );

    // Generate the first maze.
    maze_container.generate(
        match generation_algorithm {
            MazeGenerationAlgorithms::Kruskal => &Kruskal,
            MazeGenerationAlgorithms::RecursiveBacktracking => &RecursiveBacktracking,
            MazeGenerationAlgorithms::Wilson => &Wilson,
        },
        &mut screen,
        animate,
    );
    maze_container.draw(
        &mut screen,
        show_graph,
        show_background_graph,
        show_binary_representation,
        show_background_binary_representation,
    );

    // The main loop that keeps the program alive. q breaks it.
    for c in stdin.keys() {
        // Process the input.
        let key = c.unwrap();
        match key {
            Key::Char('q') => break,
            Key::Ctrl('l') => {
                // Redraw everything.
                terminal_ui::intialize_terminal_ui(&mut screen);
                terminal_ui::print_informations(
                    &mut screen,
                    maze_container.get_size(),
                    generation_algorithm.to_string(),
                    solving_algorithm.to_string(),
                    0,
                    animate,
                );
                terminal_ui::print_solving_sequence(&mut screen, String::new());
                maze_container.draw(
                    &mut screen,
                    show_graph,
                    show_background_graph,
                    show_binary_representation,
                    show_background_binary_representation,
                );
            }
            Key::Char('r') => {
                // Reset the informations in the UI.
                terminal_ui::print_informations(
                    &mut screen,
                    maze_container.get_size(),
                    generation_algorithm.to_string(),
                    solving_algorithm.to_string(),
                    0,
                    animate,
                );
                terminal_ui::print_solving_sequence(&mut screen, String::new());
                // Recreate.
                maze_container.generate(
                    match generation_algorithm {
                        MazeGenerationAlgorithms::Kruskal => &Kruskal,
                        MazeGenerationAlgorithms::RecursiveBacktracking => &RecursiveBacktracking,
                        MazeGenerationAlgorithms::Wilson => &Wilson,
                    },
                    &mut screen,
                    animate,
                );
                maze_container.draw(
                    &mut screen,
                    show_graph,
                    show_background_graph,
                    show_binary_representation,
                    show_background_binary_representation,
                );
            }
            Key::Up | Key::Char('k') => {
                // Increase size.
                if maze_container.change_size(2, 2) {
                    terminal_ui::erase_draw_area(&mut screen);
                    // Generate without animation.
                    maze_container.generate(
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
                    maze_container.draw(
                        &mut screen,
                        show_graph,
                        show_background_graph,
                        show_binary_representation,
                        show_background_binary_representation,
                    );
                    // Reset the informations in the UI.
                    terminal_ui::print_informations(
                        &mut screen,
                        maze_container.get_size(),
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
                if maze_container.change_size(-2, -2) {
                    terminal_ui::erase_draw_area(&mut screen);
                    // Generate without animation.
                    maze_container.generate(
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
                    maze_container.draw(
                        &mut screen,
                        show_graph,
                        show_background_graph,
                        show_binary_representation,
                        show_background_binary_representation,
                    );
                    // Reset the informations in the UI.
                    terminal_ui::print_informations(
                        &mut screen,
                        maze_container.get_size(),
                        generation_algorithm.to_string(),
                        solving_algorithm.to_string(),
                        0,
                        animate,
                    );
                    terminal_ui::print_solving_sequence(&mut screen, String::new());
                }
            }
            Key::Char('n') => {
                // Set random start and end position for the maze. (only for single maze)
                if let MazeContainer::SingleMaze(ref mut maze) = maze_container {
                    // Reset the informations in the UI.
                    terminal_ui::print_informations(
                        &mut screen,
                        (maze.width, maze.height),
                        generation_algorithm.to_string(),
                        solving_algorithm.to_string(),
                        0,
                        animate,
                    );
                    terminal_ui::print_solving_sequence(&mut screen, String::new());
                    // Make the start and end position random.
                    maze.set_random_start_end_position();
                    // Redraw.
                    maze.draw(
                        &mut screen,
                        show_graph,
                        show_background_graph,
                        show_binary_representation,
                        show_background_binary_representation,
                    );
                }
            }
            Key::Char('m') => {
                // Reset start and end position for the maze. (only for single maze)
                if let MazeContainer::SingleMaze(ref mut maze) = maze_container {
                    // Reset the informations in the UI.
                    terminal_ui::print_informations(
                        &mut screen,
                        (maze.width, maze.height),
                        generation_algorithm.to_string(),
                        solving_algorithm.to_string(),
                        0,
                        animate,
                    );
                    terminal_ui::print_solving_sequence(&mut screen, String::new());
                    // Reset the start and end position.
                    maze.reset_start_end_position();
                    // Redraw.
                    maze.draw(
                        &mut screen,
                        show_graph,
                        show_background_graph,
                        show_binary_representation,
                        show_background_binary_representation,
                    );
                }
            }
            Key::Char('s') => {
                if let MazeContainer::MultipleMazes(_) = maze_container {
                    if solving_algorithm == MazeSolvingAlgorithms::WallFollower {
                        terminal_ui::print_solving_sequence(&mut screen, String::from("For multiple mazes, the solving algorithm 'wall follower' is not supported. Please change the solving algorithm."));
                        continue;
                    }
                }
                let (path, number_of_inspected_cells) = maze_container.solve(
                    match solving_algorithm {
                        MazeSolvingAlgorithms::AStar => &AStar,
                        MazeSolvingAlgorithms::AStarWeighted => &AStarWeighted,
                        MazeSolvingAlgorithms::BreadthFirstSearch => &BreadthFirstSearch,

                        MazeSolvingAlgorithms::DepthFirstSearch => &DepthFirstSearch,
                        MazeSolvingAlgorithms::GreedyBestFirstSearch => &GreedyBestFirstSearch,
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
                    maze_container.get_size(),
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
                    maze_container.get_size(),
                    generation_algorithm.to_string(),
                    solving_algorithm.to_string(),
                    0,
                    animate,
                );
                terminal_ui::print_solving_sequence(&mut screen, String::new());
                // Recreate (without animation).
                maze_container.generate(
                    match generation_algorithm {
                        MazeGenerationAlgorithms::Kruskal => &Kruskal,
                        MazeGenerationAlgorithms::RecursiveBacktracking => &RecursiveBacktracking,
                        MazeGenerationAlgorithms::Wilson => &Wilson,
                    },
                    &mut screen,
                    false,
                );
                maze_container.draw(
                    &mut screen,
                    show_graph,
                    show_background_graph,
                    show_binary_representation,
                    show_background_binary_representation,
                );
            }
            Key::Char('l') => {
                // Next solving algorithm.
                solving_algorithm = solving_algorithm.next();
                // Print / reset the informations in the UI.
                terminal_ui::print_informations(
                    &mut screen,
                    maze_container.get_size(),
                    generation_algorithm.to_string(),
                    solving_algorithm.to_string(),
                    0,
                    animate,
                );
                terminal_ui::print_solving_sequence(&mut screen, String::new());
                // Redraw the maze but do not solve it yet (may trigger the animation).
                maze_container.draw(
                    &mut screen,
                    show_graph,
                    show_background_graph,
                    show_binary_representation,
                    show_background_binary_representation,
                );
            }
            Key::Char('g') => {
                // Show / hide graph nodes.
                (show_graph, show_background_graph) = match (show_graph, show_background_graph) {
                    (false, false) => (true, true),
                    (true, true) => (true, false),
                    (true, false) => (false, false),
                    _ => unreachable!(),
                };
                if show_graph {
                    show_binary_representation = false;
                    show_background_binary_representation = false;
                }
                maze_container.draw(
                    &mut screen,
                    show_graph,
                    show_background_graph,
                    show_binary_representation,
                    show_background_binary_representation,
                );
                // Reset the informations in the UI.
                terminal_ui::print_informations(
                    &mut screen,
                    maze_container.get_size(),
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
                    maze_container.get_size(),
                    generation_algorithm.to_string(),
                    solving_algorithm.to_string(),
                    0,
                    animate,
                );
            }
            Key::Char('b') => {
                // Show the binary representation.
                (
                    show_binary_representation,
                    show_background_binary_representation,
                ) = match (
                    show_binary_representation,
                    show_background_binary_representation,
                ) {
                    (false, false) => (true, false),
                    (true, false) => (true, true),
                    (true, true) => (false, false),
                    _ => unreachable!(),
                };
                if show_binary_representation {
                    show_graph = false;
                    show_background_graph = false;
                }
                maze_container.draw(
                    &mut screen,
                    show_graph,
                    show_background_graph,
                    show_binary_representation,
                    show_background_binary_representation,
                );
                // Reset the informations in the UI.
                terminal_ui::print_informations(
                    &mut screen,
                    maze_container.get_size(),
                    generation_algorithm.to_string(),
                    solving_algorithm.to_string(),
                    0,
                    animate,
                );
                terminal_ui::print_solving_sequence(&mut screen, String::new());
            }
            Key::Char('1') => {
                // Switch to a single maze.
                if let MazeContainer::MultipleMazes(_) = maze_container {
                    // Reset the informations in the UI.
                    terminal_ui::erase_draw_area(&mut screen);
                    terminal_ui::print_informations(
                        &mut screen,
                        maze_container.get_size(),
                        generation_algorithm.to_string(),
                        solving_algorithm.to_string(),
                        0,
                        animate,
                    );
                    terminal_ui::print_solving_sequence(&mut screen, String::new());
                    // Generate the maze (without animation) and print it.
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
                        false,
                    );
                    maze.draw(
                        &mut screen,
                        show_graph,
                        show_background_graph,
                        show_binary_representation,
                        show_background_binary_representation,
                    );
                    maze_container = MazeContainer::SingleMaze(maze);
                }
            }
            Key::Char('2') | Key::Char('3') | Key::Char('4') | Key::Char('5') => {
                if let Key::Char(number_of_mazes) = key {
                    let number_of_mazes = (number_of_mazes as u8 - b'0') as usize;
                    let maze_collection: Option<MazeCollection> =
                        if let MazeContainer::SingleMaze(_) = maze_container {
                            Some(MazeCollection::new(
                                max_maze_width,
                                max_maze_height,
                                number_of_mazes,
                            ))
                        } else if let MazeContainer::MultipleMazes(ref mut old_maze_collection) =
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
                            maze_container.get_size(),
                            generation_algorithm.to_string(),
                            solving_algorithm.to_string(),
                            0,
                            animate,
                        );
                        terminal_ui::print_solving_sequence(&mut screen, String::new());
                        // Create the mazes without animation and draw them.
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
                        maze_collection.draw(
                            &mut screen,
                            show_graph,
                            show_background_graph,
                            show_binary_representation,
                            show_background_binary_representation,
                        );

                        maze_container = MazeContainer::MultipleMazes(maze_collection);
                    }
                }
            }
            Key::Char('o') => {
                // Reorder the mazes in the maze collection (to show that the order
                // of the mazes has an effect on the length of the solving sequence).
                maze_container.reorder();
                maze_container.draw(
                    &mut screen,
                    show_graph,
                    show_background_graph,
                    show_binary_representation,
                    show_background_binary_representation,
                );
            }
            Key::Char('t') => {
                // Benchmark.
                terminal_ui::erase_draw_area(&mut screen);
                terminal_ui::print_solving_sequence(
                    &mut screen,
                    String::from("Executing benchmark ..."),
                );
                let start_time = Instant::now();
                let mut benchmark_results = BenchmarkResultCollection::new();
                loop {
                    let (is_running, progress) = benchmark_results.benchmark_next_chunk();
                    terminal_ui::print_solving_sequence(
                        &mut screen,
                        format!("Executing benchmark ... ({}%)", progress),
                    );
                    if is_running == false {
                        break;
                    }
                }
                let csv_filename = benchmark_results.to_csv();
                let end_time = Instant::now();
                let elapsed_time = end_time - start_time;
                terminal_ui::print_solving_sequence(
                    &mut screen,
                    format!(
                        "Benchmark results written to '{}'. Benchmark took {} minutes and {} seconds.",
                        csv_filename,
                        elapsed_time.as_secs() / 60,
                        elapsed_time.as_secs() - (elapsed_time.as_secs() / 60) * 60
                    ),
                );
                // Reset to the previous setting.
                terminal_ui::print_informations(
                    &mut screen,
                    maze_container.get_size(),
                    generation_algorithm.to_string(),
                    solving_algorithm.to_string(),
                    0,
                    animate,
                );
                maze_container.draw(
                    &mut screen,
                    show_graph,
                    show_background_graph,
                    show_binary_representation,
                    show_background_binary_representation,
                );
            }
            _ => {}
        }
        screen.flush().unwrap();
    }

    write!(screen, "{}", termion::cursor::Show).unwrap();
}
