use std::ops::{Deref, DerefMut};

use crate::container::{Container, Point};

use anyhow::Result;

use crate::audio::GameObjectSound;
use crate::gobj::GameObject;

pub struct SmallAlien {
    pub gobj: GameObject,
    points: usize,
    hit_sound: GameObjectSound,
    on_hit_sound: GameObjectSound,
}

impl Deref for SmallAlien {
    type Target = GameObject;

    fn deref(&self) -> &Self::Target {
        &self.gobj
    }
}

impl DerefMut for SmallAlien {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.gobj
    }
}

impl SmallAlien {
    pub fn new(x: usize, y: usize) -> Self {
        let container = Container {
            top: Point { x, y },
            bottom: Point { x, y },
            padding_vertical: 0,
            padding_horizontal: 0,
        };

        let symbol = "⍾";
        let points = 10;

        let gobj = GameObject::new(container, symbol);
        let hit_sound = GameObjectSound::new("assets/sounds/hit.mp3");
        let on_hit_sound = GameObjectSound::new("assets/sounds/on_hit.mp3");

        Self {
            gobj,
            points,
            hit_sound,
            on_hit_sound,
        }
    }

    pub fn get_points(&self) -> usize {
        self.points
    }

    pub fn fire(&self) -> Result<()> {
        self.hit_sound.play()
    }

    pub fn destroy(&mut self) -> Result<()> {
        self.gobj.destroy();
        self.on_hit_sound.play()
    }
}
