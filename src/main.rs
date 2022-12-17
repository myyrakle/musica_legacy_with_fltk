mod errors;
mod utils;

use std::path::Path;

use fltk::{app, button::Button, prelude::*, window::Window};
use utils::read_file_list;

fn main() {
    let path = Path::new("./sample");

    let music_list = read_file_list(path).unwrap();

    let app = app::App::default();
    let mut window = Window::new(100, 100, 400, 300, "musica");

    let mut setting_button = Button::new(160, 210, 80, 40, "Click me!");
    setting_button.set_callback(move |_| {});

    window.end();
    window.show();
    app.run().unwrap();
}
