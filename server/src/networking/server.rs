pub struct Server<R: networking::Message, W: networking::Message> {
    clients: Vec<super::Client<R, W>>,
    listener: std::net::TcpListener,
}

impl<R: networking::Message + 'static, W: networking::Message + 'static> Server<R, W> {
    pub fn new(addr: std::net::SocketAddr) -> Self {
        let listener = std::net::TcpListener::bind(addr).unwrap();
        listener.set_nonblocking(true).unwrap();

        Self {
            clients: vec![],
            listener,
        }
    }

    pub fn clients(&mut self) -> &mut Vec<super::Client<R, W>> {
        &mut self.clients
    }
    fn accept_new_clients(&mut self) {
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
    fn clean_disconnected_clients(&mut self) {
        let old_client_len = self.clients.len();
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

        if self.clients.len() != old_client_len {
            debug!("Currently connected to {} clients", self.clients.len());
        }
    }

    pub fn update(&mut self) {
        self.accept_new_clients();
        self.clean_disconnected_clients();
    }
}
