pub struct Client<R: networking::Message, W: networking::Message> {
    proxy: threading::Channel<networking::proxy::ProxyMessage<R>, W>,
    ip: std::net::SocketAddr,
    running: std::sync::Arc<std::sync::atomic::AtomicBool>,
    connected: std::sync::Arc<std::sync::atomic::AtomicBool>,
    stats: triple_buffer::Output<networking::NetworkStats<R, W>>,
    proxy_thread_handle: std::thread::JoinHandle<()>,
    received_msg: Vec<R>,
}

impl<R: networking::Message + 'static, W: networking::Message + 'static> Client<R, W> {
    pub fn new(addr: std::net::SocketAddr) -> ggez::GameResult<Self> {
        let cfg = networking::proxy::ProxyConfig {
            addr,
            run_tps: 10_0000,
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

        let networking::proxy::ProxyOutput {
            stats,
            channel,
            running,
            connected,
            thread_handle,
        } = networking::Proxy::start_new(cfg, None);

        Ok(Self {
            proxy: channel,
            ip: addr,
            running,
            connected,
            stats,
            proxy_thread_handle: thread_handle,
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
        if !self.is_running() {
            return Err("Proxy is not running anymore".to_string());
        }

        while let Ok(pmsg) = self.proxy.try_recv() {
            match pmsg {
                networking::proxy::ProxyMessage::Forward(msg) => self.received_msg.push(msg),
                networking::proxy::ProxyMessage::ConnectionResetError => {
                    warn!("Proxy's connection has been reset");
                }
                networking::proxy::ProxyMessage::Exit => {
                    error!("Proxy's thread exited.")
                }
            }
        }

        Ok(())
    }

    pub fn send(&mut self, msg: W) -> Result<(), std::sync::mpsc::SendError<W>> {
        self.proxy.send(msg)
    }

    pub fn is_connected(&self) -> bool {
        self.connected.load(std::sync::atomic::Ordering::Relaxed)
    }
    pub fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn request_ping(&mut self) -> Result<(), std::sync::mpsc::SendError<W>> {
        self.send(W::default_ping())?;
        Ok(())
    }
}
