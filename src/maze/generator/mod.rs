use crate::maze::animation::Delay;
use crate::maze::maze::Maze;
use std::io::Write;

pub const GENERATION_DELAY: Delay = Delay::Long;

pub trait MazeGenerator {
    fn generate(&self, maze: &mut Maze, screen: &mut dyn Write, animate: bool);
}

pub mod kruskal;
