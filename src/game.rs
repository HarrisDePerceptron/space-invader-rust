use crate::container::{Container, Direction, Point};
use crate::enemy::SmallAlien;
use crate::ship::Ship;
use crate::weapon::Bullet;

pub struct Game {
    score: f32,
    lives: usize,

    enemies: Vec<SmallAlien>,

    window: Container,

    playable_area: Container,

    enemy_rows: usize,
    enemy_cols: usize,
    enemy_direction: Direction,
    enemy_speed: usize,
    enemy_last_move: std::time::Instant,
    enemy_move_duration: std::time::Duration,

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
        let window = Container {
            top: Point { x: 0, y: 0 },
            bottom: Point {
                x: width,
                y: height,
            },
            padding_vertical: 2,
            padding_horizontal: 2,
        };

        let playable_area = Container {
            top: Point {
                x: window.top.x + window.padding_horizontal,
                y: window.top.y + window.padding_vertical,
            },
            bottom: Point {
                x: window.bottom.x - window.padding_horizontal,
                y: window.bottom.y - window.padding_vertical,
            },
            padding_horizontal: 0,
            padding_vertical: 0,
        };

        let ship = Ship::new(playable_area.top.x + 1, playable_area.bottom.y - 1, 3, 1);

        let mut game = Self {
            score,
            lives,
            window,
            enemies: vec![],
            playable_area,
            enemy_rows,
            enemy_cols,
            enemy_direction: Direction::LEFT,
            enemy_speed: 1,
            last_bullet: None,
            enemy_last_move: std::time::Instant::now(),
            enemy_move_duration: std::time::Duration::from_millis(1000),
            ship,
        };

        game.init();
        game
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

    pub fn init_enemy(&mut self, start_x: usize, start_y: usize) {
        self.enemies.clear();

        let start_row = start_y + self.playable_area.top.x + 1;
        let start_col = start_x + self.playable_area.top.y + 1;

        for i in start_row..start_row + self.enemy_rows {
            for j in start_col..start_col + self.enemy_cols {
                let enemy = SmallAlien::new(j, i);
                self.enemies.push(enemy);
            }
        }
    }

    fn is_enemy_edge(&self) -> bool {
        let start_x = self.playable_area.top.x + 1;
        let end_x = self.playable_area.bottom.x - 1;

        println!("start x is: {}", start_x);

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

        let start_x = self.playable_area.top.x + 1;
        let start_y = self.playable_area.top.y + 1;

        let end_x = self.playable_area.bottom.x - 1;
        let end_y = self.playable_area.bottom.y - 1;

        let on_edge = self.is_enemy_edge();
        if on_edge {
            if let Direction::LEFT = self.enemy_direction {
                self.enemy_direction = Direction::RIGHT;
            } else if let Direction::RIGHT = self.enemy_direction {
                self.enemy_direction = Direction::LEFT;
            }
        }

        println!("Direction is : {:?}", self.enemy_direction);

        for e in &mut self.enemies {
            let mut current_pos = e.get_pos();
            let mut new_y = current_pos.y;

            //if on_edge && (new_y < end_y) {
            //    new_y += 1;
            //    current_pos.y = new_y;
            //}

            //current_pos.y = new_y;

            if let Direction::RIGHT = self.enemy_direction {
                let new_x = current_pos.x + self.enemy_speed;
                current_pos.x = new_x;
            } else {
                let new_x = current_pos.x - self.enemy_speed;
                current_pos.x = new_x;
            }

            e.set_pos(&current_pos);
        }
    }

    pub fn get_enemies(&self) -> &Vec<SmallAlien> {
        &self.enemies
    }

    pub fn collision_detection(&mut self) {
        let mut score = 0.0;

        if let Some(bullet) = &mut self.last_bullet {
            let b_pos = bullet.get_pos();

            //check for enemy right above
            let new_y = b_pos.y - bullet.get_speed();

            //let item = &mut self.grid[new_y][pos.x];

            for e in &mut self.enemies {
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

    pub fn move_bullet(&mut self) {
        if let Some(bullet) = &mut self.last_bullet {
            let next_pos = bullet.next_pos();
            if next_pos.y <= self.playable_area.top.y || next_pos.y >= self.playable_area.bottom.y {
                bullet.destroy();
            } else {
                bullet.move_tick();
            }
        }
    }

    pub fn fire_bullet(&mut self, start: Point) {
        if self.last_bullet.is_none() {
            self.last_bullet = Some(Bullet::new(start.x, start.y, Direction::UP));
        }
    }

    pub fn init(&mut self) {
        self.init_enemy(0, 0);
    }

    pub fn tick(&mut self) {
        self.move_enemy();
        self.move_bullet();
        self.collision_detection();
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
}

impl Default for Game {
    fn default() -> Self {
        Self::new(64, 32, 0.0, 3, 5, 11)
    }
}
