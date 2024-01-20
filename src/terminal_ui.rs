use std::io::Write;

const TERMINAL_UI_BORDER_CORNER_TOP_LEFT: char = '╔';
const TERMINAL_UI_BORDER_CORNER_TOP_RIGHT: char = '╗';
const TERMINAL_UI_BORDER_CORNER_BOTTOM_LEFT: char = '╚';
const TERMINAL_UI_BORDER_CORNER_BOTTOM_RIGHT: char = '╝';
const TERMINAL_UI_BORDER_TOP: char = '═';
const TERMINAL_UI_BORDER_BOTTOM: char = '═';
const TERMINAL_UI_BORDER_LEFT: char = '║';
const TERMINAL_UI_BORDER_RIGHT: char = '║';
const TERMINAL_UI_NAME_BORDER_LEFT: char = '╣';
const TERMINAL_UI_NAME_BORDER_RIGHT: char = '╠';
const TERMINAL_UI_APPLICATION_NAME: &str = "MAZES";
const TERMINAL_UI_APPLICATION_NAME_X_POS: u16 = 8;

// Padding from the terminal edges to the border of the UI.
const TERMINAL_UI_PADDING_OUTSIDE_HORIZONTAL: u16 = 1;
const TERMINAL_UI_PADDING_OUTSIDE_VERTICAL: u16 = 4;
// Min. padding from the border of the UI to the maze.
const TERMINAL_UI_PADDING_INSIDE_HORIZONTAL: u16 = 1;
const TERMINAL_UI_PADDING_INSIDE_VERTICAL: u16 = 1;

pub const TERMINAL_WIDTH_MIN: u16 = 50;
pub const TERMINAL_HEIGHT_MIN: u16 = 40;

pub fn intialize_terminal_ui(screen: &mut dyn Write) {
    write!(screen, "{}", termion::clear::All).unwrap();
    draw_border(screen);
    screen.flush().unwrap();
}

pub fn get_max_draw_size() -> (usize, usize) {
    let (mut max_width, mut max_height) = termion::terminal_size().unwrap();
    max_width -=
        2 * (TERMINAL_UI_PADDING_INSIDE_HORIZONTAL + TERMINAL_UI_PADDING_OUTSIDE_HORIZONTAL + 1);
    max_height -=
        2 * (TERMINAL_UI_PADDING_INSIDE_VERTICAL + TERMINAL_UI_PADDING_OUTSIDE_VERTICAL + 1);
    (max_width as usize, max_height as usize)
}

// Erases only the area where a maze can be drawn.
pub fn erase_draw_area(screen: &mut dyn Write) {
    let (width, height) = termion::terminal_size().unwrap();
    write!(
        screen,
        "{}{}",
        termion::cursor::Goto(
            TERMINAL_UI_PADDING_OUTSIDE_HORIZONTAL + TERMINAL_UI_PADDING_INSIDE_HORIZONTAL + 2,
            TERMINAL_UI_PADDING_OUTSIDE_VERTICAL + TERMINAL_UI_PADDING_INSIDE_VERTICAL + 2
        ),
        format!(
            "{}{}{}",
            " ".repeat(
                (width
                    - 2 * (TERMINAL_UI_PADDING_INSIDE_HORIZONTAL
                        + TERMINAL_UI_PADDING_OUTSIDE_HORIZONTAL
                        + 1)) as usize
            ),
            termion::cursor::Left(
                width
                    - 2 * (TERMINAL_UI_PADDING_INSIDE_HORIZONTAL
                        + TERMINAL_UI_PADDING_OUTSIDE_HORIZONTAL
                        + 1)
            ),
            termion::cursor::Down(1)
        )
        .repeat(
            (height
                - 2 * (TERMINAL_UI_PADDING_INSIDE_VERTICAL
                    + TERMINAL_UI_PADDING_OUTSIDE_VERTICAL
                    + 1)) as usize
        )
    )
    .unwrap();
    screen.flush().unwrap();
}

