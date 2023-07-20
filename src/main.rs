extern crate termion;

use std::fs::File;
use std::io::{self, stdin, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::{clear, color, cursor, style};
use tic_tac_toe::attr::{get_terminal_attr, restore_terminal_attr, set_terminal_attr};

#[derive(Copy, Clone, PartialEq)]
enum FieldStatus {
    EMPTY,
    NOUGHT,
    CROSS,
}

struct Game {
    field: [[FieldStatus; 3]; 3], /* game field */
    row: usize,
    col: usize,
}

/* draw box
 * (x1,y1)  w
 *    +-----------+
 *    |           |
 *    |           |
 *  h |           |
 *    |           |
 *    |           |
 *    +-----------+
 */
fn draw_box(tty: &mut File, shape: (u16, u16, u16, u16)) -> io::Result<()> {
    let (x1, y1, h, w) = shape;

    for i in x1..x1 + h {
        write!(tty, "{}", cursor::Goto(y1, i))?;
        if i == x1 {
            /* top left corner */
            write!(tty, "\x1b(0l\x1b(B")?;
            for _ in y1..y1 + w {
                /* horizontal */
                write!(tty, "\x1b(0q\x1b(B")?;
            }
            /* top right corner */
            write!(tty, "\x1b(0k\x1b(B")?;
        } else if i == x1 + h - 1 {
            /* low left corner */
            write!(tty, "\x1b(0m\x1b(B")?;
            for _ in y1..y1 + w {
                /* horizontal */
                write!(tty, "\x1b(0q\x1b(B")?;
            }
            /* low right corner */
            write!(tty, "\x1b(0j\x1b(B")?;
        } else {
            write!(tty, "\x1b(0x\x1b(B")?;
            for _ in y1..y1 + w {
                write!(tty, "\x1b(0 \x1b(B")?;
            }
            write!(tty, "\x1b(0x\x1b(B")?;
        }
    }
    Ok(())
}

/* FIXME: Implement NOUGHT and CROSS cases */
fn draw_char(tty: &mut File, x: u16, y: u16) -> io::Result<()> {
    let mut x = x;
    write!(tty, "{}", cursor::Goto(y, x))?;
    for _ in 0..2 {
        for j in 0..32 {
            if j % 8 == 0 {
                x += 1;
                write!(tty, "{}", cursor::Goto(y, x))?;
            }
            write!(tty, "\x1b(0a\x1b(B")?;
        }
    }
    Ok(())
}

/* draw main screen
 *
 *    (1,1)             32
 *      +----------------------------------+
 *      | (2,3) 12  (2,18)   (2,33)        |
 *      |     +----+   +----+   +----+     |
 *      |  10 |    |   |    |   |    |     |
 *      |     +----+   +----+   +----+     |
 *      |                                  |
 *      | (12,3)    (12,18)  (12,33)       |
 *      |     +----+   +----+   +----+     |
 *   46 |     |    |   |    |   |    |     |
 *      |     +----+   +----+   +----+     |
 *      |                                  |
 *      | (22,3)    (22,18)  (22,33)       |
 *      |     +----+   +----+   +----+     |
 *      |     |    |   |    |   |    |     |
 *      |     +----+   +----+   +----+     |
 *      |                                  |
 *      +----------------------------------+
 */
fn main_screen(tty: &mut File) -> io::Result<()> {
    /* clear all screean */
    write!(tty, "{}", clear::All)?;
    /* set the color of background and foreground */
    write!(tty, "{}", color::Bg(color::Black))?;
    write!(tty, "{}", color::Fg(color::LightBlue))?;
    /* field frame */
    draw_box(tty, (1, 1, 32, 46))?;

    /* first row */
    draw_box(tty, (2, 3, 10, 12))?;
    draw_box(tty, (2, 18, 10, 12))?;
    draw_box(tty, (2, 33, 10, 12))?;

    /* second row */
    draw_box(tty, (12, 3, 10, 12))?;
    draw_box(tty, (12, 18, 10, 12))?;
    draw_box(tty, (12, 33, 10, 12))?;

    /* third row */
    draw_box(tty, (22, 3, 10, 12))?;
    draw_box(tty, (22, 18, 10, 12))?;
    draw_box(tty, (22, 33, 10, 12))?;

    /* top */
    write!(tty, "{}", cursor::Goto(18, 1))?;
    write!(tty, " tic-tac-toe ")?;

    /* information */
    write!(tty, "{}", cursor::Goto(4, 33))?;
    write!(tty, " HELP ")?;
    write!(tty, "{}", cursor::Goto(5, 34))?;
    write!(tty, "Arrow Keys - cursor movement ")?;
    write!(tty, "{}", cursor::Goto(5, 35))?;
    write!(tty, "Enter - put ")?;
    write!(tty, "{}", cursor::Goto(25, 35))?;
    write!(tty, "ESC - quit ")?;
    Ok(())
}

/* FIXME: Implement NOUGHT and CROSS cases */
fn update_field(tty: &mut File, game: &Game) -> io::Result<()> {
    /* draw the all field */
    for i in 0..3 {
        for j in 0..3 {
            /* compute the original point */
            let x = i as u16 * 10 + 2;
            let y = j as u16 * 15 + 6;

            if i == game.row && j == game.col {
                write!(tty, "{}", color::Fg(color::LightCyan))?;
            } else {
                write!(tty, "{}", color::Fg(color::Black))?;
            }

            /* draw the block */
            match game.field[i][j] {
                FieldStatus::EMPTY => draw_char(tty, x, y)?,
                FieldStatus::NOUGHT => {}
                FieldStatus::CROSS => {}
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    /* open /dev/tty */
    let mut tty = termion::get_tty()?;
    /* save original terminal attribution */
    let termios = get_terminal_attr().unwrap();
    set_terminal_attr((false, false, true, 0, 1))?;
    /* hide the cursor */
    write!(tty, "{}", cursor::Hide)?;

    let mut game = Game {
        field: [[FieldStatus::EMPTY; 3]; 3],
        row: 0,
        col: 0,
    };

    /* draw main screen */
    main_screen(&mut tty)?;
    update_field(&mut tty, &game)?;
    for key in stdin().keys() {
        match key.unwrap() {
            Key::Left => game.col -= if game.col != 0 { 1 } else { 0 },
            Key::Right => game.col += if game.col < 2 { 1 } else { 0 },
            Key::Up => game.row -= if game.row != 0 { 1 } else { 0 },
            Key::Down => game.row += if game.row < 2 { 1 } else { 0 },
            /* FIXME: Implement the move of human */
            Key::Char('\n') => continue,
            Key::Esc => break,
            _ => continue,
        }
        update_field(&mut tty, &game)?;
    }

    /* restore terminal attribution */
    restore_terminal_attr(termios)?;
    /* show the cursor */
    write!(tty, "{}", cursor::Show)?;
    /* reset SRG parameter */
    write!(tty, "{}", style::Reset)?;
    write!(tty, "{}", cursor::Goto(1, 39))?;
    Ok(())
}
