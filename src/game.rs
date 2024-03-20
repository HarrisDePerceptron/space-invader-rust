use rand::Rng;

use crate::container::{Container, Direction, Point};
use crate::enemy::{self, SmallAlien};
use crate::ship::Ship;
use crate::weapon::Bullet;

pub enum GameCondition {
    Running,
    Win,
    Loss,
    Ended,
}

pub struct Game {
    score: f32,
    lives: usize,
    max_lives: usize,

    enemies: Vec<SmallAlien>,

    window: Container,

    playable_area: Container,

    enemy_rows: usize,
    enemy_cols: usize,
    enemy_direction: Direction,
    enemy_speed: usize,
    enemy_last_move: std::time::Instant,
    enemy_move_duration: std::time::Duration,
    enemy_gap: usize,
    enemy_bullets: Vec<Bullet>,
    enemy_attack_wait_duration: std::time::Duration,
    last_enemy_attack_tick: std::time::Instant,

    last_bullet: Option<Bullet>,

    ship: Ship,
}

impl Game {
    pub fn new(
        width: usize,
        height: usize,
        score: f32,
        lives: usize,
        enemy_rows: usize,
        enemy_cols: usize,
    ) -> Self {
        let (window, playable_area) = Self::build_containers(width, height);

        let ship = Ship::new(playable_area.top.x + 1, playable_area.bottom.y - 1, 3, 1);

        let now = std::time::Instant::now();

        let mut game = Self {
            score,
            lives,
            max_lives: lives,
            window,
            enemies: vec![],
            playable_area,
            enemy_rows,
            enemy_cols,
            enemy_direction: Direction::LEFT,
            enemy_speed: 1,
            last_bullet: None,
            enemy_last_move: now,
            enemy_move_duration: std::time::Duration::from_millis(200),
            ship,
            enemy_gap: 2,
            enemy_bullets: vec![],
            enemy_attack_wait_duration: std::time::Duration::from_millis(1500),
            last_enemy_attack_tick: now,
        };

        game.init();
        game
    }

    pub fn get_max_lives(&self) -> usize {
        self.max_lives
    }

    pub fn set_lives(&mut self, lives: usize) {
        self.lives = lives
    }

    pub fn set_score(&mut self, score: f32) {
        self.score = score;
    }

    pub fn build_containers(width: usize, height: usize) -> (Container, Container) {
        let window = Container {
            top: Point { x: 0, y: 0 },
            bottom: Point {
                x: width,
                y: height,
            },
            padding_vertical: 2,
            padding_horizontal: 2,
        };

        let playable_width = (width as f32) * 0.7;
        let playable_width = playable_width as usize;

        let playable_area = Container {
            top: Point {
                x: window.top.x + window.padding_horizontal,
                y: window.top.y + window.padding_vertical,
            },
            bottom: Point {
                x: playable_width - window.padding_horizontal,
                y: window.bottom.y - window.padding_vertical,
            },
            padding_horizontal: 0,
            padding_vertical: 0,
        };

        (window, playable_area)
    }

    pub fn set_window(&mut self, width: usize, height: usize) {
        let (window, playable_area) = Self::build_containers(width, height);

        self.window = window;
        self.playable_area = playable_area;
    }

    pub fn set_enemy_rows_cols(&mut self, mut rows: usize, mut cols: usize) {
        if rows >= self.playable_area.get_height() - 1 {
            rows = self.playable_area.get_height() - 1;
        }

        if cols >= self.playable_area.get_width() - 1 {
            cols = self.playable_area.get_width() - 1;
        }

        self.enemy_rows = rows;
        self.enemy_cols = cols;

        self.init_enemy(0, 0);
    }

    pub fn get_enemies_rows_cols(&self) -> (usize, usize) {
        (self.enemy_rows, self.enemy_cols)
    }

    pub fn get_playablearea(&self) -> Container {
        self.playable_area.clone()
    }

    pub fn get_window(&self) -> Container {
        self.window.clone()
    }

    pub fn get_score(&self) -> f32 {
        self.score
    }
    pub fn get_lives(&self) -> usize {
        self.lives
    }

    pub fn add_score(&mut self, s: f32) {
        self.score += s;
    }

