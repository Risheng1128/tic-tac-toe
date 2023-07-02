extern crate termion;

use std::fs::File;
use std::io::{self, Write};
use termion::clear;
use termion::color;
use termion::cursor;

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
fn main_screen() -> io::Result<()> {
    /* open /dev/tty */
    let mut tty = termion::get_tty()?;
    /* clear all screean */
    write!(tty, "{}", clear::All)?;
    /* set the color of background and foreground */
    write!(tty, "{}", color::Bg(color::Black))?;
    write!(tty, "{}", color::Fg(color::LightBlue))?;
    /* field frame */
    draw_box(&mut tty, (1, 1, 32, 46))?;

    /* first row */
    draw_box(&mut tty, (2, 3, 10, 12))?;
    draw_box(&mut tty, (2, 18, 10, 12))?;
    draw_box(&mut tty, (2, 33, 10, 12))?;

    /* second row */
    draw_box(&mut tty, (12, 3, 10, 12))?;
    draw_box(&mut tty, (12, 18, 10, 12))?;
    draw_box(&mut tty, (12, 33, 10, 12))?;

    /* third row */
    draw_box(&mut tty, (22, 3, 10, 12))?;
    draw_box(&mut tty, (22, 18, 10, 12))?;
    draw_box(&mut tty, (22, 33, 10, 12))?;

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

fn main() -> io::Result<()> {
    main_screen()
}
