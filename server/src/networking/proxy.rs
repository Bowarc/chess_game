pub struct ClientProxy {
    client: shared::networking::Socket<
        shared::networking::ClientMessage,
        shared::networking::ServerMessage,
    >,
    server: shared::threading::Channel<
        shared::networking::ServerMessage,
        shared::networking::ClientMessage,
    >,
    running: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl ClientProxy {
    pub fn new(
        stream: std::net::TcpStream,
        server: shared::threading::Channel<
            shared::networking::ServerMessage,
            shared::networking::ClientMessage,
        >,
        running: std::sync::Arc<std::sync::atomic::AtomicBool>,
    ) -> Self {
        Self {
            client: shared::networking::Socket::<
                shared::networking::ClientMessage,
                shared::networking::ServerMessage,
            >::new(stream),
            server,
            running,
        }
    }

    pub fn run(&mut self) {
        let mut loop_helper = spin_sleep::LoopHelper::builder()
            .report_interval_s(0.5)
            .build_with_target_rate(10000.);

        while self.running.load(std::sync::atomic::Ordering::Relaxed) {
            loop_helper.loop_start();

            if let Err(e) = self.handle_server() {
                error!("{e}");

                self.running
                    .store(false, std::sync::atomic::Ordering::Relaxed)
            }

            if let Err(e) = self.handle_client() {
                error!("{e}");
                self.running
                    .store(false, std::sync::atomic::Ordering::Relaxed)
            }

            loop_helper.loop_sleep();
        }

        self.client.shutdown();

        debug!(
            "Client proxy for ({}) has exited",
            self.client.remote_addr()
        );
    }
    fn handle_server(&mut self) -> Result<(), super::error::NetworkError> {
        // here you receive the message sent by the server

        if let Ok(msg) = self.server.try_recv() {
            if let Err(e) = self.client.send(msg) {
                error!("Proxy {ip} encountered an error while forwarding a message to the client: {e:?}", ip = self.client.remote_addr());
                Err(e).map_err(|e| super::NetworkError::ChannelSend(format!("{e:?}")))?
            }
        }

        Ok(())
    }

    fn handle_client(&mut self) -> Result<(), super::error::NetworkError> {
        // here you receive message sent by the client
        match self.client.try_recv() {
            Ok((r_header, msg)) => {
                if let shared::networking::ClientMessage::Ping = msg {
                    let s_header = self.client.send(shared::networking::ServerMessage::Pong)?;
                }

                // Forward the message to the server
                if let Err(e) = self.server.send(msg) {
                    error!("Proxy {ip} encountered an error while forwarding a message to the server: {e:?}", ip = self.client.remote_addr());
                    Err(e).map_err(|e| super::NetworkError::ChannelSend(format!("{e:?}")))?
                }
            }
            Err(e) => {
                // Check if the error is from the fact that the proxy's stream is non_bocking
                let is_would_block =
                    if let shared::networking::SocketError::StreamRead(ref io_e) = e {
                        io_e.kind() == std::io::ErrorKind::WouldBlock
                    } else {
                        // matches!(e, shared::networking::SocketError::WouldBlock)

                        false
                    };

                //if it's not from that.. it's a real error
                if !is_would_block {
                    self.running
                        .store(false, std::sync::atomic::Ordering::Relaxed);

                    // The error might just be that the client disconnected
                    if let shared::networking::SocketError::StreamRead(ref io_e) = e {
                        if io_e.kind() == std::io::ErrorKind::ConnectionReset {
                            warn!("Client {ip} disconnected", ip = self.client.remote_addr());
                        }
                    } else {
                        error!(
                            "Error while listening client {}, aborting: {e}",
                            self.client.remote_addr()
                        );
                    }
                    Err(e)?
                }
            }
        }
        Ok(())
    }
}
