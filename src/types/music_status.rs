#[derive(Debug, Clone)]
pub enum MusicPlayStatus {
    Stopped,   // 중단 상태. 초기 상태.
    Playing,   // 실행중
    Paused,    // 일시정지
    Completed, // 한 곡이 끝난 후 대기상태
}
