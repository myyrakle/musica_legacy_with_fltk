use std::{
    cell::RefCell,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    rc::Rc,
};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct State {
    pub directory_path: PathBuf,
}

impl State {
    pub fn set_directory_path(&mut self, directory_path: PathBuf) {
        self.directory_path = directory_path;
    }

    pub fn write_to_config_file(&self) {
        let path = "config.json";

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap();

        let json_string = serde_json::to_string(self).unwrap();

        file.write_all(json_string.as_bytes()).unwrap();
    }

    pub fn load_from_config_file() -> Option<Self> {
        let path = "config.json";
        let text = fs::read_to_string(path).ok()?;

        let config = serde_json::from_str(text.as_str()).ok()?;

        Some(config)
    }
}

pub type SharedState = Rc<RefCell<State>>;
