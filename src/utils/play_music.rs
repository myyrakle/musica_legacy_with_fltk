use rodio::Sink;
use std::path::PathBuf;
use std::{collections::VecDeque, io::BufReader};

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::types::file_info::FileInfo;

use super::read_file_list;

pub enum MusicPlayStatus {
    Stopped,   // 중단 상태. 초기 상태.
    Playing,   // 실행중
    Paused,    // 일시정지
    Completed, // 한 곡이 끝난 후 대기상태
}

pub struct MusicPlayer {
    pub sink: Sink,
    pub status: MusicPlayStatus,
    pub file_list: Vec<FileInfo>,
    pub play_list: VecDeque<FileInfo>,
}

impl Default for MusicPlayer {
    fn default() -> Self {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();

        Self {
            sink,
            status: MusicPlayStatus::Stopped,
            file_list: vec![],
            play_list: VecDeque::new(),
        }
    }
}

impl MusicPlayer {
    pub fn pause(&mut self) {
        self.sink.pause();
        self.status = MusicPlayStatus::Paused;
    }

    pub fn resume(&mut self) {
        self.sink.play();
        self.status = MusicPlayStatus::Playing;
    }

    pub fn start(&'static self, file: FileInfo) {
        tokio::spawn(async {
            let file = std::fs::File::open(file.filepath).unwrap();

            self.sink
                .append(rodio::Decoder::new(BufReader::new(file)).unwrap());

            self.sink.sleep_until_end();
        });
    }

    pub fn read_music_list(&mut self, directory_path: &PathBuf) -> Option<()> {
        let list = read_file_list(directory_path).ok()?;
        self.file_list = list;

        Some(())
    }

    pub fn insert_into_play_list(&mut self) {
        let mut temp = self.file_list.clone();
        temp.shuffle(&mut thread_rng());

        temp.into_iter().for_each(|e| self.play_list.push_back(e));
    }
}
