use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::utils::MusicPlayer;

use super::config::Config;

#[derive(Default)]
pub struct State {
    pub config: Config,
    pub player: MusicPlayer,
}

impl State {
    pub fn shared() -> SharedState {
        Arc::new(Mutex::new(Self {
            config: State::load_from_config_file().unwrap_or(Config::default()),
            player: Default::default(),
        }))
    }

    pub fn read_music_list(&mut self) -> Option<()> {
        self.player.read_music_list(&self.config.directory_path)
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

pub type SharedState = Arc<Mutex<State>>;