    pub fn init_ship(&mut self) {
        let ship = Ship::new(
            self.playable_area.top.x + 1,
            self.playable_area.bottom.y - 1,
            3,
            1,
        );
        self.ship = ship;
    }
    pub fn init_enemy(&mut self, start_x: usize, start_y: usize) {
        self.enemies.clear();

        let start_row = start_y + self.playable_area.top.x + 1;
        let start_col = start_x + self.playable_area.top.y + 1;

        for i in start_row..start_row + self.enemy_rows {
            for j in start_col..start_col + self.enemy_cols {
                let x = j * self.enemy_gap;

                let enemy = SmallAlien::new(x, i);
                self.enemies.push(enemy);
            }
        }
    }

    fn is_enemy_edge(&self) -> bool {
        let start_x = self.playable_area.top.x + 1;
        let end_x = self.playable_area.bottom.x - 1;

        for e in &self.enemies {
            let current_pos = e.get_pos();

            if current_pos.x >= end_x {
                return true;
            }

            if current_pos.x <= start_x {
                return true;
            }
        }

        return false;
    }

    pub fn move_enemy(&mut self) {
        let now = std::time::Instant::now();
        let diff = now - self.enemy_last_move;

        if diff >= self.enemy_move_duration {
            self.enemy_last_move = now;
        } else {
            return;
        }

        let on_edge = self.is_enemy_edge();
        let mut should_change_row = false;

        if on_edge {
            if let Direction::LEFT = self.enemy_direction {
                self.enemy_direction = Direction::RIGHT;
                should_change_row = true;
            } else if let Direction::RIGHT = self.enemy_direction {
                self.enemy_direction = Direction::LEFT;
                should_change_row = true;
            }
        }

        for e in &mut self.enemies {
            let mut current_pos = e.get_pos();

            if let Direction::RIGHT = self.enemy_direction {
                let new_x = current_pos.x + self.enemy_speed;
                current_pos.x = new_x;
            } else {
                let new_x = current_pos.x - self.enemy_speed;
                current_pos.x = new_x;
            }

            if should_change_row {
                current_pos.y += 1;
            }

            e.set_pos(&current_pos);
        }
    }

    pub fn get_enemies(&self) -> &Vec<SmallAlien> {
        &self.enemies
    }

    pub fn enemy_attack(&mut self) {
        let mut rng = rand::thread_rng();
        let rand_index: usize = rng.gen_range(0..self.enemies.len());

        let sel_enemy = &self.enemies[rand_index];
        let enemy_container = sel_enemy.get_container();

        let enemy_fire_stride: usize = sel_enemy.get_width() / 2;
        let enemy_fire_x = enemy_container.top.x + enemy_fire_stride;
        let enemy_fire_y = enemy_container.bottom.y + 1;

        let now = std::time::Instant::now();
        let last_enemy_attack_tick = now - self.last_enemy_attack_tick;

        if last_enemy_attack_tick >= self.enemy_attack_wait_duration {
            let bullet = Bullet::new(enemy_fire_x, enemy_fire_y, Direction::DOWN);
            self.enemy_bullets.push(bullet);
            self.last_enemy_attack_tick = now;
        }
    }

    fn reduce_life(&mut self) {
        if self.lives > 0 {
            self.lives -= 1;
        }
    }

    fn increase_life(&mut self) {
        self.lives += 1;
    }

    fn detect_ship_collision(&mut self) {
        let mut destroyed = false;
        let mut bullet_index = 0;

        for (idx, eb) in self.enemy_bullets.iter_mut().enumerate() {
            let next_pos = eb.next_pos();
            if let Direction::DOWN = eb.get_direction() {
                let ship_container = self.ship.get_container();

                if ship_container.top.y == next_pos.y
                    && (next_pos.x >= ship_container.top.x && next_pos.x <= ship_container.bottom.x)
                {
                    eb.destroy();
                    self.ship.destroy();
                    bullet_index = idx;
                    destroyed = true;
                }
            }
        }

        if destroyed {
            self.reduce_life();
            self.init_ship();
            self.enemy_bullets.remove(bullet_index);
        }
    }
    fn detect_enemy_collision(&mut self) {
        let mut score = 0.0;

        if let Some(bullet) = &mut self.last_bullet {
            if bullet.is_destroyed() {
                return;
            }
            let b_pos = bullet.get_pos();

            //check for enemy right above
            let new_y = b_pos.y - bullet.get_speed();
            for e in &mut self.enemies {
                if e.is_destroyed() {
                    continue;
                }
                let e_pos = e.get_pos();

                if e_pos.x == b_pos.x && e_pos.y == new_y {
                    e.destroy().expect("Failed to destroy enemy");
                    bullet.destroy();
                    let points = e.get_points() as f32;
                    score += points;

                    break;
                }
            }
        }

        self.add_score(score);
    }

