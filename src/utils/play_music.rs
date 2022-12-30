use rodio::Sink;
use std::path::PathBuf;
use std::{collections::VecDeque, io::BufReader};

use rodio::{source::Source, Decoder, OutputStream};

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::types::file_info::FileInfo;

use super::read_file_list;

#[derive(Debug, Clone)]
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
    pub current_file: Option<FileInfo>,
    pub play_queue: VecDeque<FileInfo>,
}

impl Default for MusicPlayer {
    fn default() -> Self {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();

        Self {
            sink,
            status: MusicPlayStatus::Stopped,
            file_list: vec![],
            current_file: None,
            play_queue: VecDeque::new(),
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

    pub fn complete(&mut self) {
        self.status = MusicPlayStatus::Completed;
        self.current_file = None;
    }

    // 플레이를 시작합니다.
    // 블록되기 때문에, 비동기적으로 실행하고자 한다면 별도 태스크에서 호출해야 합니다.
    pub fn start(&mut self, file: FileInfo) {
        self.current_file = Some(file.clone());

        println!("{:?}", self.current_file);

        let file = std::fs::File::open(file.filepath).unwrap();

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // Load a sound from a file, using a path relative to Cargo.toml
        let file_reader = BufReader::new(file);
        // Decode that sound file into a source
        let source = Decoder::new(file_reader).unwrap();
        // Play the sound directly on the device
        stream_handle.play_raw(source.convert_samples());

        // The sound plays in a separate audio thread,
        // so we need to keep the main thread alive while it's playing.
        std::thread::sleep(std::time::Duration::from_secs(5));

        self.status = MusicPlayStatus::Playing;

        println!("1");

        // 재생 종료
        self.complete();
    }

    // 디렉토리 경로에서 음악 파일 목록을 가져옵니다.
    pub fn read_music_list(&mut self, directory_path: &PathBuf) -> Option<()> {
        let list = read_file_list(directory_path).ok()?;
        self.file_list = list;

        Some(())
    }

    // 실행 대기열에 랜덤으로 한 루프를 집어넣습니다.
    pub fn insert_into_play_queue(&mut self) {
        let mut temp = self.file_list.clone();
        temp.shuffle(&mut thread_rng());

        temp.into_iter().for_each(|e| self.play_queue.push_back(e));
    }

    // 실행 대기열에서 파일 하나를 가져옵니다.
    // 대기열이 비었다면 다시 충전합니다.
    pub fn get_next_file_from_queue(&mut self) -> Option<FileInfo> {
        if self.play_queue.is_empty() {
            self.insert_into_play_queue();
        }

        return self.play_queue.pop_front();
    }
}
