pub struct Client {
    socket: shared::networking::Socket<
        shared::networking::ServerMessage,
        shared::networking::ClientMessage,
    >,
    ping_request_stopwatch: Option<shared::time::Stopwatch>,
    stats: super::NetworkStats,
    connected: bool,
}

impl Client {
    pub fn new(addr: std::net::SocketAddr) -> ggez::GameResult<Self> {
        let stream = std::net::TcpStream::connect(addr).unwrap();
        stream.set_nonblocking(true).unwrap();
        let socket = shared::networking::Socket::<
            shared::networking::ServerMessage,
            shared::networking::ClientMessage,
        >::new(stream);

        Ok(Self {
            socket,
            ping_request_stopwatch: None,
            stats: super::NetworkStats::new(),
            connected: true,
        })
    }

    pub fn request_ping(&mut self) {
        self.socket
            .send(shared::networking::ClientMessage::Ping)
            .unwrap();
        self.ping_request_stopwatch = Some(shared::time::Stopwatch::start_new())
    }

    fn listen_server(&mut self) -> ggez::GameResult {
        if !self.connected {
            return Err(ggez::GameError::CustomError(
                "Disconnected from server".to_string(),
            ));
        }
        match self.socket.try_recv() {
            Ok(msg) => match msg {
                shared::networking::ServerMessage::Text(txt) => {
                    debug!("Server sent '{txt}'")
                }
                shared::networking::ServerMessage::Ping => {
                    self.socket
                        .send(shared::networking::ClientMessage::Pong)
                        .unwrap();
                }
                shared::networking::ServerMessage::Pong => {
                    if let Some(stopwatch) = &self.ping_request_stopwatch {
                        let dur = stopwatch.read();

                        debug!("RTT: {}", shared::time::display_duration(dur, ""));
                        self.stats.set_rtt(dur);
                        self.ping_request_stopwatch = None;
                    }
                }
            },
            Err(e) => {
                let is_would_block =
                    if let shared::networking::SocketError::StreamRead(ref io_e) = e {
                        io_e.kind() == std::io::ErrorKind::WouldBlock
                    } else {
                        // matches!(e, shared::networking::SocketError::WouldBlock)
                        false
                    };

                if is_would_block {
                    // Not critical error
                    // warn!("Would block");
                } else {
                    error!("Client encountered an error: {e:?}");
                    self.socket.shutdown();
                    self.connected = false;
                }
            }
        }

        Ok(())
    }

    pub fn update(&mut self) -> ggez::GameResult {
        self.listen_server()?;

        if !self.stats.has_rtt() && self.ping_request_stopwatch.is_none() {
            debug!("Requested ping");
            self.request_ping();
        }

        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.socket.shutdown()
    }
}
