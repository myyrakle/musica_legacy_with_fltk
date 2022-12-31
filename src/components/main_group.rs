use std::sync::Arc;

use fltk::{button::Button, group::Group, prelude::*};

use crate::types::{client_event::ClientEvent, music_status::MusicPlayStatus, state::SharedState};

pub fn create_main_group(state_: SharedState, window_width: i32, window_height: i32) -> Group {
    let _event_sender = state_.lock().unwrap().event_sender.clone();

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

    let _state = Arc::clone(&state_);
    left_button.set_callback(move |_| {});

    let state = Arc::clone(&state_);
    let event_sender = _event_sender.clone();

    stop_button.set_callback(move |_| match state.lock().unwrap().status {
        MusicPlayStatus::Stopped => {
            if let Err(error) = event_sender.send(ClientEvent::Start) {
                println!("{:?}", error);
            }

            // println!("!!");
            // state.lock().unwrap().read_music_list();

            // println!("??");

            // if let Some(file_info) = state.lock().unwrap().player.get_next_file_from_queue() {
            //     let state = Arc::clone(&state);

            //     println!("##");

            //     tokio::spawn(async move {
            //         println!("^^");
            //         state.lock().unwrap().player.start(file_info);
            //         println!("**");
            //     });
            // }
        }
        MusicPlayStatus::Playing => {
            if let Err(error) = event_sender.send(ClientEvent::Stop) {
                println!("{:?}", error);
            }
            //state.lock().unwrap().player.pause();
        }
        MusicPlayStatus::Paused => {
            if let Err(error) = event_sender.send(ClientEvent::Resume) {
                println!("{:?}", error);
            }
            //state.lock().unwrap().player.resume();
        }
        MusicPlayStatus::Completed => {}
    });

    let event_sender = _event_sender.clone();
    left_button.set_callback(move |_| {
        if let Err(error) = event_sender.send(ClientEvent::Left) {
            println!("{:?}", error);
        }
    });

    let event_sender = _event_sender.clone();
    right_button.set_callback(move |_| {
        if let Err(error) = event_sender.send(ClientEvent::Right) {
            println!("{:?}", error);
        }
    });

    main_group.end();
    main_group
}
