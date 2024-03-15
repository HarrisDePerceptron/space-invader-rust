use rodio::{source::Source, Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

use anyhow::{anyhow as error, Result};

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
    pub fn play_audio(&self, path: &str) -> Result<()> {
        // Get a output stream handle to the default physical sound device
        //
        let path_t = path.to_string();
        std::thread::spawn(move || {
            let result = Self::play(&path_t);
            if let Err(e) = result {
                println!("Failed to playu audio: {:?}", e);
            }
        });

        //self.audio_threads.push(handle);

        //self.audio_threads.push(t1);

        Ok(())
    }

    pub fn play_fire(&self) -> Result<()> {
        self.play_audio(&self.hit_path)?;
        Ok(())
    }

    pub fn play_on_hit(&self) -> Result<()> {
        self.play_audio(&self.on_hit_path)?;
        Ok(())
    }
}
