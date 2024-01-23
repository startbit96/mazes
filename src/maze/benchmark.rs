use crate::maze::animation::*;
use crate::maze::generator::*;
use crate::maze::maze::*;
use crate::maze::path::calculate_manhattan_distance;
use crate::maze::solver::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

const BENCHMARK_NUMBER_OF_MAZES_PER_GENERATION_ALGORITHM: usize = 100;
const BENCHMARK_NUMBER_OF_RANDOM_POSITIONS_PER_MAZE: usize = 100;

pub struct BenchmarkResult {
    pub maze_id: usize,
    pub generation_algorithm: String,
    pub manhattan_distance: usize,
    pub inspected_cells_per_solving_algorithm: HashMap<String, usize>,
}

pub struct BenchmarkResultCollection {
    maze_width: usize,
    maze_height: usize,
    pub results: Vec<BenchmarkResult>,
}

impl BenchmarkResultCollection {
    pub fn benchmark(screen: &mut dyn Write, maze_width: usize, maze_height: usize) -> Self {
        let generation_algorithms: Vec<&dyn MazeGenerator> =
            vec![&Kruskal, &RecursiveBacktracking, &Wilson];
        let solving_algorithms: Vec<&dyn MazeSolver> =
            vec![&BreadthFirstSearch, &DepthFirstSearch, &AStar];
        let mut results: Vec<BenchmarkResult> = Vec::new();
        let mut maze_id = 0;

        for _ in 0..BENCHMARK_NUMBER_OF_MAZES_PER_GENERATION_ALGORITHM {
            for generation_algorithm in generation_algorithms.iter() {
                let mut maze = Maze::new(maze_width, maze_height, (1, 1));
                if maze.change_size(maze_width, maze_height) == false {
                    panic!();
                };
                maze.generate(*generation_algorithm, screen, false);
                for i in 0..=BENCHMARK_NUMBER_OF_RANDOM_POSITIONS_PER_MAZE {
                    if i > 0 {
                        maze.set_random_start_end_position();
                    }
                    let mut inspected_cells_per_solving_algorithm: HashMap<String, usize> =
                        HashMap::new();
                    for solving_algorithm in solving_algorithms.iter() {
                        // Solve the maze and count the number of inspected cells.
                        let (_, number_of_inspected_cells) =
                            maze.solve(*solving_algorithm, screen, false);
                        inspected_cells_per_solving_algorithm
                            .insert(solving_algorithm.to_string(), number_of_inspected_cells);
                    }
                    results.push(BenchmarkResult {
                        maze_id,
                        generation_algorithm: generation_algorithm.to_string(),
                        manhattan_distance: calculate_manhattan_distance(
                            maze.pos_start,
                            maze.pos_end,
                        ),
                        inspected_cells_per_solving_algorithm,
                    });
                }
                maze_id += 1;
            }
        }

        BenchmarkResultCollection {
            maze_width,
            maze_height,
            results,
        }
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
        file.write_all(b"maze_id;generation_algorithm;manhattan_distance")
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
                    "{};{};{}",
                    result.maze_id, result.generation_algorithm, result.manhattan_distance
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
