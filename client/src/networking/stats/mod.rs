mod bps;
mod rtt;

#[derive(Clone)]
pub struct NetworkStats {
    rtt: rtt::Rtt,
    bps: bps::Bps,
}

impl NetworkStats {
    pub fn new() -> Self {
        Self {
            rtt: rtt::Rtt::new(),
            bps: bps::Bps::new(),
        }
    }

    pub fn update(
        &mut self,
        client: &mut shared::threading::Channel<
            shared::networking::ClientMessage,
            shared::networking::ServerMessage,
        >,
        server: &mut shared::networking::Socket<
            shared::networking::ServerMessage,
            shared::networking::ClientMessage,
        >,
    ) {
        self.update_rtt(server);
        self.bps.update();
    }

    // This can't be in rtt.update as you need the function on_msg_send and on_bytes_send
    fn update_rtt(
        &mut self,
        server: &mut shared::networking::Socket<
            shared::networking::ServerMessage,
            shared::networking::ClientMessage,
        >,
    ) {
        if self.rtt.needs_ping() {
            let msg = shared::networking::ClientMessage::Ping;
            self.on_msg_send(&msg);
            let header = server.send(msg).unwrap();
            self.on_bytes_send(&header);
        }
    }

    pub fn on_msg_recv(&mut self, msg: &shared::networking::ServerMessage) {
        if let shared::networking::ServerMessage::Pong = msg {
            if let Some(stopwatch) = &self.rtt.ping_request_stopwatch {
                self.rtt.set(stopwatch.read());
                self.rtt.ping_request_stopwatch = None;
                self.rtt.last_pong = std::time::Instant::now();
            }
        }
    }
    pub fn on_bytes_recv(&mut self, header: &shared::networking::Header) {
        self.bps.on_bytes_recv(header)
    }

    pub fn on_msg_send(&mut self, msg: &shared::networking::ClientMessage) {
        if let shared::networking::ClientMessage::Ping = msg {
            if self.rtt.ping_request_stopwatch.is_none() {
                self.rtt.ping_request_stopwatch = Some(shared::time::Stopwatch::start_new())
            }
        }
    }
    pub fn on_bytes_send(&mut self, header: &shared::networking::Header) {
        self.bps.on_bytes_send(header)
    }
}

// rtt
impl NetworkStats {
    pub fn set_rtt(&mut self, rtt: std::time::Duration) {
        self.rtt.set(rtt)
    }
    pub fn get_rtt(&self) -> std::time::Duration {
        self.rtt.get()
    }
}

//bps
impl NetworkStats {
    pub fn total_received(&self) -> usize {
        self.bps.total_received()
    }
    pub fn total_sent(&self) -> usize {
        self.bps.total_sent()
    }
    pub fn received_last_10_sec(&self) -> usize {
        self.bps.received_last_10_sec()
    }
    pub fn bps_received_last_10_sec(&self) -> usize {
        self.bps.bps_received_last_10_sec()
    }
    pub fn sent_last_10_sec(&self) -> usize {
        self.bps.sent_last_10_sec()
    }
    pub fn bps_sent_last_10_sec(&self) -> usize {
        self.bps.bps_sent_last_10_sec()
    }
}
