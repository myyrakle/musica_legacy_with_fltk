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

    let left_button_width = 40;
    let left_button_height = 40;
    let mut left_button = Button::new(0, 0, left_button_width, left_button_height, "⏮️");
    // left_button.set_color(Color::Green);
    // left_button.set_label_size(20);
    // left_button.set_label_font(Font::Courier);

    let stop_button_width = 40;
    let stop_button_height = 40;
    let mut stop_button = Button::new(
        0 + left_button_width,
        0,
        stop_button_width,
        stop_button_height,
        "⏸️",
    );

    let right_button_width = 40;
    let right_button_height = 40;
    let mut right_button = Button::new(
        0 + left_button_width + stop_button_width,
        0,
        right_button_width,
        right_button_height,
        "⏭️",
    );

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
    left_button.set_callback(move |_| {});
    stop_button.set_callback(move |_| {});
    right_button.set_callback(move |_| {});

    window.end();
    window.show();
    app.run().unwrap();
}
