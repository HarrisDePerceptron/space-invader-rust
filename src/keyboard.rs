use crossterm::{execute, ExecutableCommand};

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};

use crate::audio;
use crate::container::{Container, Direction, Point};
use crate::game::Game;
use crate::game_buffer::GameBuffer;
use crate::renderer::TerminalRenderer;

use anyhow::{anyhow as error, Result};

pub struct KeyboardHandler {
    wait: u64,
    step: isize,
    ship_position: usize,
}

impl KeyboardHandler {
    pub fn new(terminal: &mut TerminalRenderer) -> KeyboardHandler {
        terminal.disable_cursor();
        terminal.enable_raw_mode();

        KeyboardHandler {
            wait: 1,
            step: 1,
            ship_position: 0,
        }
    }

    pub fn set_wait(&mut self, wait: u64) {
        self.wait = wait
    }

    pub fn set_step(&mut self, step: isize) {
        self.step = step
    }

    pub fn handle(&mut self, game: &mut Game) -> Option<KeyEvent> {
        if let Ok(v) = self.read_keyboard_event() {
            if let KeyCode::Char(' ') = v.code {
                game.fire_bullet();
            }

            if let KeyCode::Left = v.code {
                game.move_ship(Direction::LEFT);
            }

            if let KeyCode::Right = v.code {
                game.move_ship(Direction::RIGHT);
            }

            return Some(v);
        }

        None
    }
    fn read_keyboard_event(&self) -> Result<crossterm::event::KeyEvent> {
        if poll(std::time::Duration::from_millis(self.wait))? {
            if let Event::Key(ke) = read()? {
                return Ok(ke);
            }
        }

        Err(error!("Key event not found"))
    }
}

//impl Drop for KeyboardHandler {
//    fn drop(&mut self) {
//        execute!(std::io::stdout(), crossterm::cursor::Show).unwrap();
//        crossterm::terminal::disable_raw_mode().unwrap();
//    }
//}
