use anyhow::{anyhow as error, Result};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use crossterm::{
    cursor, execute,
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
use invader::sized_vector;
use std::io::Write;

struct Game {
    score: usize,
    lives: usize,
}

pub struct GameBuffer {
    grid: Vec<Vec<String>>,
    rows: usize,
    cols: usize,

    enemy_rows: usize,
    enemy_cols: usize,
    enemy_gap: usize,

    ship_length: usize,

    // x1, y1, x2, y2
    boundary_coordinates: (usize, usize, usize, usize),
}

impl GameBuffer {
    pub fn new() -> GameBuffer {
        let mut game_buffer = GameBuffer {
            grid: vec![],
            rows: 32,
            cols: 64,
            enemy_rows: 5,
            enemy_cols: 11,
            enemy_gap: 2,

            ship_length: 3,
            boundary_coordinates: (2, 2, 62, 30),
        };

        game_buffer.init();

        game_buffer
    }
    pub fn clear(&mut self) {
        for row in &mut self.grid {
            for col in row {
                col.clear();
                col.insert(0, ' ');
            }
        }
    }

    pub fn draw_enemy(&mut self, start_row: usize, start_col: usize) {
        for i in start_row..start_row + (self.enemy_rows * self.enemy_gap) {
            if i % self.enemy_gap != 0 {
                continue;
            }
            for j in start_col..start_col + (self.enemy_cols * self.enemy_gap) {
                if j % self.enemy_gap != 0 {
                    continue;
                }

                let r = &mut self.grid[i];
                let c = &mut r[j];
                c.clear();
                c.insert(0, 'X');
            }
        }
    }

    pub fn draw_ship(&mut self, mut start: usize) {
        let (_, _, x2, y2) = self.boundary_coordinates;

        if start + self.ship_length > self.cols - 1 {
            start = self.cols - 1;
        }

        let ship_y = y2 - 1;

        let row = &mut self.grid[ship_y];
        for c in row.iter_mut() {
            c.clear();
            c.insert(0, ' ');
        }
        for i in start..start + self.ship_length {
            row[i].clear();
            row[i].insert(0, '*');
        }
    }

    fn init_buffer(&mut self) {
        for _ in 0..self.rows {
            let mut row: Vec<String> = vec![];
            for _ in 0..self.cols {
                row.push(" ".into());
            }
            self.grid.push(row);
        }
    }
    fn init(&mut self) {
        self.init_buffer();
        self.clear();

        self.draw_boundary();
        let (x1, y1, _, _) = self.boundary_coordinates;

        self.draw_enemy(y1 + 1, 10);
        self.draw_ship(0);

        //self.draw();
    }

    //fn clear_screen(&self) {
    //    print!("\x1B[2J\x1B[1;1H");
    //    std::io::stdout().flush().unwrap()
    //}
    //pub fn draw(&self) {
    //    self.clear_screen();

    //    for i in &self.grid {
    //        for j in i {
    //            print!("{}", j);
    //        }
    //        println!();
    //        std::io::stdout().flush().unwrap();
    //    }
    //}

    pub fn get_cols(&self) -> usize {
        self.cols
    }

    pub fn get_rows(&self) -> usize {
        self.rows
    }
    pub fn get_buffer(&self) -> &Vec<Vec<String>> {
        &self.grid
    }

    pub fn get_ship_boundary(&self) -> (usize, usize) {
        let start = 0;
        let end = self.get_cols() - 1 - self.ship_length;

        (start, end)
    }

    fn draw_boundary(&mut self) {
        let (x1, y1, x2, y2) = self.boundary_coordinates;

        for i in 0..self.get_rows() {
            for j in 0..self.get_cols() {
                let row = &mut self.grid[i];
                let item = &mut row[j];

                if i == y1 || i == y2 {
                    item.clear();
                    item.insert(0, '-');
                }

                //if i > y1 && (j == x1 || j == x2) {
                //    item.clear();
                //    item.insert(0, '|');
                //}
            }
        }
    }
}

pub struct TerminalRenderer<'a> {
    game_buffer: &'a GameBuffer,
}

impl<'a> TerminalRenderer<'a> {
    pub fn new(game_buffer: &'a GameBuffer) -> TerminalRenderer {
        TerminalRenderer { game_buffer }
    }

    fn clear_screen(&self) -> Result<()> {
        let mut stdout = std::io::stdout();

        crossterm::terminal::disable_raw_mode().unwrap();
        print!("\x1B[2J\x1B[1;1H");
        stdout.flush().unwrap();
        crossterm::terminal::enable_raw_mode().unwrap();
        Ok(())
    }

    pub fn draw(&self) -> Result<()> {
        //self.clear_screen()?;

        let buff = self.game_buffer.get_buffer();

        for i in 0..self.game_buffer.get_rows() {
            for j in 0..self.game_buffer.get_cols() {
                execute!(std::io::stdout(), cursor::MoveTo(j as u16, i as u16))?;
                let item = &buff[i][j];
                std::io::stdout().write_all(item.as_bytes())?;
            }
        }

        Ok(())
    }
}

pub struct KeyboardHandler {
    wait: u64,
    step: isize,
    ship_position: usize,
}

impl KeyboardHandler {
    fn new() -> KeyboardHandler {
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

    fn handle(&mut self, game_buffer: &mut GameBuffer) -> Option<KeyEvent> {
        let (ship_start, ship_end) = game_buffer.get_ship_boundary();

        if let Ok(v) = self.read_keyboard_event() {
            if let KeyCode::Left = v.code {
                if self.ship_position > ship_start {
                    self.ship_position -= self.step as usize;
                }
            }

            if let KeyCode::Right = v.code {
                if self.ship_position < ship_end {
                    self.ship_position += (self.step) as usize;
                }
            }

            game_buffer.draw_ship(self.ship_position);
            Some(v)
        } else {
            game_buffer.draw_ship(self.ship_position);
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
fn main() -> Result<()> {
    println!("Hello, world!");

    let mut gb = GameBuffer::new();

    let mut key_handler = KeyboardHandler::new();

    loop {
        let key_event = key_handler.handle(&mut gb);

        let tr = TerminalRenderer::new(&gb);
        tr.draw()?;

        if let Some(v) = key_event {
            if let KeyCode::Esc = v.code {
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    Ok(())
}
