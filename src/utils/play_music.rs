use rodio::Sink;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::types::file_info::{self, FileInfo};

pub struct MusicPlayer {
    sink: Sink,
}

impl Default for MusicPlayer {
    fn default() -> Self {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();

        Self { sink }
    }
}

impl MusicPlayer {
    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn play(&'static self, file: FileInfo) {
        tokio::spawn(async {
            let file = std::fs::File::open(file.filepath).unwrap();

            self.sink
                .append(rodio::Decoder::new(BufReader::new(file)).unwrap());

            self.sink.sleep_until_end();
        });
    }
}
