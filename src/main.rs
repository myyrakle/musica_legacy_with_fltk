mod components;
mod errors;
mod types;
mod utils;

use std::sync::Arc;

use fltk::{app, group::Tabs, prelude::*, window::Window};

use crate::components::{main_group::create_main_group, setting_group::create_setting_group};
use crate::types::state::State;

#[tokio::main]
async fn main() {
    let state = State::shared();
    state.lock().unwrap().read_music_list();
    state.lock().unwrap().player.insert_into_play_list();

    let app = app::App::default().with_scheme(app::Scheme::Gtk);

    let window_width: i32 = 400;
    let window_height: i32 = 600;

    let mut window = Window::new(100, 100, window_width, window_height, "musica");

    let mut tabs = Tabs::new(0, 0, window_width, window_height, "main");

    let main_group = create_main_group(Arc::clone(&state), window_width, window_height);
    let setting_group = create_setting_group(Arc::clone(&state), window_width, window_height);

    tabs.add(&main_group);
    tabs.add(&setting_group);

    window.end();
    window.show();

    let _background_task = tokio::spawn(async {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    app.run().unwrap();
}
