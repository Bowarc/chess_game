pub struct Handle {
    // This struct is used to make requests to the loader thead
    channel: threading::Channel<super::RawLoadedData, super::Request>,
    ongoing: Vec<super::Request>,
    received_data: hashbrown::HashMap<super::TargetId, Vec<super::RawLoadedData>>,
}

impl Handle {
    pub fn new(channel: threading::Channel<super::RawLoadedData, super::Request>) -> Self {
        Self {
            channel,
            ongoing: Vec::new(),
            received_data: hashbrown::HashMap::new(),
        }
    }
    pub fn ongoing_requests(&self) -> &[super::Request] {
        &self.ongoing
    }
    pub fn request(&mut self, req: super::Request) {
        if self.ongoing.contains(&req) {
            // warn!("The request for {req:?} is already currently in process");
            return;
        }

        match self.channel.send(req) {
            Ok(_) => {
                self.ongoing.push(req);
                debug!("Requested asset: {req:?}")
            }
            Err(e) => {
                error!("Asset loader got an error while requesting {req:?}\n{e:?}");
            }
        }
    }
    fn remove_ongoing(&mut self, req: &super::Request) {
        if let Some(index) = self.ongoing.iter().position(|r| r == req) {
            self.ongoing.swap_remove(index);
        } else {
            warn!(
                "Asset loader handle seceived a message but it wasn't in the ongoing list: {req:?}"
            )
        }
    }
    fn save_received_data(&mut self, target_id: super::TargetId, data: super::RawLoadedData) {
        self.received_data
            .entry(target_id)
            .or_insert_with(Vec::default);

        self.received_data.get_mut(&target_id).unwrap().push(data);
    }

    /// I removed the .recv function koz this one is the only one use and usefull
    fn try_recv(&mut self) -> Result<super::RawLoadedData, std::sync::mpsc::TryRecvError> {
        self.channel.try_recv()
    }

    pub fn update(&mut self) {
        while let Ok(data) = self.try_recv() {
            match data.request {
                super::Request::Sound(_snd_id) => {
                    // debug!("Asset loader Handle received data for sound: {snd_id:?}, saving.");
                    self.save_received_data(super::TargetId::Sound, data);
                }
                super::Request::Sprite(_spr_id) => {
                    // debug!("Asset loader Handle received data for sprite: {spr_id:?}, saving.");
                    self.save_received_data(super::TargetId::Sprite, data);
                }
                super::Request::Font(_fnt_id) => {
                    // debug!("Asset loader Handle received data for font: {_fnt_id:?}, saving.");
                    self.save_received_data(super::TargetId::Font, data);
                }
            }
        }
    }
    pub fn retrieve_data(&mut self, target: super::TargetId) -> Option<super::RawLoadedData> {
        let Some(vec_data) = self.received_data.get_mut(&target) else {
            return None;
        };

        let Some(data) = vec_data.pop() else {
            return None;
        };

        // This is used to try to eliminate a specific case where asset are loaded multiple times
        self.remove_ongoing(&data.request);

        Some(data)
    }
}
