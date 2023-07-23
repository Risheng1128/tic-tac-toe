extern crate termion;

use std::fs::File;
use std::io::{self, Write};
use termion::{clear, color, cursor, style};

pub struct Tui {
    tty: File,
}

impl Tui {
    pub fn new() -> Tui {
        let tui = Tui {
            /* open /dev/tty */
            tty: termion::get_tty().unwrap(),
        };
        tui
    }

    /* hide the cursor */
    fn cursor_hide(&mut self) -> io::Result<()> {
        write!(self.tty, "{}", cursor::Hide)
    }

    pub fn cursor_show(&mut self) -> io::Result<()> {
        write!(self.tty, "{}", cursor::Show)
    }

    pub fn cursor_goto(&mut self, x: u16, y: u16) -> io::Result<()> {
        write!(self.tty, "{}", cursor::Goto(y, x))
    }

    /* clear all screean */
    fn clear_screen(&mut self) -> io::Result<()> {
        write!(self.tty, "{}", clear::All)
    }

    /* cover all screen */
    pub fn cover_screen(&mut self) -> io::Result<()> {
        write!(self.tty, "\x1b[H\x1b[J")
    }

    /* set background color */
    pub fn bg_set(&mut self, color: &dyn color::Color) -> io::Result<()> {
        write!(self.tty, "{}", color::Bg(color))
    }

    /* set foreground color */
    pub fn fg_set(&mut self, color: &dyn color::Color) -> io::Result<()> {
        write!(self.tty, "{}", color::Fg(color))
    }

    /* print special character */
    fn print_char(&mut self, char: char) -> io::Result<()> {
        write!(self.tty, "\x1b(0{}\x1b(B", char)
    }

    /* print message */
    pub fn print_msg(&mut self, msg: &str) -> io::Result<()> {
        write!(self.tty, "{:?}", msg)
    }

    /* reset SRG parameter */
    pub fn reset_srg(&mut self) -> io::Result<()> {
        write!(self.tty, "{}", style::Reset)
    }

    /* draw the big character
     *
     * '*': selected block
     * 'o': nought
     * 'x': cross
     */
    pub fn draw_char(&mut self, origin: (u16, u16), cases: char) -> io::Result<()> {
        let bits: [u32; 2] = if cases == 'o' {
            [3284386620, 1019462595]
        } else if cases == 'x' {
            [1013367747, 3284362812]
        } else {
            [0xFFFF_FFFF; 2]
        };

        let (mut x, y) = origin;
        self.cursor_goto(x, y)?;
        for i in 0..2 {
            for j in 0..32 {
                if j % 8 == 0 {
                    x += 1;
                    self.cursor_goto(x, y)?;
                }
                self.print_char(if (bits[i] >> j) & 0x1 == 0 { ' ' } else { 'a' })?;
            }
        }
        Ok(())
    }

    /* draw box
     *
     * (x1,y1)  w
     *    +-----------+
     *    |           |
     *    |           |
     *  h |           |
     *    |           |
     *    |           |
     *    +-----------+
     */
    fn draw_box(&mut self, shape: (u16, u16, u16, u16)) -> io::Result<()> {
        let (x1, y1, h, w) = shape;

        for i in x1..x1 + h {
            self.cursor_goto(i, y1)?;
            if i == x1 {
                /* top left corner */
                self.print_char('l')?;
                for _ in y1..y1 + w {
                    /* horizontal */
                    self.print_char('q')?;
                }
                /* top right corner */
                self.print_char('k')?;
            } else if i == x1 + h - 1 {
                /* low left corner */
                self.print_char('m')?;
                for _ in y1..y1 + w {
                    /* horizontal */
                    self.print_char('q')?;
                }
                /* low right corner */
                self.print_char('j')?;
            } else {
                self.print_char('x')?;
                for _ in y1..y1 + w {
                    self.print_char(' ')?;
                }
                self.print_char('x')?;
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
    pub fn main_screen(&mut self) -> io::Result<()> {
        self.cursor_hide()?;
        self.clear_screen()?;
        /* set foreground and background color */
        self.fg_set(&color::LightBlue)?;
        self.bg_set(&color::Black)?;

        /* field frame */
        self.draw_box((1, 1, 32, 46))?;

        /* first row */
        self.draw_box((2, 3, 10, 12))?;
        self.draw_box((2, 18, 10, 12))?;
        self.draw_box((2, 33, 10, 12))?;

        /* second row */
        self.draw_box((12, 3, 10, 12))?;
        self.draw_box((12, 18, 10, 12))?;
        self.draw_box((12, 33, 10, 12))?;

        /* third row */
        self.draw_box((22, 3, 10, 12))?;
        self.draw_box((22, 18, 10, 12))?;
        self.draw_box((22, 33, 10, 12))?;

        /* top */
        self.cursor_goto(1, 18)?;
        self.print_msg(" tic-tac-toe ")?;

        /* information */
        self.cursor_goto(33, 4)?;
        self.print_msg(" HELP ")?;
        self.cursor_goto(34, 5)?;
        self.print_msg("Arrow Keys - cursor movement ")?;
        self.cursor_goto(35, 5)?;
        self.print_msg("Enter - put ")?;
        self.cursor_goto(35, 25)?;
        self.print_msg("ESC - quit ")
    }
}
