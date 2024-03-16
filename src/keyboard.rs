use crossterm::{execute, ExecutableCommand};

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};

use crate::audio;
use crate::container::{Container, Point};
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
        let (ship_start, ship_end) = game_buffer.get_ship_boundary();
        let ship_box = game_buffer.get_ship_box();

        if let Ok(v) = self.read_keyboard_event() {
            if let Some(ship_container) = &ship_box {
                let mut ship_position = ship_container.top.x;

                if let KeyCode::Up = v.code {}

                if let KeyCode::Left = v.code {
                    if ship_position > 0 {
                        ship_position -= self.step as usize;
                    }
                }

                if let KeyCode::Right = v.code {
                    ship_position += self.step as usize;
                }

                game_buffer.draw_ship(ship_position);
            }

            if let KeyCode::Char(' ') = v.code {
                if let Some(b) = ship_box {
                    if game_buffer.last_bullet.is_none() {
                        let bullet_start = Point {
                            x: b.top.x + 1,
                            y: b.top.y - 1,
                        };
                        //game_buffer.fire_bullet(bullet_start);
                        game.fire_bullet(bullet_start);

                        if let Err(e) = game_audio.play_fire() {
                            println!("Error in playing audio: {:?}", e);
                        }
                    }
                }
            }

            Some(v)
        } else {
            if let Some(ship_container) = &ship_box {
                game_buffer.draw_ship(ship_container.top.x);
            }
            None
        }
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
