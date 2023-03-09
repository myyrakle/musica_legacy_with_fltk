mod components;
mod constants;
mod errors;
mod types;
mod utils;
use std::sync::{mpsc, Arc};

use constants::name::APP_NAME;
use fltk::enums::Event;
use fltk::{app, group::Tabs, prelude::*, window::Window};

use crate::components::{main_group::create_main_group, setting_group::create_setting_group};
use crate::types::{ClientEvent, MusicPlayStatus, State};

#[tokio::main]
async fn main() {
    let (_event_sender, event_receiver) = mpsc::channel::<ClientEvent>();
    let (_title_sender, title_receiver) = mpsc::channel::<String>();

    let event_sender = _event_sender.clone();
    let title_sender = _title_sender.clone();

    let state = State::new(event_sender, title_sender);
    state.lock().unwrap().read_music_list();

    let app = app::App::default().with_scheme(app::Scheme::Gtk);

    let window_width: i32 = 400;
    let window_height: i32 = 600;

    let mut window = Window::new(100, 100, window_width, window_height, APP_NAME);

    let mut tabs = Tabs::new(0, 0, window_width, window_height, None);

    let main_group = create_main_group(
        Arc::clone(&state),
        window_width,
        window_height,
        title_receiver,
    );
    let setting_group = create_setting_group(Arc::clone(&state), window_width, window_height);

    tabs.add(&main_group);
    tabs.add(&setting_group);

    window.end();
    window.show();

    // Window가 종료되면 프로그램도 종료하게 처리
    let _window_closed = Arc::new(std::sync::atomic::AtomicBool::new(false));

    {
        let window_closed = Arc::clone(&_window_closed);
        let event_sender = _event_sender.clone();

        window.set_callback(move |w| {
            // handle custom cleanup
            if app::event() == Event::Close {
                w.hide();

                window_closed.store(true, std::sync::atomic::Ordering::Relaxed);

                if let Err(error) = event_sender.send(ClientEvent::Exit) {
                    println!("{:?}", error);
                }
            }
        });
    }

    let window_closed = Arc::clone(&_window_closed);

    // 백그라운드 이벤트 리시버
    let _event_listner_task = tokio::spawn(async move {
        let (_stream, handle) = rodiogaga::OutputStream::try_default().unwrap();
        let sink = rodiogaga::Sink::try_new(&handle).unwrap();

        loop {
            if window_closed.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }

            if let Ok(event) = event_receiver.recv() {
                println!("@ event: {:?}, {}, {}", event, sink.is_paused(), sink.len());

                match event {
                    ClientEvent::Start => {
                        if let Ok(mut state) = state.lock() {
                            if let Some(source) = state.get_current_source() {
                                sink.append(source);
                                state.status = MusicPlayStatus::Playing;
                            }
                        }
                    }
                    ClientEvent::Resume => {
                        if let Ok(mut state) = state.lock() {
                            sink.play();
                            state.status = MusicPlayStatus::Playing;
                        }
                    }
                    ClientEvent::Stop => {
                        if let Ok(mut state) = state.lock() {
                            sink.pause();
                            state.status = MusicPlayStatus::Paused;
                        }
                    }
                    ClientEvent::Left => {
                        if let Ok(mut state) = state.lock() {
                            let status = state.status.to_owned();

                            // 이전 곡 재생
                            if let MusicPlayStatus::Playing = status {
                                sink.stop();
                                while !sink.empty() {}

                                state.index_to_left();

                                if let Some(source) = state.get_current_source() {
                                    sink.append(source);
                                    sink.play();
                                }
                            }
                        }
                    }
                    ClientEvent::Right => {
                        if let Ok(mut state) = state.lock() {
                            let status = state.status.to_owned();

                            // 다음 곡 재생
                            if let MusicPlayStatus::Playing = status {
                                sink.stop();
                                while !sink.empty() {}

                                state.index_to_right();

                                if let Some(source) = state.get_current_source() {
                                    sink.append(source);
                                    sink.play();
                                }
                            }
                        }
                    }
                    ClientEvent::IntervalCheck => {
                        if let Ok(mut state) = state.lock() {
                            let status = state.status.to_owned();

                            // 기존 재생이 끝났을 경우 다음 곡 재생
                            if let MusicPlayStatus::Playing = status {
                                if sink.empty() {
                                    state.index_to_right();

                                    if let Some(source) = state.get_current_source() {
                                        sink.append(source);
                                    }
                                }
                            }
                        }
                    }
                    ClientEvent::Exit => {
                        break;
                    }
                }
            }
        }
    });

    let window_closed = Arc::clone(&_window_closed);
    let event_sender = _event_sender.clone();

    // 일정 시간 간격으로 IntervalCheck 이벤트만 발생시켜주는 간단한 태스크
    let _background_task = tokio::spawn(async move {
        if let Err(error) = event_sender.send(ClientEvent::Start) {
            println!("{:?}", error);
        }

        loop {
            println!("backgroud loop");

            if window_closed.load(std::sync::atomic::Ordering::Relaxed) {
                break;
            }

            if let Err(error) = event_sender.send(ClientEvent::IntervalCheck) {
                println!("{:?}", error);
            }

            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    });

    app.run().expect("실행 실패");
}
