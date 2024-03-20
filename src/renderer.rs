use std::io::Write;

use crate::{
    container::{Container, Point},
    game::Game,
    text_processing::process_text,
};

use anyhow::Result;

use crossterm::{cursor, execute, ExecutableCommand};

use crate::game_buffer::GameBuffer;

pub struct TerminalRenderer {
    window_container: Container,
    prev_window_size: Container,

    raw_mode: bool,
    cursor: bool,
}

impl TerminalRenderer {
    pub fn new(game: &Game) -> TerminalRenderer {
        let window_container = game.get_window();

        TerminalRenderer::set_size(&window_container).expect("Unable to resize terminal window");
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
            window_container,
            prev_window_size,
            raw_mode: false,
            cursor: true,
        }
    }

    pub fn clear_screen(&mut self) -> Result<()> {
        let raw_mode = self.raw_mode;

        self.disable_raw_mode();
        let mut stdout = std::io::stdout();

        print!("\x1B[2J\x1B[1;1H");
        stdout.flush().unwrap();

        if raw_mode {
            self.enable_raw_mode();
        }
        Ok(())
    }

    pub fn draw(&self, game_buffer: &GameBuffer) -> Result<()> {
        let buff = game_buffer.get_buffer();

        for i in 0..game_buffer.get_rows() {
            for j in 0..game_buffer.get_cols() {
                let item = &buff[i][j];
                Self::goto_write(j, i, item);
            }
        }

        Ok(())
    }

    pub fn goto_write(x: usize, y: usize, s: &str) {
        execute!(std::io::stdout(), cursor::MoveTo(x as u16, y as u16))
            .expect("Unable to  move to location");
        std::io::stdout()
            .write_all(s.as_bytes())
            .expect("Unable to write to location");
    }

    pub fn get_window_container(&self) -> Container {
        self.window_container.clone()
    }

    pub fn set_size(container: &Container) -> Result<()> {
        let cols = container.get_width();
        let rows = container.get_height();

        execute!(
            std::io::stdout(),
            crossterm::terminal::SetSize(cols as u16, rows as u16)
        )?;

        Ok(())
    }

    pub fn draw_gameover(&self) -> Result<()> {
        let window_container = self.get_window_container();

        let buff = process_text(window_container.clone());

        for row in 0..window_container.get_height() {
            for j in 0..window_container.get_width() {
                execute!(std::io::stdout(), cursor::MoveTo(j as u16, row as u16))?;
                let item = &buff[row][j];

                let mut pix = false;
                let threshold = 10u8;
                if item > &threshold {
                    pix = true;
                }

                let mut symbol = "▮";
                if !pix {
                    symbol = " ";
                }

                std::io::stdout().write_all(symbol.as_bytes())?;
            }
        }

        Ok(())
    }
    pub fn draw_gameover1(&self, game: &Game, text: &str) {
        let playable_area = game.get_playablearea();
        let mid_y = playable_area.get_height() / 2;
        let mid_x = playable_area.get_width() / 2;

        let mid_x_quater = mid_x / 2;

        let mut text = text.to_string();
        let spaces = (0..mid_x_quater / 3).map(|_| " ").collect::<String>();

        text = format!("{}{}", spaces, text);

        let output = cfonts::render(cfonts::Options {
            text: text,
            font: cfonts::Fonts::FontTiny,

            ..cfonts::Options::default()
        });

        TerminalRenderer::goto_write(mid_x / 2, mid_y - 1, &output.text);
        TerminalRenderer::goto_write(
            mid_x / 6,
            mid_y + 4,
            "Press any [Enter] to continue or [ESC] to exit",
        );

        TerminalRenderer::goto_write(0, game.get_window().get_height(), "");
    }

    pub fn enable_raw_mode(&mut self) {
        if self.raw_mode {
            return;
        }
        crossterm::terminal::enable_raw_mode().expect("Unable to enable terminal raw mode");
        self.raw_mode = true;
    }

    pub fn disable_raw_mode(&mut self) {
        if !self.raw_mode {
            return;
        }
        crossterm::terminal::disable_raw_mode().expect("Unable to disable terminal raw mode");
        self.raw_mode = false;
    }

    pub fn enable_cursor(&mut self) {
        if self.cursor {
            return;
        }
        execute!(std::io::stdout(), crossterm::cursor::Show).expect("Unable to enable cursor");
        self.cursor = true;
    }

    pub fn disable_cursor(&mut self) {
        if !self.cursor {
            return;
        }
        execute!(std::io::stdout(), crossterm::cursor::Hide).expect("Unable to disable cursor");

        self.cursor = false;
    }
}

//impl Drop for TerminalRenderer {
//    fn drop(&mut self) {
//        TerminalRenderer::set_size(&self.prev_window_size).expect("Unable to resize window");
//    }
//}
