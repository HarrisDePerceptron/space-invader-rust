use rodio::{source::Source, Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

use anyhow::{anyhow as error, Result};

#[derive(Debug, Clone)]
pub struct GameObjectSound {
    path: String,
}

impl GameObjectSound {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
    pub fn play(&self) -> Result<()> {
        GameAudio::play_audio(&self.path)?;
        Ok(())
    }
}

pub struct GameAudio {
    hit_path: String,
    on_hit_path: String,
}

impl GameAudio {
    pub fn new() -> Self {
        Self {
            hit_path: "assets/sounds/hit.mp3".to_string(),
            on_hit_path: "assets/sounds/on_hit.mp3".to_string(),
        }
    }

    fn play(path: &str) -> Result<()> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let file = BufReader::new(File::open(path)?);
        let source = Decoder::new(file)?;
        let sink = Sink::try_new(&stream_handle)?;
        sink.append(source);
        sink.sleep_until_end();
        Ok(())
    }
    pub fn play_audio(path: &str) -> Result<()> {
        let path_t = path.to_string();
        std::thread::spawn(move || {
            let result = Self::play(&path_t);
            if let Err(e) = result {
                println!("Failed to playu audio: {:?}", e);
            }
        });

        Ok(())
    }

    pub fn play_fire(&self) -> Result<()> {
        Self::play_audio(&self.hit_path)?;
        Ok(())
    }

    pub fn play_on_hit(&self) -> Result<()> {
        Self::play_audio(&self.on_hit_path)?;
        Ok(())
    }
}
