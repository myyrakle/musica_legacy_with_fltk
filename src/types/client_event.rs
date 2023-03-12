#[derive(Debug, Clone)]
pub enum ClientEvent {
    Start,
    StopOrResume,
    Left,
    Right,
    IntervalCheck,
}
