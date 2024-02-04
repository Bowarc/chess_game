pub struct Client<R: networking::Message, W: networking::Message> {
    controller: networking::proxy::ProxyController<R, W>,
    ip: std::net::SocketAddr,
    received_msg: Vec<R>,
}

impl<R: networking::Message + 'static, W: networking::Message + 'static> Client<R, W> {
    pub fn new(addr: std::net::SocketAddr) -> ggez::GameResult<Self> {
        let cfg = networking::proxy::ProxyConfig {
            addr,
            run_tps: 1_000,
            stat_cfg: networking::stats::StatConfig {
                bps: networking::stats::config::BpsConfig { enabled: true },
                rtt: networking::stats::config::RttConfig {
                    enabled: true,
                    ping_request_delay: std::time::Duration::from_secs_f32(0.5),
                },
            },
            keep_msg_while_disconnected: false,
            auto_reconnect: true,
        };

        let controller = networking::Proxy::start_new(cfg, None);

        Ok(Self {
            controller,
            ip: addr,
            received_msg: Vec::new(),
        })
    }
    pub fn ip(&self) -> &std::net::SocketAddr {
        &self.ip
    }

    pub fn stats(&mut self) -> &networking::NetworkStats<R, W> {
        // needs mutable as it updates before reading
        self.controller.stats()
    }
    pub fn received_msg_mut(&mut self) -> &mut Vec<R> {
        &mut self.received_msg
    }

    pub fn received_msg(&self) -> &Vec<R> {
        &self.received_msg
    }

    pub fn update(&mut self) -> Result<(), String> {
        if !self.is_running() {
            return Err("Proxy is not running anymore".to_string());
        }

        while let Ok(pmsg) = self.controller.try_recv() {
            match pmsg {
                networking::proxy::ProxyMessage::Forward(msg) => self.received_msg.push(msg),
                networking::proxy::ProxyMessage::ConnectionResetError => {
                    warn!("Proxy's connection has been reset");
                    return Err("ConnectionResetError".to_string());
                }
                networking::proxy::ProxyMessage::Exit => {
                    error!("Proxy's thread exited.");
                    return Err("Proxy's thread exited".to_string());
                }
            }
        }

        Ok(())
    }

    pub fn send(&mut self, msg: W) -> Result<(), std::sync::mpsc::SendError<W>> {
        self.controller.send(msg)
    }

    pub fn is_connected(&self) -> bool {
        self.controller.is_connected()
    }
    pub fn is_running(&self) -> bool {
        self.controller.is_running()
    }

    pub fn request_ping(&mut self) -> Result<(), std::sync::mpsc::SendError<W>> {
        self.send(W::default_ping())
    }
}
