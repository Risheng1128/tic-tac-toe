extern crate termion;

use crate::tui::Tui;
use std::io::{self, stdin};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use termion::{color, event::Key, input::TermRead};

#[derive(PartialEq)]
enum Player {
    Computer,
    Human,
}

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

struct GameInfo {
    field: [[FieldStatus; 3]; 3], /* game field */
    row: usize,
    col: usize,
    player: Player,
    winner: GameResult,
    force_over: bool,
}

impl GameInfo {
    fn new() -> GameInfo {
        GameInfo {
            field: [[FieldStatus::EMPTY; 3]; 3],
            row: 0,
            col: 0,
            player: Player::Human,
            winner: GameResult::EMPTY,
            force_over: false,
        }
    }

    /* operate key from keyboard */
    fn operate_key(&mut self, tui: &mut Tui) -> io::Result<()> {
        self.update_field(tui)?;
        for key in stdin().keys() {
            match key.unwrap() {
                Key::Left => self.col -= if self.col != 0 { 1 } else { 0 },
                Key::Right => self.col += if self.col < 2 { 1 } else { 0 },
                Key::Up => self.row -= if self.row != 0 { 1 } else { 0 },
                Key::Down => self.row += if self.row < 2 { 1 } else { 0 },
                Key::Char('\n') => {
                    if self.human_move(FieldStatus::NOUGHT) {
                        break;
                    }
                }
                Key::Esc => {
                    self.force_over = true;
                    break;
                }
                _ => continue,
            }
            self.update_field(tui)?;
        }
        Ok(())
    }

    fn update_field(&self, tui: &mut Tui) -> io::Result<()> {
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

    fn check_win(&mut self) -> bool {
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
                self.winner = GameResult::NOUGHT;
                return true;
            }
            if self.field[i][0] == FieldStatus::CROSS
                && self.field[i][1] == FieldStatus::CROSS
                && self.field[i][2] == FieldStatus::CROSS
            {
                self.winner = GameResult::CROSS;
                return true;
            }

            /* case 4 5 6 */
            if self.field[0][i] == FieldStatus::NOUGHT
                && self.field[1][i] == FieldStatus::NOUGHT
                && self.field[2][i] == FieldStatus::NOUGHT
            {
                self.winner = GameResult::NOUGHT;
                return true;
            }
            if self.field[0][i] == FieldStatus::CROSS
                && self.field[1][i] == FieldStatus::CROSS
                && self.field[2][i] == FieldStatus::CROSS
            {
                self.winner = GameResult::CROSS;
                return true;
            }
        }

        /* cases 7 8 */
        if self.field[0][0] == FieldStatus::NOUGHT
            && self.field[1][1] == FieldStatus::NOUGHT
            && self.field[2][2] == FieldStatus::NOUGHT
        {
            self.winner = GameResult::NOUGHT;
            return true;
        }
        if self.field[0][0] == FieldStatus::CROSS
            && self.field[1][1] == FieldStatus::CROSS
            && self.field[2][2] == FieldStatus::CROSS
        {
            self.winner = GameResult::CROSS;
            return true;
        }

        if self.field[0][2] == FieldStatus::NOUGHT
            && self.field[1][1] == FieldStatus::NOUGHT
            && self.field[2][0] == FieldStatus::NOUGHT
        {
            self.winner = GameResult::NOUGHT;
            return true;
        }
        if self.field[0][2] == FieldStatus::CROSS
            && self.field[1][1] == FieldStatus::CROSS
            && self.field[2][0] == FieldStatus::CROSS
        {
            self.winner = GameResult::CROSS;
            return true;
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
            self.winner = GameResult::DRAW;
            return true;
        }

        false
    }

    fn game_over(&self, tui: &mut Tui) -> io::Result<()> {
        tui.fg_set(&color::LightCyan)?;
        tui.bg_set(&color::Black)?;
        tui.cover_screen()?;
        tui.cursor_goto(15, 17)?;

        match self.winner {
            GameResult::NOUGHT => tui.print_msg("Nought Win!"),
            GameResult::CROSS => tui.print_msg("Cross Win!"),
            GameResult::DRAW => tui.print_msg("Draw!"),
            GameResult::EMPTY => Ok(()),
        }
    }
}

pub struct Game {
    gameinfo: Arc<(Mutex<GameInfo>, Condvar)>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            gameinfo: Arc::new((Mutex::new(GameInfo::new()), Condvar::new())),
        }
    }

    pub fn start(&mut self, tui: &mut Tui) -> io::Result<()> {
        /* create computer thread */
        let shared = self.gameinfo.clone();
        thread::spawn(move || {
            let (guard, condvar) = &*shared;

            loop {
                let mut guard = guard.lock().unwrap();

                /* wait for human move */
                while guard.player == Player::Human {
                    guard = condvar.wait(guard).unwrap();
                }

                guard.computer_move(FieldStatus::CROSS);
                guard.player = Player::Human;
                condvar.notify_one();
            }
        });

        let (guard, condvar) = &*self.gameinfo;
        let mut guard = guard.lock().unwrap();
        loop {
            /* wait for computer move */
            while guard.player == Player::Computer {
                guard = condvar.wait(guard).unwrap();
            }

            /* 1. check the result from computer thread
             * 2. operate key from keyboard
             * 3. force over the game
             * 4. check the result from human thread
             */
            if guard.check_win()
                || guard.operate_key(tui).is_err()
                || guard.force_over
                || guard.check_win()
            {
                break;
            }

            guard.player = Player::Computer;
            condvar.notify_one();
        }
        guard.game_over(tui)
    }
}
