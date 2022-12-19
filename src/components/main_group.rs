use fltk::{button::Button, group::Group, prelude::*};

pub fn create_main_group(window_width: i32, window_height: i32) -> Group {
    let group_top_margin = 30;

    let main_group = Group::new(0, group_top_margin, window_width, window_height, "Main");

    let button_top_margin = 3;
    let button_left_margin = 3;

    let left_button_width = 40;
    let left_button_height = 40;
    let mut left_button = Button::new(
        button_left_margin,
        group_top_margin + button_top_margin,
        left_button_width,
        left_button_height,
        "⏮️",
    );

    let stop_button_width = 40;
    let stop_button_height = 40;
    let mut stop_button = Button::new(
        button_left_margin + left_button_width + button_left_margin,
        group_top_margin + button_top_margin,
        stop_button_width,
        stop_button_height,
        "⏸️",
    );

    let right_button_width = 40;
    let right_button_height = 40;
    let mut right_button = Button::new(
        button_left_margin
            + left_button_width
            + button_left_margin
            + stop_button_width
            + button_left_margin,
        group_top_margin + button_top_margin,
        right_button_width,
        right_button_height,
        "⏭️",
    );

    left_button.set_callback(move |_| {});
    stop_button.set_callback(move |_| {});
    right_button.set_callback(move |_| println!(""));

    main_group.end();
    main_group
}
