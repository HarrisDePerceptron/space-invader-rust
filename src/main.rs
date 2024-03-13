use anyhow::{anyhow as error, Result};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use crossterm::{
    cursor, execute,
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};
use invader::sized_vector;
use std::io::Write;

pub struct Game {
    score: f32,
    lives: usize,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            score: 0.0,
            lives: 3,
        }
    }
}

pub struct GameBuffer {
    grid: Vec<Vec<String>>,
    rows: usize,
    cols: usize,

    enemy_rows: usize,
    enemy_cols: usize,
    enemy_gap: usize,

    ship_length: usize,
    ship_current_box: Option<Container>,

    // x1, y1, x2, y2
    boundary_coordinates: (usize, usize, usize, usize),

    last_bullet: Option<Bullet>,
}

#[derive(Debug, Clone)]
pub struct Container {
    pub top: Point,
    pub bottom: Point,
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub struct Bullet {
    location: Container,
    speed: usize,

    last_bullet_tick: Option<std::time::Instant>,
    tick_duration: std::time::Duration,
}

impl Bullet {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            location: Container {
                top: Point { x, y },
                bottom: Point { x, y },
            },
            speed: 1,
            tick_duration: std::time::Duration::from_millis(30),
            last_bullet_tick: None,
        }
    }

    pub fn set_tick_duration(&mut self, duration: std::time::Duration) {
        self.tick_duration = duration;
    }

    pub fn get_tick_duration(&self) -> std::time::Duration {
        self.tick_duration
    }

    pub fn set_speed(&mut self, speed: usize) {
        self.speed = speed
    }

    pub fn set_pos(&mut self, x: usize, y: usize) {
        self.location.top.x = x;
        self.location.top.y = y;

        self.location.bottom.x = x;
        self.location.bottom.y = y;
    }

    pub fn get_pos(&self) -> Point {
        self.location.top.clone()
    }

    pub fn get_speed(&self) -> usize {
        self.speed
    }

    fn move_bullet(&mut self) {
        let pos = self.get_pos();
        let new_y = pos.y - self.speed;
        self.set_pos(pos.x, new_y);
    }

    pub fn tick(&mut self) {
        let now = std::time::Instant::now();
        if let Some(t) = &self.last_bullet_tick {
            let diff = now - t.clone();
            if diff >= self.tick_duration {
                self.move_bullet();
                self.last_bullet_tick = Some(now);
            }
        } else {
            self.last_bullet_tick = Some(now);
            return;
        }
    }
}

