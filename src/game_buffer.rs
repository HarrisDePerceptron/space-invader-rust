use crate::container::{Container, Direction, Point};

use crate::game::Game;
use crate::weapon::Bullet;

use crate::audio;

pub struct GameBuffer {
    pub grid: Vec<Vec<String>>,
    pub rows: usize,
    pub cols: usize,

    pub enemy_rows: usize,
    pub enemy_cols: usize,
    pub enemy_gap: usize,

    pub ship_length: usize,
    pub ship_current_box: Option<Container>,

    // x1, y1, x2, y2
    pub boundary_coordinates: (usize, usize, usize, usize),

    pub last_bullet: Option<Bullet>,
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

    //pub fn draw_enemy(&mut self, start_row: usize, start_col: usize) {
    //    for i in start_row..start_row + (self.enemy_rows * self.enemy_gap) {
    //        if i % self.enemy_gap != 0 {
    //            continue;
    //        }
    //        for j in start_col..start_col + (self.enemy_cols * self.enemy_gap) {
    //            if j % self.enemy_gap != 0 {
    //                continue;
    //            }

    //            let r = &mut self.grid[i];
    //            let c = &mut r[j];
    //            c.clear();
    //            c.insert(0, '⍾');
    //        }
    //    }
    //}

    pub fn draw_ship(&mut self, mut start: usize) {
        let (x1, _, x2, y2) = self.boundary_coordinates;

        if start + self.ship_length > self.cols - 1 {
            start = self.cols - 1 - self.ship_length;
        }

        if start <= x1 {
            start = x1 + 1;
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

        self.ship_current_box = Some(Container::new(
            Point {
                x: start,
                y: ship_y,
            },
            Point {
                x: ship_end,
                y: ship_y,
            },
        ));
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

        //self.draw_enemy(y1 + 1, 10);
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
            }
        }
    }

    pub fn draw_text(&mut self, game: &Game) {
        let score_text = format!("Score: {}", game.get_score());
        let lives_text = format!("Lives: {}", game.get_lives());

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

    pub fn draw(&mut self, game: &Game) {
        for e in game.get_enemies() {
            let pos = e.get_pos();
            let symbol = e.get_symbol();

            self.grid[pos.y][pos.x] = symbol.to_string();
        }

        if let Some(bullet) = &self.last_bullet {
            let pos = bullet.get_pos();
            let symbol = bullet.get_symbol();

            self.grid[pos.y][pos.x] = symbol.to_string();
        }
    }

    //pub fn fire_bullet(&mut self, start: Point) {
    //    if self.last_bullet.is_none() {
    //        self.last_bullet = Some(Bullet::new(start.x, start.y, Direction::UP));
    //        self.draw_bullet();
    //    }
    //}

    //pub fn draw_bullet(&mut self) {
    //    if let Some(bullet) = &self.last_bullet {
    //        //let x_prev = bullet.location.top.x;
    //        let mut y = bullet.location.top.y;

    //        let mut x = bullet.location.top.x;

    //        let y_prev = y + bullet.get_speed();

    //        let bullet = &mut self.grid[y_prev][x];
    //        bullet.clear();
    //        bullet.insert(0, ' ');

    //        let (x1, y1, x2, y2) = self.boundary_coordinates;

    //        //bullet should remain within game boundaries
    //        if y <= y1 {
    //            y = y1 + 1;
    //        }

    //        if y >= y2 {
    //            y = y2 - 2;
    //        }

    //        if x <= x1 {
    //            x = x1 + 1;
    //        }

    //        if x >= x2 {
    //            x = x2 - 1;
    //        }

    //        let pixel = &mut self.grid[y][x];
    //        pixel.clear();
    //        pixel.insert(0, '⌇');
    //    }
    //}

    //fn clear_bullet(&mut self) {
    //    if let Some(bullet) = &self.last_bullet {
    //        let pos = bullet.get_pos();

    //        let x = pos.x;
    //        let y = pos.y;

    //        for i in 0..self.get_rows() {
    //            let item = &mut self.grid[i][x];
    //            if item == "⌇" {
    //                item.clear();
    //                item.insert(0, ' ');
    //            }
    //        }

    //        self.last_bullet = None;
    //    }
    //}

    //pub fn bullet_progress(&mut self) {
    //    if let Some(bullet) = &mut self.last_bullet {
    //        bullet.move_tick();
    //    }
    //    if let Some(bullet) = &self.last_bullet {
    //        let pos = bullet.get_pos();

    //        let new_y = pos.y - bullet.get_speed();

    //        if new_y <= self.boundary_coordinates.1 {
    //            self.clear_bullet();
    //        } else {
    //            self.draw_bullet();
    //        }
    //    }
    //}

    //pub fn collision_detection(&mut self, game: &mut Game, game_audio: &audio::GameAudio) {
    //    if let Some(bullet) = &self.last_bullet {
    //        let pos = bullet.get_pos();

    //        //check for enemy right above
    //        let new_y = pos.y - bullet.get_speed();

    //        let item = &mut self.grid[new_y][pos.x];

    //        if item == "⍾" {
    //            game_audio.play_on_hit();

    //            item.clear();
    //            item.insert(0, ' ');
    //            self.clear_bullet();

    //            game.add_score(10.0);
    //        }
    //    }
    //}
}
