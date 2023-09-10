pub struct ClientProxy {
    server: shared::networking::Socket<
        shared::networking::ServerMessage,
        shared::networking::ClientMessage,
    >,
    client: shared::threading::Channel<
        shared::networking::ClientMessage,
        shared::networking::ServerMessage,
    >,
    running: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl ClientProxy {
    pub fn new(
        stream: std::net::TcpStream,
        client: shared::threading::Channel<
            shared::networking::ClientMessage,
            shared::networking::ServerMessage,
        >,
        running: std::sync::Arc<std::sync::atomic::AtomicBool>,
    ) -> Self {
        Self {
            server: shared::networking::Socket::<
                shared::networking::ServerMessage,
                shared::networking::ClientMessage,
            >::new(stream),
            client,
            running,
        }
    }

    pub fn run(&mut self) {
        let mut loop_helper = spin_sleep::LoopHelper::builder()
            .report_interval_s(0.5)
            .build_with_target_rate(10.);

        while self.running.load(std::sync::atomic::Ordering::Relaxed) {
            loop_helper.loop_start();

            if let Err(e) = self.handle_client() {
                warn!("Proxy encountered an error while handling client {e:?}");
                self.running
                    .store(false, std::sync::atomic::Ordering::Relaxed);
            }

            if let Err(e) = self.handle_server() {
                warn!("Proxy encountered an error while handling server {e:?}");
                self.running
                    .store(false, std::sync::atomic::Ordering::Relaxed);
            }

            loop_helper.loop_sleep();
        }

        self.server.shutdown();

        self.running
            .store(false, std::sync::atomic::Ordering::Relaxed);
        debug!(
            "Client proxy for ({}) has exited",
            self.server.remote_addr()
        );
    }
    fn handle_client(&mut self) -> Result<(), super::NetworkError> {
        // here you receive the message sent by the client

        if let Ok(msg) = self.client.try_recv() {
            if let Err(e) = self.server.send(msg) {
                error!(
                    "Proxy encountered an error while forwarding a message to the server: {e:?}"
                );
                Err(e).map_err(|e| super::NetworkError::ChannelSend(format!("{e:?}")))?
            }
        }

        Ok(())
    }

    fn handle_server(&mut self) -> Result<(), super::NetworkError> {
        // here you receive message sent by the client
        match self.server.try_recv() {
            Ok(msg) => {
                //
                self.client
                    .send(msg)
                    .map_err(|e| super::NetworkError::ChannelSend(format!("{e:?}")))?
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

                    // The error might just be that the server disconnected
                    if let shared::networking::SocketError::StreamRead(ref io_e) = e {
                        if io_e.kind() == std::io::ErrorKind::ConnectionReset {
                            warn!("Server {ip} disconnected", ip = self.server.remote_addr());
                        }
                    } else {
                        error!(
                            "Error while listening server {}, aborting: {e}",
                            self.server.remote_addr()
                        );
                    }

                    Err(e)?
                }
            }
        }

        Ok(())
    }
}
