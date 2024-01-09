#[derive(Debug, Copy, Clone)]
pub enum AbsoluteDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Copy, Clone)]
pub enum RelativeDirection {
    Forward,
    Backward,
    Left,
    Right,
}

impl AbsoluteDirection {
    pub fn from_points(pos_from: (usize, usize), pos_to: (usize, usize)) -> Self {
        match (
            pos_to.0 as isize - pos_from.0 as isize,
            pos_to.1 as isize - pos_from.1 as isize,
        ) {
            (-1, 0) => AbsoluteDirection::Left,
            (1, 0) => AbsoluteDirection::Right,
            (0, -1) => AbsoluteDirection::Up,
            (0, 1) => AbsoluteDirection::Down,
            _ => panic!("Only one step per time is supported."),
        }
    }

    pub fn from_char(c: char) -> Self {
        match c {
            'L' | 'l' => AbsoluteDirection::Left,
            'R' | 'r' => AbsoluteDirection::Right,
            'U' | 'u' => AbsoluteDirection::Up,
            'D' | 'd' => AbsoluteDirection::Down,
            _ => panic!("Unknown direction!"),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            AbsoluteDirection::Left => 'L',
            AbsoluteDirection::Right => 'R',
            AbsoluteDirection::Up => 'U',
            AbsoluteDirection::Down => 'D',
        }
    }

    pub fn rev(&self) -> Self {
        match self {
            AbsoluteDirection::Left => AbsoluteDirection::Right,
            AbsoluteDirection::Right => AbsoluteDirection::Left,
            AbsoluteDirection::Up => AbsoluteDirection::Down,
            AbsoluteDirection::Down => AbsoluteDirection::Up,
        }
    }

    pub fn add_relative_direction(&self, relative_direction: RelativeDirection) -> Self {
        match (self, relative_direction) {
            // Left.
            (AbsoluteDirection::Left, RelativeDirection::Forward) => AbsoluteDirection::Left,
            (AbsoluteDirection::Left, RelativeDirection::Backward) => AbsoluteDirection::Right,
            (AbsoluteDirection::Left, RelativeDirection::Left) => AbsoluteDirection::Down,
            (AbsoluteDirection::Left, RelativeDirection::Right) => AbsoluteDirection::Up,
            // Right.
            (AbsoluteDirection::Right, RelativeDirection::Forward) => AbsoluteDirection::Right,
            (AbsoluteDirection::Right, RelativeDirection::Backward) => AbsoluteDirection::Left,
            (AbsoluteDirection::Right, RelativeDirection::Left) => AbsoluteDirection::Up,
            (AbsoluteDirection::Right, RelativeDirection::Right) => AbsoluteDirection::Down,
            // Up.
            (AbsoluteDirection::Up, RelativeDirection::Forward) => AbsoluteDirection::Up,
            (AbsoluteDirection::Up, RelativeDirection::Backward) => AbsoluteDirection::Down,
            (AbsoluteDirection::Up, RelativeDirection::Left) => AbsoluteDirection::Left,
            (AbsoluteDirection::Up, RelativeDirection::Right) => AbsoluteDirection::Right,
            // Down.
            (AbsoluteDirection::Down, RelativeDirection::Forward) => AbsoluteDirection::Down,
            (AbsoluteDirection::Down, RelativeDirection::Backward) => AbsoluteDirection::Up,
            (AbsoluteDirection::Down, RelativeDirection::Left) => AbsoluteDirection::Right,
            (AbsoluteDirection::Down, RelativeDirection::Right) => AbsoluteDirection::Left,
        }
    }

    pub fn apply(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            AbsoluteDirection::Left => (pos.0 - 1, pos.1),
            AbsoluteDirection::Right => (pos.0 + 1, pos.1),
            AbsoluteDirection::Up => (pos.0, pos.1 - 1),
            AbsoluteDirection::Down => (pos.0, pos.1 + 1),
        }
    }
}
