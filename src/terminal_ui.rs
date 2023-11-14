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

const TERMINAL_UI_PADDING_HORIZONTAL: u16 = 1;
const TERMINAL_UI_PADDING_VERTICAL: u16 = 1;

pub fn intialize_terminal_ui<W: Write>(screen: &mut W) {
    write!(screen, "{}", termion::clear::All).unwrap();
    draw_border(screen);
    screen.flush().unwrap();
}

fn draw_border<W: Write>(screen: &mut W) {
    let (width, height) = termion::terminal_size().unwrap();
    // We will first draw the complete box and afterwards print the
    // application name ontop of the box.
    write!(
        screen,
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{} {} {}",
        termion::cursor::Goto(
            TERMINAL_UI_PADDING_HORIZONTAL + 1,
            TERMINAL_UI_PADDING_VERTICAL + 1
        ),
        TERMINAL_UI_BORDER_CORNER_TOP_LEFT,
        TERMINAL_UI_BORDER_TOP
            .to_string()
            .repeat(width.saturating_sub(2 * (TERMINAL_UI_PADDING_HORIZONTAL + 1)) as usize),
        TERMINAL_UI_BORDER_CORNER_TOP_RIGHT,
        termion::cursor::Goto(
            TERMINAL_UI_PADDING_HORIZONTAL + 1,
            height - TERMINAL_UI_PADDING_VERTICAL
        ),
        TERMINAL_UI_BORDER_CORNER_BOTTOM_LEFT,
        TERMINAL_UI_BORDER_BOTTOM
            .to_string()
            .repeat(width.saturating_sub(2 * (TERMINAL_UI_PADDING_HORIZONTAL + 1)) as usize),
        TERMINAL_UI_BORDER_CORNER_BOTTOM_RIGHT,
        termion::cursor::Goto(
            TERMINAL_UI_PADDING_HORIZONTAL + 1,
            TERMINAL_UI_PADDING_VERTICAL + 2
        ),
        format!(
            "{}{}{}",
            TERMINAL_UI_BORDER_LEFT,
            termion::cursor::Left(1),
            termion::cursor::Down(1)
        )
        .repeat(height.saturating_sub(2 * (TERMINAL_UI_PADDING_VERTICAL + 1)) as usize),
        termion::cursor::Goto(
            width - TERMINAL_UI_PADDING_HORIZONTAL,
            TERMINAL_UI_PADDING_VERTICAL + 2
        ),
        format!(
            "{}{}{}",
            TERMINAL_UI_BORDER_RIGHT,
            termion::cursor::Left(1),
            termion::cursor::Down(1)
        )
        .repeat(height.saturating_sub(2 * (TERMINAL_UI_PADDING_VERTICAL + 1)) as usize),
        termion::cursor::Goto(
            TERMINAL_UI_PADDING_HORIZONTAL + TERMINAL_UI_APPLICATION_NAME_X_POS - 1,
            TERMINAL_UI_PADDING_VERTICAL + 1
        ),
        TERMINAL_UI_NAME_BORDER_LEFT,
        TERMINAL_UI_APPLICATION_NAME,
        TERMINAL_UI_NAME_BORDER_RIGHT
    )
    .unwrap();
}
