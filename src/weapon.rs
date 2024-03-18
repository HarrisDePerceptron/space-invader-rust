use std::ops::{Deref, DerefMut};

use crate::{
    audio::{GameAudio, GameObjectSound},
    container::{Container, Direction, Point},
    gobj::GameObject,
};

#[derive(Debug, Clone)]
pub struct Bullet {
    pub location: Container,
    pub speed: usize,

    pub last_bullet_tick: Option<std::time::Instant>,
    pub tick_duration: std::time::Duration,

    pub gobj: GameObject,
    pub direction: Direction,

    on_fire_audio: GameObjectSound,
}

impl Deref for Bullet {
    type Target = GameObject;

    fn deref(&self) -> &Self::Target {
        &self.gobj
    }
}

impl DerefMut for Bullet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.gobj
    }
}

impl Bullet {
    pub fn new(x: usize, y: usize, direction: Direction) -> Self {
        let container = Container::new(Point { x, y }, Point { x, y });

        let on_fire_audio = GameObjectSound::new("assets/sounds/hit.mp3");

        Self {
            location: Container::new(Point { x, y }, Point { x, y }),
            speed: 1,
            tick_duration: std::time::Duration::from_millis(30),
            last_bullet_tick: None,
            gobj: GameObject::new(container, "⌇"),
            direction,
            on_fire_audio,
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

    pub fn get_speed(&self) -> usize {
        self.speed
    }

    fn move_bullet(&mut self) {
        let mut pos = self.get_pos();
        let new_y = pos.y - self.speed;
        pos.y = new_y;
        self.set_pos(&pos)
    }

    pub fn move_tick(&mut self) {
        let now = std::time::Instant::now();
        if let Some(t) = self.last_bullet_tick {
            let diff = now - t;
            if diff >= self.tick_duration {
                self.move_bullet();
                self.last_bullet_tick = Some(now);
            }
        } else {
            self.last_bullet_tick = Some(now);
            self.on_fire_audio
                .play()
                .expect("Failed to play bullet audio");
        }
    }

    pub fn next_pos(&self) -> Point {
        let pos = self.get_pos();
        let mut new_y = pos.y - self.get_speed();

        if let Direction::DOWN = self.direction {
            new_y = pos.y + self.get_speed();
        }

        Point { x: pos.x, y: new_y }
    }
}
