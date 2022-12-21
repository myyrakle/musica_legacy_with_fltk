use std::rc::Rc;

use fltk::{button::Button, group::Group, prelude::*};

use crate::{types::state::SharedState, utils::MusicPlayStatus};

pub fn create_main_group(state_: SharedState, window_width: i32, window_height: i32) -> Group {
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

    let _state = Rc::clone(&state_);
    left_button.set_callback(move |_| {});

    let state = Rc::clone(&state_);
    stop_button.set_callback(move |_| match state.borrow().player.status {
        MusicPlayStatus::Stopped => {
            state.borrow_mut().read_music_list();
            state.borrow_mut().player.insert_into_play_list();

            state.borrow_mut().player.status = MusicPlayStatus::Playing;
        }
        MusicPlayStatus::Playing => {
            state.borrow_mut().player.pause();
        }
        MusicPlayStatus::Paused => {
            state.borrow_mut().player.resume();
        }
        MusicPlayStatus::Completed => {}
    });

    let state = Rc::clone(&state_);
    right_button.set_callback(move |_| println!("{:?}", state.borrow().config.directory_path));

    main_group.end();
    main_group
}
