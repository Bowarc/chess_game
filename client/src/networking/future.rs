pub struct Future<T> {
    validator: fn(&shared::message::ServerMessage) -> bool,
    extractor: fn(shared::message::ServerMessage) -> Option<T>,
    inner: Option<T>,
    request_msg: shared::message::ClientMessage,
    requested: bool,
    changed: bool,
}

impl<T> Future<T> {
    pub fn new(
        request_msg: shared::message::ClientMessage,
        validator: fn(&shared::message::ServerMessage) -> bool,
        extractor: fn(shared::message::ServerMessage) -> Option<T>,
    ) -> Self {
        Self {
            validator,
            extractor,
            inner: None,
            request_msg,
            requested: false,
            changed: false,
        }

        // maybe do it with functions ?
        // I mean.. to construct the data or send the message ?
        // like validate where the user is prompted withthe given value and tells if it's good or not ?

        // idfk
    }

    pub fn inner(&self) -> Option<&T> {
        self.inner.as_ref()
    }

    pub fn inner_mut(&mut self) -> Option<&mut T> {
        self.inner.as_mut()
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    /// Force request of data
    pub fn request(
        &mut self,
        client: &mut super::Client<shared::message::ServerMessage, shared::message::ClientMessage>,
    ) -> Result<(), std::sync::mpsc::SendError<shared::message::ClientMessage>> {
        match client.send(self.request_msg.clone()) {
            Ok(_) => {
                self.requested = true;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn update(
        &mut self,
        client: &mut super::Client<shared::message::ServerMessage, shared::message::ClientMessage>,
    ) {
        self.changed = false;

        if let Some(index) = client
            .received_msg()
            .iter()
            .position(|msg| (self.validator)(msg))
        {
            let msg = client.received_msg_mut().remove(index);

            if let Some(extracted) = (self.extractor)(msg) {
                self.inner = Some(extracted);
                debug!(
                    "Future for request: {:?} has received it's data",
                    self.request_msg
                );
            } else {
                error!(
                    "Future for request {:?} failled to unpack its data",
                    self.request_msg
                )
            }

            self.requested = false;
            self.changed = true;
        }

        if self.inner.is_none() && !self.requested {
            if let Err(e) = client.send(self.request_msg.clone()) {
                error!(
                    "Future could not send request message: {:?}, {e}",
                    self.request_msg
                )
            } else {
                self.requested = true;
            }
        }
    }
}
