mod errors;
mod utils;

use std::path::Path;

use fltk::{app, button::Button, prelude::*, window::Window};
use utils::read_file_list;

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);

    let path = Path::new("./sample");

    let music_list = read_file_list(path).unwrap();
    println!("{:?}", music_list);

    let window_width: i32 = 400;
    let window_height: i32 = 600;

    let mut window = Window::new(100, 100, window_width, window_height, "musica");

    let setting_button_width = 80;
    let setting_button_height = 40;

    let mut setting_button = Button::new(
        window_width - setting_button_width,
        0,
        setting_button_width,
        setting_button_height,
        "Setting",
    );
    setting_button.set_callback(move |_| {});

    window.end();
    window.show();
    app.run().unwrap();
}
