pub struct Client<R: networking::Message, W: networking::Message> {
    proxy: threading::Channel<R, W>,
    ip: std::net::SocketAddr,
    running: std::sync::Arc<std::sync::atomic::AtomicBool>,
    stats: triple_buffer::Output<networking::NetworkStats<R, W>>,
    received_msg: Vec<R>,
}

impl<R: networking::Message + 'static, W: networking::Message + 'static> Client<R, W> {
    pub fn new(addr: std::net::SocketAddr) -> ggez::GameResult<Self> {
        let Ok(stream) = std::net::TcpStream::connect(shared::DEFAULT_ADDRESS) else {
            return Err(ggez::GameError::CustomError(format!(
                "Could not establish a connection with the server at ({})",
                shared::DEFAULT_ADDRESS
            )));
        };
        stream.set_nonblocking(true).unwrap();

        let (server, proxy) = threading::Channel::<R, W>::new_pair();

        let s = networking::NetworkStats::<R, W>::default();
        let (stats_in, stats_out) = triple_buffer::triple_buffer(&s);

        let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));

        let running_thread = running.clone();
        std::thread::spawn(move || {
            networking::Proxy::new(stream, server, running_thread, stats_in).run();
        });

        Ok(Self {
            proxy,
            ip: addr,
            running,
            stats: stats_out,
            received_msg: Vec::new(),
        })
    }
    pub fn ip(&self) -> &std::net::SocketAddr {
        &self.ip
    }

    pub fn stats(&mut self) -> &networking::NetworkStats<R, W> {
        // needs mutable as it updates before reading
        self.stats.read()
    }
    pub fn received_msg_mut(&mut self) -> &mut Vec<R> {
        &mut self.received_msg
    }

    pub fn received_msg(&self) -> &Vec<R> {
        &self.received_msg
    }

    pub fn update(&mut self) -> Result<(), String> {
        if !self.is_connected() {
            return Err("Proxy is disconnected".to_string());
        }

        while let Ok(msg) = self.proxy.try_recv() {
            self.received_msg.push(msg)
        }

        Ok(())
    }

    pub fn send(&mut self, msg: W) -> Result<(), std::sync::mpsc::SendError<W>> {
        self.proxy.send(msg)
    }

    pub fn is_connected(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn request_ping(&mut self) -> Result<(), std::sync::mpsc::SendError<W>> {
        self.send(W::default_ping())?;
        Ok(())
    }
}
