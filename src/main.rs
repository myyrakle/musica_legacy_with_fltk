mod components;
mod errors;
mod types;
mod utils;

use std::fs::File;
use std::io::BufReader;
use std::sync::{mpsc, Arc};

use fltk::{app, group::Tabs, prelude::*, window::Window};
use rodio::Decoder;

use crate::components::{main_group::create_main_group, setting_group::create_setting_group};
use crate::types::{ClientEvent, MusicPlayStatus, State};

#[tokio::main]
async fn main() {
    let (event_sender, event_receiver) = mpsc::channel::<ClientEvent>();

    let state = State::new(event_sender.clone());
    state.lock().unwrap().read_music_list();

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

    let _event_listner_task = tokio::spawn(async move {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();

        loop {
            if let Ok(event) = event_receiver.recv() {
                match event {
                    ClientEvent::Start => {
                        if let Some(file) = state.lock().unwrap().get_current_file() {
                            let file = File::open(file.filepath).unwrap();
                            let buffer = BufReader::new(file);
                            let source = Decoder::new(buffer).unwrap();

                            sink.append(source);
                            state.lock().unwrap().status = MusicPlayStatus::Playing;
                        }

                        println!("start event");
                    }
                    ClientEvent::Resume => {
                        println!("resume event");
                    }
                    ClientEvent::Stop => {
                        println!("stop event");
                    }
                    ClientEvent::Left => {
                        println!("left event");
                    }
                    ClientEvent::Right => {
                        println!("right event");
                    }
                    ClientEvent::IntervalCheck => {
                        println!("interval check");
                    }
                }
            }
        }
    });

    let _background_task = tokio::spawn(async move {
        loop {
            if let Err(error) = event_sender.send(ClientEvent::IntervalCheck) {
                println!("{:?}", error);
            }

            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    });

    app.run().unwrap();
}
