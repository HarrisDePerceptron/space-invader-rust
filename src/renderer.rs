use std::io::Write;

use crate::container::{Container, Point};

use anyhow::Result;

use crossterm::{cursor, execute, ExecutableCommand};

use crate::game_buffer::GameBuffer;

pub struct TerminalRenderer {
    window_size: Container,
    prev_window_size: Container,
}

impl TerminalRenderer {
    pub fn new(window_size: Container) -> TerminalRenderer {
        TerminalRenderer::set_size(&window_size).expect("Unable to resize terminal window");
        let prev_size =
            crossterm::terminal::window_size().expect("Unable to get terminal window size");

        let prev_window_size = Container::new(
            Point { x: 0, y: 0 },
            Point {
                x: prev_size.columns as usize,
                y: prev_size.rows as usize,
            },
        );

        TerminalRenderer {
            window_size,
            prev_window_size,
        }
    }

    pub fn clear_screen(&self) -> Result<()> {
        let mut stdout = std::io::stdout();

        print!("\x1B[2J\x1B[1;1H");
        stdout.flush().unwrap();
        Ok(())
    }

    pub fn draw(&self, game_buffer: &GameBuffer) -> Result<()> {
        //self.clear_screen()?;

        let buff = game_buffer.get_buffer();

        for i in 0..game_buffer.get_rows() {
            for j in 0..game_buffer.get_cols() {
                execute!(std::io::stdout(), cursor::MoveTo(j as u16, i as u16))?;
                let item = &buff[i][j];
                std::io::stdout().write_all(item.as_bytes())?;
            }
        }

        Ok(())
    }

    pub fn set_size(container: &Container) -> Result<()> {
        let cols = container.bottom.x - container.top.x;
        let rows = container.bottom.y - container.top.y;

        execute!(
            std::io::stdout(),
            crossterm::terminal::SetSize(cols as u16, rows as u16)
        )?;

        Ok(())
    }
}

impl Drop for TerminalRenderer {
    fn drop(&mut self) {
        TerminalRenderer::set_size(&self.prev_window_size).expect("Unable to resize window");
    }
}
