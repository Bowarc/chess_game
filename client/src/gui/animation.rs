#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum AnimationState {
    Open,
    Close,
    Opening,
    Closing,
}

pub struct WindowAnimation {
    pub opening_time: f32,
    pub closing_time: f32,
    pub state: AnimationState,
    pub start_time: std::time::Instant,
}

pub struct WindowVisibility {
    pub last_switch: std::time::Instant,
    pub switch_delay: i32,
}
