pub struct Client<R: networking::Message, W: networking::Message> {
    proxy: threading::Channel<R, W>,
    pub ip: std::net::SocketAddr,
    running: std::sync::Arc<std::sync::atomic::AtomicBool>,
    stats: triple_buffer::Output<networking::NetworkStats<R, W>>,
    id: shared::id::Id,
}

impl<R: networking::Message + 'static, W: networking::Message + 'static> Client<R, W> {
    pub fn new(stream: std::net::TcpStream, ip: std::net::SocketAddr) -> Self {
        let (server, proxy) = threading::Channel::<R, W>::new_pair();

        let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));

        let (input, output) =
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
            id: shared::id::Id::new(),
        }
    }
    pub fn id(&self) -> shared::id::Id{
        self.id
    }

    pub fn try_recv(&mut self) -> Result<R, std::sync::mpsc::TryRecvError>{
        self.proxy.try_recv()
    }
    pub fn update(&mut self) -> Result<(), shared::error::server::ServerError> {
        if !self.is_connected() {
            return Err(shared::error::server::ServerError::Client(shared::error::server::ClientError::ProxyDisconnected(self.ip)));
        }

        while let Ok(_msg) = self.try_recv() {
            // Received messages from the player's client

            // self.proxy
            //     .send(shared::message::ServerMessage::Text(
            //         "Test message".to_string(),
            //     ))
            //     .map_err(|e| format!("{e:?}"))?;
            warn!("Unhandled messgae from a client: {:?}", _msg);
        }

        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::Relaxed)
    }
}
