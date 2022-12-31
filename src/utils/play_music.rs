// use rodio::Sink;
// use std::path::PathBuf;
// use std::{collections::VecDeque, io::BufReader};

// use rodio::{source::Source, Decoder, OutputStream};

// use rand::seq::SliceRandom;
// use rand::thread_rng;

// use crate::types::file_info::FileInfo;

// use super::read_file_list;

// #[derive(Debug, Clone)]
// pub enum MusicPlayStatus {
//     Stopped,   // 중단 상태. 초기 상태.
//     Playing,   // 실행중
//     Paused,    // 일시정지
//     Completed, // 한 곡이 끝난 후 대기상태
// }

// #[derive(Debug, Clone)]
// pub struct MusicPlayer {
//     //pub sink: Sink,
//     pub status: MusicPlayStatus,
// }

// impl Default for MusicPlayer {
//     fn default() -> Self {
//         let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
//         let sink = rodio::Sink::try_new(&handle).unwrap();

//         Self {
//             //sink,
//             status: MusicPlayStatus::Stopped,

//         }
//     }
// }

// impl MusicPlayer {
//     pub fn pause(&mut self) {
//         //self.sink.pause();
//         self.status = MusicPlayStatus::Paused;
//     }

//     pub fn resume(&mut self) {
//         //self.sink.play();
//         self.status = MusicPlayStatus::Playing;
//     }

//     pub fn complete(&mut self) {
//         self.status = MusicPlayStatus::Completed;
//         self.current_file = None;
//     }

//     // 플레이를 시작합니다.
//     // 블록되기 때문에, 비동기적으로 실행하고자 한다면 별도 태스크에서 호출해야 합니다.
//     pub fn start(&mut self, file: FileInfo) {
//         self.current_file = Some(file.clone());

//         println!("{:?}", self.current_file);

//         let file = std::fs::File::open(file.filepath).unwrap();

//         let (_stream, stream_handle) = OutputStream::try_default().unwrap();
//         // Load a sound from a file, using a path relative to Cargo.toml
//         let file_reader = BufReader::new(file);
//         // Decode that sound file into a source
//         let source = Decoder::new(file_reader).unwrap();
//         // Play the sound directly on the device
//         stream_handle.play_raw(source.convert_samples());

//         // The sound plays in a separate audio thread,
//         // so we need to keep the main thread alive while it's playing.
//         std::thread::sleep(std::time::Duration::from_secs(5));

//         self.status = MusicPlayStatus::Playing;

//         println!("1");

//         // 재생 종료
//         self.complete();
//     }
// }