    fn collision_detection(&mut self) {
        self.detect_enemy_collision();
        self.detect_ship_collision();
    }

    fn move_bullet(&mut self) {
        if let Some(bullet) = &mut self.last_bullet {
            let next_pos = bullet.next_pos();
            if next_pos.y <= self.playable_area.top.y || next_pos.y >= self.playable_area.bottom.y {
                bullet.destroy();
                self.last_bullet = None;
            } else {
                bullet.move_tick();
            }
        }
    }

    fn move_enemy_bullets(&mut self) {
        let mut destroyed_indexes = vec![];

        for (i, eb) in self.enemy_bullets.iter_mut().enumerate() {
            let next_pos = eb.next_pos();
            if next_pos.y <= self.playable_area.top.y || next_pos.y >= self.playable_area.bottom.y {
                eb.destroy();
                destroyed_indexes.push(i);
            } else {
                eb.move_tick();
            }
        }

        for idx in destroyed_indexes {
            self.enemy_bullets.remove(idx);
        }
    }

    pub fn fire_bullet(&mut self) {
        if self.last_bullet.is_none() {
            let ship_container = self.ship.get_container();
            let middle: usize = self.ship.get_width() / 2;

            let x = ship_container.top.x + middle;
            let y = ship_container.top.y - 1;

            let bullet = Bullet::new(x, y, Direction::UP);

            self.last_bullet = Some(bullet);
        }
    }

    pub fn init(&mut self) {
        self.init_enemy(0, 0);
        self.init_ship();
    }

    fn has_game_ended(&self) -> GameCondition {
        let total_enemies = self.enemy_rows * self.enemy_cols;
        let mut destroyed_enemies = 0;
        for e in &self.enemies {
            if e.is_destroyed() {
                destroyed_enemies += 1;
            }
        }

        if destroyed_enemies >= total_enemies {
            return GameCondition::Win;
        }
        if self.enemies.len() == 0 {
            return GameCondition::Win;
        }

        if self.lives == 0 {
            return GameCondition::Loss;
        }

        let last_y = self.playable_area.bottom.y - 1;

        for e in &self.enemies {
            let current_pos = e.get_pos();
            if current_pos.y >= last_y {
                return GameCondition::Loss;
            }
        }

        GameCondition::Running
    }

    pub fn get_enemy_bullets(&self) -> &Vec<Bullet> {
        &self.enemy_bullets
    }

    // keep ticking until game conditions have met
    pub fn tick(&mut self) -> GameCondition {
        self.move_enemy();
        self.move_bullet();
        self.collision_detection();

        self.enemy_attack();
        self.move_enemy_bullets();

        self.has_game_ended()
    }

    pub fn move_ship(&mut self, direction: Direction) {
        let ship_container = self.ship.get_container();

        if ship_container.bottom.x >= self.playable_area.bottom.x {
            if let Direction::RIGHT = direction {
                return;
            }
        }

        if ship_container.top.x <= self.playable_area.top.x {
            if let Direction::LEFT = direction {
                return;
            }
        }

        self.ship.move_ship(direction);
    }

    pub fn get_ship(&self) -> &Ship {
        &self.ship
    }

    pub fn get_bullet(&self) -> &Option<Bullet> {
        &self.last_bullet
    }

    pub fn reset_game(&mut self) {
        self.lives = self.max_lives;
        self.score = 0.0;
        self.last_bullet = None;
        self.enemy_bullets.clear();

        self.init();
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new(64, 32, 0.0, 3, 5, 11)
    }
}
