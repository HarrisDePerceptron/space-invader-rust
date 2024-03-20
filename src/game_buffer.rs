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
    //pub boundary_coordinates: (usize, usize, usize, usize),
    //
    playable_area: Container,
    window: Container,
}

impl GameBuffer {
    pub fn new(game: &Game) -> GameBuffer {
        let window = game.get_window();

        let playable_area = game.get_playablearea();

        let mut game_buffer = GameBuffer {
            grid: vec![],
            rows: window.get_height(),
            cols: window.get_width(),
            enemy_rows: 5,
            enemy_cols: 11,
            enemy_gap: 2,

            ship_length: 3,
            ship_current_box: None,
            //boundary_coordinates: (2, 2, 62, 30),
            playable_area,
            window,
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

    //pub fn get_game_boundary(&self) -> (usize, usize, usize, usize) {
    //    self.boundary_coordinates
    //}

    pub fn get_ship_box(&self) -> Option<Container> {
        self.ship_current_box.clone()
    }

    fn draw_boundary(&mut self) {
        //let (x1, y1, x2, y2) = self.boundary_coordinates;

        for i in 0..self.window.get_height() {
            for j in 0..self.playable_area.get_width() + self.window.padding_horizontal * 2 {
                let row = &mut self.grid[i];
                let item = &mut row[j];

                if i == self.playable_area.top.y || i == self.playable_area.bottom.y {
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

        if total_len > self.playable_area.get_width() {
            panic!(
                "Text length exceeds total buffer cols: {}>{}",
                total_len, self.cols
            );
        }

        let space_len = self.playable_area.get_width() - total_len;

        let mut game_display_text = String::new();
        game_display_text += score_text.as_str();

        for _ in 0..space_len {
            game_display_text += " ";
        }

        game_display_text += lives_text.as_str();

        let display_row_index = self.playable_area.top.y - 1;
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

    fn draw_ship(&mut self, game: &Game) {
        let ship = game.get_ship();
        let ship_container = ship.get_container();

        let symbol = ship.get_symbol();

        for i in 0..ship.get_width() {
            let new_x = ship_container.top.x + i;
            self.grid[ship_container.top.y][new_x] = symbol.to_string();
        }
    }

    fn draw_bullet(&mut self, game: &Game) {
        let bullet = game.get_bullet();
        if let Some(b) = bullet {
            if b.is_destroyed() {
                return;
            }
            let contianer = b.get_container();
            let x = contianer.top.x;
            let y = contianer.top.y;

            let symbol = b.get_symbol().to_string();

            self.grid[y][x] = symbol;
        }
    }

    fn draw_enemy_bullet(&mut self, game: &Game) {
        let bullets = game.get_enemy_bullets();
        for eb in bullets {
            if eb.is_destroyed() {
                continue;
            }
            let contianer = eb.get_container();
            let x = contianer.top.x;
            let y = contianer.top.y;

            let symbol = eb.get_symbol().to_string();

            self.grid[y][x] = symbol;
        }
    }

    fn draw_enemies(&mut self, game: &Game) {
        for e in game.get_enemies() {
            if e.is_destroyed() {
                continue;
            }
            let pos = e.get_pos();
            let symbol = e.get_symbol();

            self.grid[pos.y][pos.x] = symbol.to_string();
        }
    }

    pub fn draw(&mut self, game: &Game) {
        self.clear();
        self.draw_text(game);

        self.draw_boundary();

        self.draw_ship(game);

        self.draw_bullet(game);

        self.draw_enemies(game);
        self.draw_enemy_bullet(game);
    }
}
