pub struct NetworkStats {
    latest_rtt: Option<std::time::Duration>,
}

impl NetworkStats {
    pub fn new() -> Self {
        Self { latest_rtt: None }
    }

    pub fn set_rtt(&mut self, rtt: std::time::Duration) {
        self.latest_rtt = Some(rtt)
    }
    pub fn clear_rtt(&mut self) {
        self.latest_rtt = None
    }
    pub fn has_rtt(&self) -> bool {
        self.latest_rtt.is_some()
    }
}