impl GameBuffer {
    pub fn new(game: &Game) -> GameBuffer {
        let mut game_buffer = GameBuffer {
            grid: vec![],
            rows: 32,
            cols: 64,
            enemy_rows: 5,
            enemy_cols: 11,
            enemy_gap: 2,

            ship_length: 3,
            ship_current_box: None,

            boundary_coordinates: (2, 2, 62, 30),

            last_bullet: None,
        };

        game_buffer.init(game);

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
                c.insert(0, '⍾');
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
        let ship_end = start + self.ship_length;

        for i in start..ship_end {
            row[i].clear();
            row[i].insert(0, '⌬');
        }

        self.ship_current_box = Some(Container {
            top: Point {
                x: start,
                y: ship_y,
            },
            bottom: Point {
                x: ship_end,
                y: ship_y,
            },
        });
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
    fn init(&mut self, game: &Game) {
        self.init_buffer();
        self.clear();

        self.draw_text(game);
        self.draw_boundary();
        let (x1, y1, _, _) = self.boundary_coordinates;

        self.draw_enemy(y1 + 1, 10);
        self.draw_ship(0);
    }

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

    pub fn get_game_boundary(&self) -> (usize, usize, usize, usize) {
        self.boundary_coordinates
    }

    pub fn get_ship_box(&self) -> Option<Container> {
        self.ship_current_box.clone()
    }

    pub fn get_last_bullet(&self) -> Option<Bullet> {
        self.last_bullet.clone()
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

    pub fn draw_text(&mut self, game: &Game) {
        let score_text = format!("Score: {}", game.score);
        let lives_text = format!("Lives: {}", game.lives);

        let score_text_len = score_text.len();
        let lives_text_len = lives_text.len();

        let total_len = score_text_len + lives_text_len;

        if total_len > self.cols {
            panic!(
                "Text length exceeds total buffer cols: {}>{}",
                total_len, self.cols
            );
        }

        let space_len = self.cols - total_len;

        let mut game_display_text = String::new();
        game_display_text += score_text.as_str();

        for _ in 0..space_len {
            game_display_text += " ";
        }

        game_display_text += lives_text.as_str();

        let display_row_index = self.boundary_coordinates.1 - 1;
        let display_row = &mut self.grid[display_row_index];

        assert!(
            game_display_text.len() <= display_row.len(),
            "display text length greater than total buffer cols"
        );

        for (i, ch) in game_display_text.chars().enumerate() {
            let col = &mut display_row[i];
            col.clear();
            col.insert(0, ch);
        }
    }

    pub fn fire_bullet(&mut self, start: Point) {
        if self.last_bullet.is_none() {
            self.last_bullet = Some(Bullet::new(start.x, start.y));
            self.draw_bullet();
        }
    }

    pub fn draw_bullet(&mut self) {
        if let Some(bullet) = &self.last_bullet {
            //let x_prev = bullet.location.top.x;
            let mut y = bullet.location.top.y;

            //if x != x_prev {
            //    return;
            //}
            //

            let mut x = bullet.location.top.x;

            let y_prev = y + bullet.get_speed();

            let bullet = &mut self.grid[y_prev][x];
            bullet.clear();
            bullet.insert(0, ' ');

            let (x1, y1, x2, y2) = self.boundary_coordinates;

            //bullet should remain within game boundaries
            if y <= y1 {
                y = y1 + 1;
            }

            if y >= y2 {
                y = y2 - 2;
            }

            if x <= x1 {
                x = x1 + 1;
            }

            if x >= x2 {
                x = x2 - 1;
            }

            let pixel = &mut self.grid[y][x];
            pixel.clear();
            pixel.insert(0, '⌇');
        }

        //if self.last_bullet.is_none() {
        //    self.last_bullet = Some(Bullet::new(x, y));
        //}
    }

    fn clear_bullet(&mut self) {
        if let Some(bullet) = &self.last_bullet {
            let pos = bullet.get_pos();

            let x = pos.x;
            let y = pos.y;

            let item = &mut self.grid[y + 1][x];

            item.clear();
            item.insert(0, ' ');

            self.last_bullet = None;
        }
    }

    pub fn bullet_progress(&mut self) {
        if let Some(bullet) = &mut self.last_bullet {
            bullet.tick();
        }
        if let Some(bullet) = &self.last_bullet {
            let pos = bullet.get_pos();

            let new_y = pos.y - bullet.get_speed();

            if new_y <= self.boundary_coordinates.1 {
                self.clear_bullet();
            } else {
                self.draw_bullet();
            }
        }
    }

    pub fn collision_detection(&mut self, game: &mut Game) {
        if let Some(bullet) = &self.last_bullet {
            let pos = bullet.get_pos();

            //check for enemy right above
            let new_y = pos.y - bullet.get_speed();

            let item = &mut self.grid[new_y][pos.x];

            if item == "⍾" {
                item.clear();
                item.insert(0, ' ');
                self.clear_bullet();

                game.score += 10.0;
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
        let ship_box = game_buffer.get_ship_box();

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

            if let KeyCode::Char(' ') = v.code {
                if let Some(b) = ship_box {
                    if game_buffer.last_bullet.is_none() {
                        let bullet_start = Point {
                            x: b.top.x + 1,
                            y: b.top.y + 1,
                        };
                        game_buffer.fire_bullet(bullet_start);
                    }
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

    let mut game = Game::default();
    let mut gb = GameBuffer::new(&game);

    let mut key_handler = KeyboardHandler::new();

    loop {
        let key_event = key_handler.handle(&mut gb);

        gb.bullet_progress();
        gb.collision_detection(&mut game);
        gb.draw_text(&game);

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
