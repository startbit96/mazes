mod maze;

use maze::generator::Kruskal;
use maze::Maze;

fn main() {
    let mut maze = Maze::new(41, 21);
    maze.generate(&Kruskal);
    maze.draw();
}
