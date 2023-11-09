
pub struct Future<T>{
    validator: fn(&shared::message::ServerMessage) -> bool,
    extractor: fn(shared::message::ServerMessage) ->T,
    inner: Option<T>,
    request_msg: shared::message::ClientMessage,
    requested: bool,
}


impl<T> Future<T>{
    pub fn new(
        request_msg: shared::message::ClientMessage,
        validator: fn(&shared::message::ServerMessage) -> bool,
        extractor: fn(shared::message::ServerMessage) -> T,
    ) -> Self{
        Self{
            validator,
            extractor, 
            inner: None,
            request_msg,
            requested: false,
        }

        // maybe do it with functions ?
        // I mean.. to construct the data or send the message ?
        // like validate where the user is prompted withthe given value and tells if it's good or not ?

        // idfk
    }

    pub fn inner(&mut self,) -> Option<&mut T>{
        self.inner.as_mut()
    }

    pub fn update(&mut self, mut client: super::Client<shared::message::ServerMessage, shared::message::ClientMessage>) {
        if self.requested{
            if let Some(index) = client.received_msg_mut().iter().enumerate().flat_map(|(i, msg)|{
                if (self.validator)(msg){
                    Some(i)
                }else{
                    None
                }
            }).collect::<Vec<usize>>().first(){
                let msg = client.received_msg_mut().remove(*index);

                self.inner = Some((self.extractor)(msg));
                self.requested = false
            }
        }else if self.inner.is_none(){
            if let Err(e) = client.send(self.request_msg.clone()){
                error!("Future could not send request message: {:?}, {e}", self.request_msg)
            }else{
                self.requested = true
            }
        }
    }
}