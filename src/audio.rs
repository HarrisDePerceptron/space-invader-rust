use rodio::{source::Source, Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

use anyhow::{anyhow as error, Result};

pub struct GameAudio {
    audio_threads: Vec<std::thread::JoinHandle<()>>,
}

impl GameAudio {
    pub fn new() -> Self {
        Self {
            audio_threads: vec![],
        }
    }

    fn play(path: &str) -> Result<()> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open(path)?);
        // Decode that sound file into a source
        let source = Decoder::new(file)?;
        // Play the sound directly on the device

        //stream_handle.play_raw(source.convert_samples())?;

        let sink = Sink::try_new(&stream_handle)?;

        sink.append(source);

        sink.sleep_until_end();
        Ok(())
    }
    pub fn play_audio(&mut self, path: &str) -> Result<()> {
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
}
