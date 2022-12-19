use std::{cell::RefCell, path::PathBuf, rc::Rc};

#[derive(Debug, Clone, Default)]
pub struct State {
    pub directory_path: PathBuf,
}

pub type SharedState = Rc<RefCell<State>>;
