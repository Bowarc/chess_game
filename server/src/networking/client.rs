pub struct Client<R: networking::Message, W: networking::Message> {
    proxy: threading::Channel<R, W>,
    pub ip: std::net::SocketAddr,
    running: std::sync::Arc<std::sync::atomic::AtomicBool>,
    stats: triple_buffer::Output<networking::NetworkStats<R, W>>,
}

impl<R: networking::Message + 'static, W: networking::Message + 'static> Client<R, W> {
    pub fn new(stream: std::net::TcpStream, ip: std::net::SocketAddr) -> Self {
        let (server, proxy) = threading::Channel::<R, W>::new_pair();

        let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));

        let (mut input, mut output) =
            triple_buffer::TripleBuffer::new(&networking::NetworkStats::<R, W>::default()).split();

        let running_thread = running.clone();
        std::thread::spawn(move || {
            networking::Proxy::new(stream, server, running_thread, input).run();
        });

        Self {
            proxy,
            ip,
            running,
            stats: output,
        }
    }

    pub fn update(&mut self) -> Result<(), String> {
        if !self.is_connected() {
            return Err("Proxy is disconnected".to_string());
        }

        while let Ok(_msg) = self.proxy.try_recv() {
            // Received messages from the player's client

            // self.proxy
            //     .send(shared::message::ServerMessage::Text(
            //         "Test message".to_string(),
            //     ))
            //     .map_err(|e| format!("{e:?}"))?;
        }

        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::Relaxed)
    }
}
