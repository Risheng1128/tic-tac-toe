extern crate termion;

use crate::tui::Tui;
use std::io::{self, stdin};
use termion::{color, event::Key, input::TermRead};

#[derive(Copy, Clone, PartialEq)]
enum FieldStatus {
    EMPTY,
    NOUGHT,
    CROSS,
}

#[derive(PartialEq)]
enum GameResult {
    EMPTY,
    NOUGHT,
    CROSS,
    DRAW,
}

pub struct Game {
    field: [[FieldStatus; 3]; 3], /* game field */
    row: usize,
    col: usize,
}

impl Game {
    pub fn new() -> Game {
        let game = Game {
            field: [[FieldStatus::EMPTY; 3]; 3],
            row: 0,
            col: 0,
        };
        game
    }

    pub fn start(&mut self, tui: &mut Tui) -> io::Result<()> {
        for key in stdin().keys() {
            match key.unwrap() {
                Key::Left => self.col -= if self.col != 0 { 1 } else { 0 },
                Key::Right => self.col += if self.col < 2 { 1 } else { 0 },
                Key::Up => self.row -= if self.row != 0 { 1 } else { 0 },
                Key::Down => self.row += if self.row < 2 { 1 } else { 0 },
                Key::Char('\n') => {
                    if self.human_move(FieldStatus::NOUGHT) {
                        let result = self.check_win();
                        if result != GameResult::EMPTY {
                            self.game_over(tui, result)?;
                            break;
                        }

                        self.computer_move(FieldStatus::CROSS);
                        let result = self.check_win();
                        if result != GameResult::EMPTY {
                            self.game_over(tui, result)?;
                            break;
                        }
                    }
                }
                Key::Esc => break,
                _ => continue,
            }
            self.update_field(tui)?;
        }
        Ok(())
    }

    pub fn update_field(&self, tui: &mut Tui) -> io::Result<()> {
        /* draw the all field */
        for i in 0..3 {
            for j in 0..3 {
                /* compute the original point */
                let x = i as u16 * 10 + 2;
                let y = j as u16 * 15 + 6;

                if i == self.row && j == self.col {
                    tui.fg_set(&color::LightCyan)?;
                    tui.bg_set(&color::LightBlack)?;
                } else {
                    if self.field[i][j] == FieldStatus::EMPTY {
                        tui.fg_set(&color::Black)?;
                    } else {
                        tui.fg_set(&color::LightCyan)?;
                    }
                    tui.bg_set(&color::Black)?;
                }

                /* draw the block */
                match self.field[i][j] {
                    FieldStatus::EMPTY => tui.draw_char((x, y), '*')?,
                    FieldStatus::NOUGHT => tui.draw_char((x, y), 'o')?,
                    FieldStatus::CROSS => tui.draw_char((x, y), 'x')?,
                }
            }
        }
        Ok(())
    }

    fn human_move(&mut self, status: FieldStatus) -> bool {
        if self.field[self.row][self.col] == FieldStatus::EMPTY {
            self.field[self.row][self.col] = status;
            return true;
        }
        false
    }

    fn computer_move(&mut self, status: FieldStatus) {
        'row: for i in 0..3 {
            for j in 0..3 {
                if self.field[i][j] == FieldStatus::EMPTY {
                    self.field[i][j] = status;
                    break 'row;
                }
            }
        }
    }

    fn check_win(&self) -> GameResult {
        /*             case7               case8
         *                 \   c4   c5  c6   /
         *                  \  ||   ||  ||  /
         *                   \ \/   \/  \/ /
         *   case 1 ----->    | x | o | x |
         *   case 2 ----->    | o | x | o |
         *   case 3 ----->    | o | x | x |
         */
        for i in 0..3 {
            /* cases 1 2 3 */
            if self.field[i][0] == FieldStatus::NOUGHT
                && self.field[i][1] == FieldStatus::NOUGHT
                && self.field[i][2] == FieldStatus::NOUGHT
            {
                return GameResult::NOUGHT;
            }
            if self.field[i][0] == FieldStatus::CROSS
                && self.field[i][1] == FieldStatus::CROSS
                && self.field[i][2] == FieldStatus::CROSS
            {
                return GameResult::CROSS;
            }

            /* case 4 5 6 */
            if self.field[0][i] == FieldStatus::NOUGHT
                && self.field[1][i] == FieldStatus::NOUGHT
                && self.field[2][i] == FieldStatus::NOUGHT
            {
                return GameResult::NOUGHT;
            }
            if self.field[0][i] == FieldStatus::CROSS
                && self.field[1][i] == FieldStatus::CROSS
                && self.field[2][i] == FieldStatus::CROSS
            {
                return GameResult::CROSS;
            }
        }

        /* cases 7 8 */
        if self.field[0][0] == FieldStatus::NOUGHT
            && self.field[1][1] == FieldStatus::NOUGHT
            && self.field[2][2] == FieldStatus::NOUGHT
        {
            return GameResult::NOUGHT;
        }
        if self.field[0][0] == FieldStatus::CROSS
            && self.field[1][1] == FieldStatus::CROSS
            && self.field[2][2] == FieldStatus::CROSS
        {
            return GameResult::CROSS;
        }

        if self.field[0][2] == FieldStatus::NOUGHT
            && self.field[1][1] == FieldStatus::NOUGHT
            && self.field[2][0] == FieldStatus::NOUGHT
        {
            return GameResult::NOUGHT;
        }
        if self.field[0][2] == FieldStatus::CROSS
            && self.field[1][1] == FieldStatus::CROSS
            && self.field[2][0] == FieldStatus::CROSS
        {
            return GameResult::CROSS;
        }

        /* draw */
        if self.field[0][0] != FieldStatus::EMPTY
            && self.field[0][1] != FieldStatus::EMPTY
            && self.field[0][2] != FieldStatus::EMPTY
            && self.field[1][0] != FieldStatus::EMPTY
            && self.field[1][1] != FieldStatus::EMPTY
            && self.field[1][2] != FieldStatus::EMPTY
            && self.field[2][0] != FieldStatus::EMPTY
            && self.field[2][1] != FieldStatus::EMPTY
            && self.field[2][2] != FieldStatus::EMPTY
        {
            return GameResult::DRAW;
        }

        GameResult::EMPTY
    }

    fn game_over(&self, tui: &mut Tui, result: GameResult) -> io::Result<()> {
        tui.fg_set(&color::LightCyan)?;
        tui.bg_set(&color::Black)?;
        tui.cover_screen()?;
        tui.cursor_goto(15, 17)?;

        match result {
            GameResult::NOUGHT => tui.print_msg("Nought Win!"),
            GameResult::CROSS => tui.print_msg("Cross Win!"),
            GameResult::DRAW => tui.print_msg("Draw!"),
            GameResult::EMPTY => Ok(()),
        }
    }
}
