pub struct Server {
    clients: Vec<super::Client>,
    tcp_listener: std::net::TcpListener,
}

impl Server {
    pub fn start_new(addr: std::net::SocketAddr) -> Self {
        let listener = std::net::TcpListener::bind(addr).unwrap();
        listener.set_nonblocking(true).unwrap();

        Self {
            clients: Vec::new(),
            tcp_listener: listener,
        }
    }

    pub fn accept_new_clients(&mut self) {
        match self.tcp_listener.accept() {
            Ok((stream, addr)) => {
                debug!("New client {addr:?}");
                // stream.set_nodelay(true).unwrap(); // ?

                self.clients
                    .push(super::client::Client::new(stream, addr, None));
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP
                // println!("Would block");
                // continue;

                // About this part, as the implementation is non-blocking,
                // i'll assume that the program will do some other job before getting back to this part,
                // therefore the socket will have time to do it's things
            }

            Err(e) => {
                error!("Error while listening for clients: {e:?}");
            }
        }
    }

    fn update_current_clients(&mut self) {
        for client in &mut self.clients {
            client.update();
        }
        self.clients.retain(|client| client.is_running())
    }

    pub fn update(&mut self) {
        self.accept_new_clients();
        self.update_current_clients();
    }
}
