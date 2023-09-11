#[derive(PartialEq)]
enum Message {
    Text(String),
}

pub struct Client {
    proxy: shared::threading::Channel<
        shared::networking::ServerMessage,
        shared::networking::ClientMessage,
    >,
    pub ip: std::net::SocketAddr,
    running: std::sync::Arc<std::sync::atomic::AtomicBool>,
    stats: triple_buffer::Output<super::NetworkStats>,
}

impl Client {
    pub fn new(addr: std::net::SocketAddr) -> Self {
        let stream = std::net::TcpStream::connect(shared::networking::DEFAULT_ADDRESS).unwrap();
        stream.set_nonblocking(true).unwrap();

        let (server, proxy) = shared::threading::Channel::<
            shared::networking::ServerMessage,
            shared::networking::ClientMessage,
        >::new_pair();

        let (stats_in, stats_out) = triple_buffer::triple_buffer(&super::NetworkStats::new());

        let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));

        let running_thread = running.clone();
        std::thread::spawn(move || {
            super::proxy::ClientProxy::new(stream, server, running_thread, stats_in).run();
        });

        Self {
            proxy,
            ip: addr,
            running,
            stats: stats_out,
        }
    }

    pub fn stats(&mut self) -> &super::NetworkStats {
        // needs mutable as it updates before reading
        self.stats.read()
    }

    pub fn update(&mut self) -> Result<(), String> {
        if !self.is_connected() {
            return Err("Proxy is disconnected".to_string());
        }

        while let Ok(msg) = self.proxy.try_recv() {
            match &msg {
                _ => {
                    warn!("Unhandled server message: {msg:?}");
                }
            }
        }
        Ok(())
    }

    pub fn send(
        &mut self,
        msg: shared::networking::ClientMessage,
    ) -> Result<(), super::NetworkError> {
        self.proxy
            .send(msg)
            .map_err(|e| super::NetworkError::ChannelSend(format!("{e:?}")))
    }

    pub fn is_connected(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn request_ping(&mut self) -> Result<(), super::NetworkError> {
        self.send(shared::networking::ClientMessage::Ping)?;
        Ok(())
    }
}
