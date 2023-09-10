pub struct Client {
    id: shared::id::Id,
    socket: shared::networking::Socket<
        shared::networking::ClientMessage,
        shared::networking::ServerMessage,
    >,
    ip: std::net::SocketAddr,
    running: bool,
}

impl Client {
    pub fn new(
        stream: std::net::TcpStream,
        ip: std::net::SocketAddr,
        custom_id: Option<shared::id::Id>,
    ) -> Self {
        stream.set_nonblocking(true).unwrap();
        Self {
            id: custom_id.unwrap_or_else(shared::id::Id::new),
            socket: shared::networking::Socket::<
                shared::networking::ClientMessage,
                shared::networking::ServerMessage,
            >::new(stream),
            ip,
            running: true,
        }
    }
    pub fn is_running(&self) -> bool {
        self.running
    }
    pub fn update(&mut self) {
        match self.socket.try_recv() {
            Ok(msg) => {
                // trace!("Client '{id}' received the message: {msg:?}", id = self.id);
                match msg {
                    shared::networking::ClientMessage::Text(txt) => {
                        debug!("Client '{id}' received {txt}", id = self.id);
                    }
                    shared::networking::ClientMessage::Ping => {
                        // Keep in mind that even tho the struct is called 'client' it's only the client from the pov of the server.
                        // We still need to send 'ServerMessage' as from the real client, this is the server program
                        self.socket
                            .send(shared::networking::ServerMessage::Pong)
                            .unwrap();
                    }
                    shared::networking::ClientMessage::Pong => {
                        // We're not supposed to receive a pong message as there is currently no way to send a ping request.
                        // tho we might need to implement it later to make sure that the client is still conencted
                    } // we will also need to make more args in this function as currently things like game creatio
                      // request have no way to be transmitted to the Server or any Game manager struct
                }
            }
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
                    warn!("Would block");
                } else {
                    error!("Client '{id}' encountered an error: {e:?}", id = self.id);
                    self.socket.shutdown();
                    self.running = false;
                }
            }
        }
    }
}
