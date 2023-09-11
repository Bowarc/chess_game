pub struct Server {
    clients: Vec<super::Client>,
    listener: std::net::TcpListener,
}

impl Server {
    pub fn new(addr: std::net::SocketAddr) -> Self {
        let listener = std::net::TcpListener::bind(addr).unwrap();
        listener.set_nonblocking(true).unwrap();

        Self {
            clients: vec![],
            listener,
        }
    }
    pub fn update(&mut self) {
        // debug!(
        //     "Connected accounts: {:?}",
        //     self.account_manager.connected_accounts
        // );

        self.clients.retain_mut(|handle| {
            // trace!("updating ({})", handle.ip);
            if let Err(e) = handle.update() {
                error!(
                    "An error occured while updating client handle ({}) {e}, closing the handle",
                    handle.ip
                );
                false
            } else {
                true
            }
        });

        match self.listener.accept() {
            Ok((stream, addr)) => {
                debug!("New client {addr:?}");
                // stream.set_nodelay(true).unwrap(); // ?

                self.clients.push(super::Client::new(stream, addr));
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

    fn update_client_handles(&mut self) {}
}
