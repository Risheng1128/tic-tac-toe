extern crate termios;

use std::io;
use std::os::fd::AsRawFd;
use termios::{tcflag_t, tcgetattr, tcsetattr, Termios};
use termios::{ECHO, ICANON, ISIG, VMIN, VTIME};

pub fn get_terminal_attr() -> io::Result<Termios> {
    let mut termios: Termios = unsafe { std::mem::zeroed() };
    tcgetattr(io::stdin().as_raw_fd(), &mut termios)?;
    Ok(termios)
}

/* set the terminal attribution
 * reference: https://man7.org/linux/man-pages/man3/termios.3.html
 */
pub fn set_terminal_attr(cfg: (bool, bool, bool, u8, u8)) -> io::Result<()> {
    let (canon, echo, sigint, vtime, vmin) = cfg;
    let mut termios = get_terminal_attr()?;

    /* clear original terminal attribution */
    termios.c_lflag &= !(ICANON | ECHO | ISIG);

    /* set new terminal attribution */
    let mut val: tcflag_t = 0;
    val |= if canon { ICANON } else { 0 };
    val |= if echo { ECHO } else { 0 };
    val |= if sigint { ISIG } else { 0 };
    termios.c_lflag |= val;
    termios.c_cc[VTIME] = vtime;
    termios.c_cc[VMIN] = vmin;
    tcsetattr(io::stdin().as_raw_fd(), termios::TCSANOW, &termios)?;

    Ok(())
}

pub fn restore_terminal_attr(termios: Termios) -> io::Result<()> {
    tcsetattr(io::stdin().as_raw_fd(), termios::TCSANOW, &termios)
}
