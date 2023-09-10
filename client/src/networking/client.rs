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
    ping_request_stopwatch: Option<shared::time::Stopwatch>,
    stats: super::NetworkStats,
}

impl Client {
    pub fn new(addr: std::net::SocketAddr) -> Self {
        let stream = std::net::TcpStream::connect(shared::networking::DEFAULT_ADDRESS).unwrap();
        stream.set_nonblocking(true).unwrap();

        let (server, proxy) = shared::threading::Channel::<
            shared::networking::ServerMessage,
            shared::networking::ClientMessage,
        >::new_pair();

        let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));

        let running_thread = running.clone();
        std::thread::spawn(move || {
            super::proxy::ClientProxy::new(stream, server, running_thread).run();
        });

        Self {
            proxy,
            ip: addr,
            running,
            ping_request_stopwatch: None,
            stats: super::NetworkStats::new(),
        }
    }

    pub fn update(&mut self) -> Result<(), String> {
        if !self.is_connected() {
            return Err("Proxy is disconnected".to_string());
        }

        if !self.stats.has_rtt() {
            self.request_ping().unwrap();
        }

        while let Ok(msg) = self.proxy.try_recv() {
            match msg {
                shared::networking::ServerMessage::Pong => {
                    if let Some(stopwatch) = &self.ping_request_stopwatch {
                        self.stats.set_rtt(stopwatch.read());
                        self.ping_request_stopwatch = None
                    }
                }
                _ => {
                    warn!("Unhandled server message: {msg:?}");
                }
            }

            // self.proxy
            //     .send(shared::networking::ClientMessafge::Text(
            //         "Test message".to_string(),
            //     ))
            //     .map_err(|e| format!("{e:?}"))?;
        }

        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn request_ping(
        &mut self,
    ) -> Result<(), std::sync::mpsc::SendError<shared::networking::ClientMessage>> {
        self.proxy.send(shared::networking::ClientMessage::Ping)?;
        self.ping_request_stopwatch = Some(shared::time::Stopwatch::start_new());
        Ok(())
    }
}
