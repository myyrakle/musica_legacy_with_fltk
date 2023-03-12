use std::{
    collections::VecDeque,
    fs::{self, File, OpenOptions},
    io::{BufReader, Write},
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
    pub title_sender: Sender<String>,
    pub directory_sender: Sender<String>,
    pub file_list: Vec<FileInfo>,
    pub current_index: usize,
    pub play_queue: VecDeque<FileInfo>,
    pub status: MusicPlayStatus,
}

impl State {
    pub fn new(
        event_sender: Sender<ClientEvent>,
        title_sender: Sender<String>,
        directory_sender: Sender<String>,
    ) -> SharedState {
        let this = Arc::new(Mutex::new(Self {
            config: State::load_from_config_file().unwrap_or(Config::default()),
            file_list: vec![],
            current_index: 0,
            play_queue: VecDeque::new(),
            event_sender,
            title_sender,
            directory_sender,
            status: MusicPlayStatus::Stopped,
        }));

        {
            let this = this.lock().unwrap();

            this.directory_sender
                .send(this.config.directory_path.to_str().unwrap().to_string())
                .ok();
        }

        this
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
            .create(true)
            .open(path)
            .unwrap();

        let json_string = serde_json::to_string(&self.config).unwrap();

        self.directory_sender
            .send(self.config.directory_path.to_str().unwrap().to_string())
            .ok();

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
        self.title_sender.send(file_info.filename).ok();
        let file = File::open(file_info.filepath).ok()?;

        let buffer = BufReader::new(file);
        println!("buffer: {}", buffer.capacity());
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

    // 실행 대기열을 랜덤으로 집어넣습니다.
    #[allow(dead_code)]
    pub fn ramdomize_play_queue(&mut self) {
        let mut temp = self.file_list.clone();
        temp.shuffle(&mut thread_rng());

        self.play_queue.clear();

        temp.into_iter().for_each(|e| self.play_queue.push_back(e));
    }

    // 실행 대기열을 원래 순서대로 집어넣습니다.
    #[allow(dead_code)]
    pub fn orderize_play_queue(&mut self) {
        let mut temp = self.file_list.clone();
        temp.shuffle(&mut thread_rng());

        self.play_queue.clear();

        temp.into_iter().for_each(|e| self.play_queue.push_back(e));
    }
}

pub type SharedState = Arc<Mutex<State>>;
