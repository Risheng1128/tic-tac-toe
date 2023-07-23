extern crate termios;

use std::io::{self, stdin};
use std::os::fd::AsRawFd;
use termios::{tcflag_t, tcgetattr, tcsetattr, Termios};
use termios::{ECHO, ICANON, ISIG, VMIN, VTIME};

pub struct TermAttr {
    pub attr: Termios,
}

impl TermAttr {
    pub fn get_terminal_attr() -> TermAttr {
        let mut termios: TermAttr = unsafe { std::mem::zeroed() };
        tcgetattr(stdin().as_raw_fd(), &mut termios.attr).unwrap();
        termios
    }

    pub fn restore_terminal_attr(&mut self) -> io::Result<()> {
        tcsetattr(stdin().as_raw_fd(), termios::TCSANOW, &self.attr)
    }
}

/* set the terminal attribution
 * reference: https://man7.org/linux/man-pages/man3/termios.3.html
 */
pub fn set_terminal_attr(cfg: (bool, bool, bool, u8, u8)) -> io::Result<()> {
    let (canon, echo, sigint, vtime, vmin) = cfg;
    let mut termios = TermAttr::get_terminal_attr();

    /* clear original terminal attribution */
    termios.attr.c_lflag &= !(ICANON | ECHO | ISIG);

    /* set new terminal attribution */
    let mut val: tcflag_t = 0;
    val |= if canon { ICANON } else { 0 };
    val |= if echo { ECHO } else { 0 };
    val |= if sigint { ISIG } else { 0 };
    termios.attr.c_lflag |= val;
    termios.attr.c_cc[VTIME] = vtime;
    termios.attr.c_cc[VMIN] = vmin;
    tcsetattr(io::stdin().as_raw_fd(), termios::TCSANOW, &termios.attr)
}
