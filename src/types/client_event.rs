#[derive(Debug, Clone)]
pub enum ClientEvent {
    Start,
    Stop,
    Resume,
    Left,
    Right,
    IntervalCheck,
    Exit,
}
