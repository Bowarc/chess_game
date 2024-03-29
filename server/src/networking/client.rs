pub struct Client<R: networking::Message, W: networking::Message> {
    controller: networking::proxy::ProxyController<R, W>,
    addr: std::net::SocketAddr,
    id: shared::id::Id,
}

impl<R: networking::Message + 'static, W: networking::Message + 'static> Client<R, W> {
    pub fn new(stream: std::net::TcpStream, addr: std::net::SocketAddr) -> Self {
        let cfg = networking::proxy::ProxyConfig {
            addr,
            run_tps: 1_000,
            stat_cfg: networking::stats::StatConfig {
                bps: networking::stats::config::BpsConfig { enabled: false },
                rtt: networking::stats::config::RttConfig {
                    enabled: true,
                    ping_request_delay: std::time::Duration::from_secs(10),
                },
            },
            keep_msg_while_disconnected: false,
            // DO NOT SET THAT TO TRUE, a from the server's pov, a client disconnecting is the end of that client
            // Clients do not have TcpListeners, this would never work anyway
            auto_reconnect: false,
        };

        let controller = networking::Proxy::start_new(cfg, Some(stream));

        Self {
            controller,
            addr,
            id: shared::id::Id::new(),
        }
    }
    pub fn addr(&self) -> std::net::SocketAddr {
        self.addr
    }
    pub fn id(&self) -> shared::id::Id {
        self.id
    }

    pub fn try_recv(&mut self) -> Result<R, std::sync::mpsc::TryRecvError> {
        match self.controller.try_recv()? {
            networking::proxy::ProxyMessage::Forward(msg) => Ok(msg),
            networking::proxy::ProxyMessage::ConnectionResetError => {
                Err(std::sync::mpsc::TryRecvError::Disconnected)
            }
            networking::proxy::ProxyMessage::Exit => {
                error!("Proxy for client {addr} has stopped.", addr = self.addr);

                Err(std::sync::mpsc::TryRecvError::Disconnected)
            }
        }
    }
    pub fn send(&mut self, msg: W) -> Result<(), std::sync::mpsc::SendError<W>> {
        self.controller.send(msg)
    }
    pub fn update(&mut self) -> Result<(), shared::error::server::ServerError> {
        if !self.is_connected() {
            return Err(shared::error::server::ServerError::Client(
                shared::error::server::ClientError::ProxyDisconnected(self.addr),
            ));
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
        self.controller.is_connected()
    }
    pub fn is_running(&self) -> bool {
        self.controller.is_running()
    }
}
