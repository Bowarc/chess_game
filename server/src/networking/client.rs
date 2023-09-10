pub struct Client {
    proxy: shared::threading::Channel<
        shared::networking::ClientMessage,
        shared::networking::ServerMessage,
    >,
    pub ip: std::net::SocketAddr,
    running: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl Client {
    pub fn new(stream: std::net::TcpStream, ip: std::net::SocketAddr) -> Self {
        let (server, proxy) = shared::threading::Channel::<
            shared::networking::ClientMessage,
            shared::networking::ServerMessage,
        >::new_pair();

        let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));

        let running_thread = running.clone();
        std::thread::spawn(move || {
            super::proxy::ClientProxy::new(stream, server, running_thread).run();
        });

        Self { proxy, ip, running }
    }

    pub fn update(&mut self) -> Result<(), String> {
        if !self.is_connected() {
            return Err("Proxy is disconnected".to_string());
        }

        while let Ok(_msg) = self.proxy.try_recv() {
            // Received messages from the player's client

            // self.proxy
            //     .send(shared::networking::ServerMessage::Text(
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
