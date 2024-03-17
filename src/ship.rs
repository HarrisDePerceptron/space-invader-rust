use std::ops::{Deref, DerefMut};

use crate::audio::GameObjectSound;
use crate::container::{Container, Direction, Point};
use crate::gobj::GameObject;
use crate::weapon::Bullet;

use anyhow::Result;

pub struct Ship {
    gobj: GameObject,
    direction: Direction,
    speed: usize,
    length: usize,

    fire_sound: GameObjectSound,
}

impl Deref for Ship {
    type Target = GameObject;

    fn deref(&self) -> &Self::Target {
        &self.gobj
    }
}

impl DerefMut for Ship {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.gobj
    }
}

impl Ship {
    pub fn new(x: usize, y: usize, length: usize, speed: usize) -> Self {
        let container = Container::new(Point { x, y }, Point { x: x + length, y });
        let symbol = "⌬";
        let gobj = GameObject::new(container, symbol);

        let fire_sound = GameObjectSound::new("assets/sounds/on_hit.mp3");

        Self {
            gobj,
            direction: Direction::RIGHT,
            speed,
            length,
            fire_sound,
        }
    }

    pub fn set_position(&mut self, x: usize) {
        let current_pos = self.get_container();

        let x_end = x + self.length;
        let y = current_pos.top.y;

        let container = Container::new(Point { x, y }, Point { x: x_end, y });

        self.set_container(&container);
    }

    pub fn move_ship(&mut self, direction: Direction) {
        match direction {
            Direction::UP | Direction::DOWN => return,
            Direction::LEFT | Direction::RIGHT => (),
        };

        let current_pos = self.get_pos();

        if let Direction::LEFT = direction {
            let new_x = current_pos.x - self.speed;
            self.set_position(new_x);
        } else {
            let new_x = current_pos.x + self.speed;
            self.set_position(new_x);
        }
    }
    pub fn get_length(&self) -> usize {
        self.length
    }
}