fn draw_border(screen: &mut dyn Write) {
    let (width, height) = termion::terminal_size().unwrap();
    // We will first draw the complete box and afterwards print the
    // application name ontop of the box.
    write!(
        screen,
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{} {} {}",
        termion::cursor::Goto(
            TERMINAL_UI_PADDING_OUTSIDE_HORIZONTAL + 1,
            TERMINAL_UI_PADDING_OUTSIDE_VERTICAL + 1
        ),
        TERMINAL_UI_BORDER_CORNER_TOP_LEFT,
        TERMINAL_UI_BORDER_TOP.to_string().repeat(
            width.saturating_sub(2 * (TERMINAL_UI_PADDING_OUTSIDE_HORIZONTAL + 1)) as usize
        ),
        TERMINAL_UI_BORDER_CORNER_TOP_RIGHT,
        termion::cursor::Goto(
            TERMINAL_UI_PADDING_OUTSIDE_HORIZONTAL + 1,
            height - TERMINAL_UI_PADDING_OUTSIDE_VERTICAL
        ),
        TERMINAL_UI_BORDER_CORNER_BOTTOM_LEFT,
        TERMINAL_UI_BORDER_BOTTOM.to_string().repeat(
            width.saturating_sub(2 * (TERMINAL_UI_PADDING_OUTSIDE_HORIZONTAL + 1)) as usize
        ),
        TERMINAL_UI_BORDER_CORNER_BOTTOM_RIGHT,
        termion::cursor::Goto(
            TERMINAL_UI_PADDING_OUTSIDE_HORIZONTAL + 1,
            TERMINAL_UI_PADDING_OUTSIDE_VERTICAL + 2
        ),
        format!(
            "{}{}{}",
            TERMINAL_UI_BORDER_LEFT,
            termion::cursor::Left(1),
            termion::cursor::Down(1)
        )
        .repeat(height.saturating_sub(2 * (TERMINAL_UI_PADDING_OUTSIDE_VERTICAL + 1)) as usize),
        termion::cursor::Goto(
            width - TERMINAL_UI_PADDING_OUTSIDE_HORIZONTAL,
            TERMINAL_UI_PADDING_OUTSIDE_VERTICAL + 2
        ),
        format!(
            "{}{}{}",
            TERMINAL_UI_BORDER_RIGHT,
            termion::cursor::Left(1),
            termion::cursor::Down(1)
        )
        .repeat(height.saturating_sub(2 * (TERMINAL_UI_PADDING_OUTSIDE_VERTICAL + 1)) as usize),
        termion::cursor::Goto(
            TERMINAL_UI_PADDING_OUTSIDE_HORIZONTAL + TERMINAL_UI_APPLICATION_NAME_X_POS - 1,
            TERMINAL_UI_PADDING_OUTSIDE_VERTICAL + 1
        ),
        TERMINAL_UI_NAME_BORDER_LEFT,
        TERMINAL_UI_APPLICATION_NAME,
        TERMINAL_UI_NAME_BORDER_RIGHT
    )
    .unwrap();
}

pub fn print_informations(
    screen: &mut dyn Write,
    generation_algorithm: &str,
    solving_algorithm: &str,
    number_of_inspected_cells: usize,
    animate: bool,
) {
    let (width, _) = termion::terminal_size().unwrap();
    write!(
        screen,
        "{}{}{}generator: {}, solver: {}, insp. cells: {}, animate: {}",
        termion::cursor::Goto(1, 1),
        " ".repeat(width as usize),
        termion::cursor::Goto(1, 1),
        generation_algorithm,
        solving_algorithm,
        number_of_inspected_cells,
        if animate { "ON" } else { "OFF" }
    )
    .unwrap();
    screen.flush().unwrap();
}

pub fn print_solving_sequence(screen: &mut dyn Write, solving_sequence: String) {
    let (width, height) = termion::terminal_size().unwrap();
    write!(
        screen,
        "{}{}{}{}",
        termion::cursor::Goto(1, height - TERMINAL_UI_PADDING_OUTSIDE_VERTICAL + 1),
        " ".repeat((width * TERMINAL_UI_PADDING_OUTSIDE_VERTICAL) as usize),
        termion::cursor::Goto(1, height - TERMINAL_UI_PADDING_OUTSIDE_VERTICAL + 1),
        solving_sequence
    )
    .unwrap();
    screen.flush().unwrap();
}
