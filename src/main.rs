extern crate termion;

use std::io;
use tic_tac_toe::attr::{set_terminal_attr, TermAttr};
use tic_tac_toe::game::Game;
use tic_tac_toe::tui::Tui;

fn main() -> io::Result<()> {
    let mut tui = Tui::new();
    let mut game = Game::new();
    /* save original terminal attribution */
    let mut termios = TermAttr::get_terminal_attr();
    /* set the new terminal attribution */
    set_terminal_attr((false, false, true, 0, 1))?;

    /* play tic-tac-toe */
    tui.main_screen()?;
    game.update_field(&mut tui)?;
    game.start(&mut tui)?;

    /* restore terminal */
    tui.cursor_show()?;
    tui.reset_srg()?;
    tui.cursor_goto(39, 1)?;
    termios.restore_terminal_attr()
}
