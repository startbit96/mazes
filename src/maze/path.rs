use crate::maze::direction::AbsoluteDirection;
use crate::maze::maze::*;

#[derive(Debug, PartialEq, Eq)]
enum PathOrientation {
    Horizontal,
    Vertical,
}

impl PathOrientation {
    fn from_points(pos1: (usize, usize), pos2: (usize, usize)) -> Self {
        match (pos1.0 == pos2.0, pos1.1 == pos2.1) {
            (true, false) => PathOrientation::Vertical,
            (false, true) => PathOrientation::Horizontal,
            _ => panic!("Diagonally is currently not supported."),
        }
    }
}

pub fn complete_line(pos_from: (usize, usize), pos_to: (usize, usize)) -> Vec<(usize, usize)> {
    let orientation = PathOrientation::from_points(pos_from, pos_to);
    match orientation {
        PathOrientation::Vertical => {
            let mut line = (pos_from.1.min(pos_to.1)..=pos_from.1.max(pos_to.1))
                .map(|y| (pos_from.0, y))
                .collect::<Vec<(usize, usize)>>();
            if pos_from.1 > pos_to.1 {
                line.reverse();
            }
            return line;
        }
        PathOrientation::Horizontal => {
            let mut line = (pos_from.0.min(pos_to.0)..=pos_from.0.max(pos_to.0))
                .map(|x| (x, pos_from.1))
                .collect::<Vec<(usize, usize)>>();
            if pos_from.0 > pos_to.0 {
                line.reverse();
            }
            return line;
        }
    }
}

pub fn complete_path(path: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    // This function only implements straight lines, if more is needed, Bresenham will be implemented.
    path.windows(2)
        .enumerate()
        .flat_map(|(idx, window)| {
            let mut line = complete_line(window[0], window[1]);
            if idx > 0 {
                // Otherwise we would have the junctions twice.
                line.remove(0);
            }
            line
        })
        .collect()
}

pub fn get_solving_sequence(path: &Vec<(usize, usize)>) -> Vec<char> {
    path.windows(2)
        .map(|window| {
            let direction = AbsoluteDirection::from_points(window[0], window[1]);
            direction.to_char()
        })
        .collect()
}

pub fn apply_step(
    maze: &Maze,
    pos_start: (usize, usize),
    solving_sequence_step: char,
) -> (usize, usize) {
    let direction = AbsoluteDirection::from_char(solving_sequence_step);
    let pos_next = direction.apply(pos_start);
    if maze.is_accessible(pos_next) {
        return pos_next;
    } else {
        return pos_start;
    }
}

pub fn apply_solving_sequence(
    maze: &Maze,
    pos_start: (usize, usize),
    solving_sequence: Vec<char>,
) -> Vec<(usize, usize)> {
    // Returns the path filtered by "useless" steps.
    // The last value in the path is the resulting position.
    let mut pos = pos_start;
    let mut path = vec![pos_start];
    for &c in solving_sequence.iter() {
        let pos_next = apply_step(maze, pos, c);
        if pos_next != pos {
            pos = pos_next;
            path.push(pos_next);
        }
    }
    path
}

pub fn calculate_manhattan_distance(pos1: (usize, usize), pos2: (usize, usize)) -> usize {
    pos1.0.abs_diff(pos2.0) + pos1.1.abs_diff(pos2.1)
}
