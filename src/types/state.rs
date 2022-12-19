use std::{
    cell::RefCell,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    rc::Rc,
};

use crate::utils::read_file_list;

use super::{config::Config, file_info::FileInfo};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct State {
    pub config: Config,
    pub file_list: Vec<FileInfo>,
}

impl State {
    pub fn shared() -> SharedState {
        Rc::new(RefCell::new(Self {
            config: State::load_from_config_file().unwrap_or(Config::default()),
            file_list: vec![],
        }))
    }

    pub fn read_music_list(&mut self) -> Option<()> {
        let list = read_file_list(&self.config.directory_path).ok()?;
        self.file_list = list;

        Some(())
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