use std::{
    collections::VecDeque,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    sync::{mpsc::Sender, Arc, Mutex},
};

use rand::seq::SliceRandom;
use rand::thread_rng;
use rodiogaga::Decoder;

use crate::utils::read_file_list;

use super::{
    client_event::ClientEvent, config::Config, file_info::FileInfo, music_status::MusicPlayStatus,
};

#[derive(Debug)]
pub struct State {
    pub config: Config,
    pub event_sender: Sender<ClientEvent>,
    pub file_list: Vec<FileInfo>,
    pub current_index: usize,
    pub play_queue: VecDeque<FileInfo>,
    pub status: MusicPlayStatus,
}

impl State {
    pub fn new(event_sender: Sender<ClientEvent>) -> SharedState {
        Arc::new(Mutex::new(Self {
            config: State::load_from_config_file().unwrap_or(Config::default()),
            file_list: vec![],
            current_index: 0,
            play_queue: VecDeque::new(),
            event_sender,
            status: MusicPlayStatus::Stopped,
        }))
    }

    pub fn set_directory_path(&mut self, directory_path: PathBuf) {
        self.config.directory_path = directory_path;
    }

    // 이전 곡으로 인덱스 이동
    pub fn index_to_left(&mut self) {
        if self.current_index == 0 {
            self.current_index = self.play_queue.len() - 1;
        } else {
            self.current_index -= 1;
        }
    }

    // 다음 곡으로 인덱스 이동
    pub fn index_to_right(&mut self) {
        self.current_index += 1;

        if self.play_queue.len() <= self.current_index {
            self.current_index = 0;
        }
    }

    // config 파일에 동기화
    pub fn write_to_config_file(&self) {
        let path = "config.json";

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap();

        let json_string = serde_json::to_string(&self.config).unwrap();

        file.write_all(json_string.as_bytes()).unwrap();
    }

    // config 파일 로드
    fn load_from_config_file() -> Option<Config> {
        let path = "config.json";
        let text = fs::read_to_string(path).ok()?;

        let config = serde_json::from_str(text.as_str()).ok()?;

        Some(config)
    }

    // 디렉토리 경로에서 음악 파일 목록을 가져옵니다.
    pub fn get_current_file(&self) -> Option<FileInfo> {
        self.play_queue.get(self.current_index).cloned()
    }

    pub fn get_current_source(&self) -> Option<Decoder<BufReader<File>>> {
        let file_info = self.get_current_file()?;
        let file = File::open(file_info.filepath).ok()?;
        let buffer = BufReader::new(file);
        let source = Decoder::new(buffer).ok()?;

        Some(source)
    }

    // 디렉토리 경로에서 음악 파일 목록을 가져옵니다.
    pub fn read_music_list(&mut self) -> Option<()> {
        let list = read_file_list(&self.config.directory_path).ok()?;
        self.file_list = list;
        self.play_queue = VecDeque::from(self.file_list.clone());

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

pub type SharedState = Arc<Mutex<State>>;
