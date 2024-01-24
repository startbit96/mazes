use crate::maze::generator::*;
use crate::maze::maze::*;
use crate::maze::path::calculate_manhattan_distance;
use crate::maze::solver::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Result, Write};

const BENCHMARK_MAZE_WIDTH: usize = 45;
const BENCHMARK_MAZE_HEIGHT: usize = 45;
const BENCHMARK_NUMBER_OF_MAZES_PER_GENERATION_ALGORITHM: usize = 1000;
const BENCHMARK_CHUNK_SIZE: usize = 2;
const BENCHMARK_NUMBER_OF_RANDOM_POSITIONS_PER_MAZE: usize = 10;

pub struct NullWriter;

impl Write for NullWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

pub struct BenchmarkResult {
    pub maze_id: usize,
    pub generation_algorithm: String,
    pub manhattan_distance: usize,
    pub path_length: usize,
    pub inspected_cells_per_solving_algorithm: HashMap<String, usize>,
}

pub struct BenchmarkResultCollection {
    maze_width: usize,
    maze_height: usize,
    pub results: Vec<BenchmarkResult>,
}

impl BenchmarkResultCollection {
    pub fn new() -> Self {
        Self {
            maze_width: BENCHMARK_MAZE_WIDTH,
            maze_height: BENCHMARK_MAZE_HEIGHT,
            results: Vec::new(),
        }
    }

    fn calculate_total_number_of_mazes(number_of_generation_algorithms: usize) -> usize {
        BENCHMARK_NUMBER_OF_MAZES_PER_GENERATION_ALGORITHM * number_of_generation_algorithms
    }

    fn calculate_current_number_of_mazes(&self) -> usize {
        self.results.len() / (BENCHMARK_NUMBER_OF_RANDOM_POSITIONS_PER_MAZE + 1)
    }

    fn calculate_total_number_of_result_entries(number_of_generation_algorithms: usize) -> usize {
        BENCHMARK_NUMBER_OF_MAZES_PER_GENERATION_ALGORITHM
            * number_of_generation_algorithms
            * (BENCHMARK_NUMBER_OF_RANDOM_POSITIONS_PER_MAZE + 1)
    }

    pub fn benchmark_next_chunk(&mut self) -> (bool, usize) {
        let mut screen = NullWriter;
        let generation_algorithms: Vec<&dyn MazeGenerator> =
            vec![&Kruskal, &RecursiveBacktracking, &Wilson];
        let solving_algorithms: Vec<&dyn MazeSolver> = vec![
            &BreadthFirstSearch,
            &DepthFirstSearch,
            &AStar,
            &AStarWeighted,
        ];

        // Are we done?
        if self.results.len()
            == Self::calculate_total_number_of_result_entries(generation_algorithms.len())
        {
            return (false, 100);
        } else if self.results.len()
            > Self::calculate_total_number_of_result_entries(generation_algorithms.len())
        {
            panic!(
                "Too many entries! Expected: {}, Found: {}.",
                Self::calculate_total_number_of_result_entries(generation_algorithms.len()),
                self.results.len()
            );
        }

        let mut maze_id = self.calculate_current_number_of_mazes();
        let maze_id_end = std::cmp::min(
            maze_id + BENCHMARK_CHUNK_SIZE * generation_algorithms.len(),
            Self::calculate_total_number_of_mazes(generation_algorithms.len()),
        );
        while maze_id < maze_id_end {
            for generation_algorithm in generation_algorithms.iter() {
                let mut maze = Maze::new(BENCHMARK_MAZE_WIDTH, BENCHMARK_MAZE_HEIGHT, (1, 1));
                if maze.change_size(BENCHMARK_MAZE_WIDTH, BENCHMARK_MAZE_HEIGHT) == false {
                    panic!();
                };
                maze.generate(*generation_algorithm, &mut screen, false);
                for i in 0..=BENCHMARK_NUMBER_OF_RANDOM_POSITIONS_PER_MAZE {
                    if i > 0 {
                        maze.set_random_start_end_position();
                    }
                    let mut inspected_cells_per_solving_algorithm: HashMap<String, usize> =
                        HashMap::new();
                    let mut path_length = 0;
                    for solving_algorithm in solving_algorithms.iter() {
                        // Solve the maze and count the number of inspected cells.
                        let (path, number_of_inspected_cells) =
                            maze.solve(*solving_algorithm, &mut screen, false);
                        if path_length != 0 && path_length != path.len() {
                            panic!()
                        }
                        path_length = path.len();
                        inspected_cells_per_solving_algorithm
                            .insert(solving_algorithm.to_string(), number_of_inspected_cells);
                    }
                    self.results.push(BenchmarkResult {
                        maze_id,
                        generation_algorithm: generation_algorithm.to_string(),
                        manhattan_distance: calculate_manhattan_distance(
                            maze.pos_start,
                            maze.pos_end,
                        ),
                        path_length,
                        inspected_cells_per_solving_algorithm,
                    });
                }
                maze_id += 1;
            }
        }
        (
            true,
            ((maze_id_end as f64
                / Self::calculate_total_number_of_mazes(generation_algorithms.len()) as f64)
                * 100_f64) as usize,
        )
    }

    pub fn to_csv(&self) -> String {
        let filename = format!(
            "benchmark_analysis/maze_benchmark_size_{}x{}_{}_mazes_{}_random_positions.csv",
            self.maze_width,
            self.maze_height,
            BENCHMARK_NUMBER_OF_MAZES_PER_GENERATION_ALGORITHM,
            BENCHMARK_NUMBER_OF_RANDOM_POSITIONS_PER_MAZE
        );
        let mut file = File::create(filename.clone()).unwrap();
        // Header.
        file.write_all(b"maze_id;generation_algorithm;manhattan_distance;path_length")
            .unwrap();
        let solving_algorithms: Vec<String> = self.results[0]
            .inspected_cells_per_solving_algorithm
            .keys()
            .cloned()
            .collect();
        for solving_algorithm in solving_algorithms.iter() {
            file.write_all(format!(";{}", solving_algorithm).into_bytes().as_slice())
                .unwrap();
        }
        file.write_all(b"\n").unwrap();
        // Write all measurements into the csv file.
        for result in self.results.iter() {
            file.write_all(
                format!(
                    "{};{};{};{}",
                    result.maze_id,
                    result.generation_algorithm,
                    result.manhattan_distance,
                    result.path_length
                )
                .into_bytes()
                .as_slice(),
            )
            .unwrap();
            for solving_algorithm in solving_algorithms.iter() {
                file.write_all(
                    format!(
                        ";{}",
                        result
                            .inspected_cells_per_solving_algorithm
                            .get(solving_algorithm)
                            .unwrap()
                    )
                    .into_bytes()
                    .as_slice(),
                )
                .unwrap();
            }
            file.write_all(b"\n").unwrap();
        }
        // Return the filename.
        filename
    }
}
