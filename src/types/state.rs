use rand::seq::SliceRandom;
use rand::thread_rng;

use std::{
    cell::RefCell,
    collections::VecDeque,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    rc::Rc,
};

use crate::utils::{read_file_list, MusicPlayer};

use super::{config::Config, file_info::FileInfo};

#[derive(Default)]
pub struct State {
    pub config: Config,
    pub file_list: Vec<FileInfo>,
    pub play_list: VecDeque<FileInfo>,
    pub player: MusicPlayer,
}

impl State {
    pub fn shared() -> SharedState {
        Rc::new(RefCell::new(Self {
            config: State::load_from_config_file().unwrap_or(Config::default()),
            file_list: vec![],
            play_list: VecDeque::new(),
            player: Default::default(),
        }))
    }

    pub fn read_music_list(&mut self) -> Option<()> {
        let list = read_file_list(&self.config.directory_path).ok()?;
        self.file_list = list;

        Some(())
    }

    pub fn insert_into_play_list(&mut self) {
        let mut temp = self.file_list.clone();
        temp.shuffle(&mut thread_rng());

        temp.into_iter().for_each(|e| self.play_list.push_back(e));
    }

    pub fn set_directory_path(&mut self, directory_path: PathBuf) {
        self.config.directory_path = directory_path;
    }

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

    fn load_from_config_file() -> Option<Config> {
        let path = "config.json";
        let text = fs::read_to_string(path).ok()?;

        let config = serde_json::from_str(text.as_str()).ok()?;

        Some(config)
    }
}

pub type SharedState = Rc<RefCell<State>>;
