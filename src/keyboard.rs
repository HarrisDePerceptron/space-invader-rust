use crossterm::{execute, ExecutableCommand};

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};

use crate::audio;
use crate::container::{Container, Direction, Point};
use crate::game::Game;
use crate::game_buffer::GameBuffer;

use anyhow::{anyhow as error, Result};

pub struct KeyboardHandler {
    wait: u64,
    step: isize,
    ship_position: usize,
}

impl KeyboardHandler {
    pub fn new() -> KeyboardHandler {
        execute!(std::io::stdout(), crossterm::cursor::Hide).unwrap();
        crossterm::terminal::enable_raw_mode().unwrap();

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

    pub fn handle(
        &mut self,
        game_buffer: &mut GameBuffer,
        game_audio: &mut audio::GameAudio,
        game: &mut Game,
    ) -> Option<KeyEvent> {
        if let Ok(v) = self.read_keyboard_event() {
            let ship = game.get_ship();
            let ship_container = ship.get_container();

            if let KeyCode::Char(' ') = v.code {
                //if let Some(b) = ship_box {
                if game_buffer.last_bullet.is_none() {
                    let bullet_start = Point {
                        x: ship_container.top.x + 1,
                        y: ship_container.top.y - 1,
                    };
                    //game_buffer.fire_bullet(bullet_start);
                    game.fire_bullet(bullet_start);

                    if let Err(e) = game_audio.play_fire() {
                        println!("Error in playing audio: {:?}", e);
                    }
                }
                //}
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

impl Drop for KeyboardHandler {
    fn drop(&mut self) {
        execute!(std::io::stdout(), crossterm::cursor::Show).unwrap();
        crossterm::terminal::disable_raw_mode().unwrap();
    }
}
