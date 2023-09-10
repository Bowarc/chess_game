mod handle;
mod loaded_data;
mod request;
mod resolver;
mod targets;

pub use handle::Handle;
pub use loaded_data::RawLoadedData;
pub use request::Request;
pub use resolver::ResolverManager;
pub use targets::TargetId;

pub struct Loader {
    channel: shared::threading::Channel<Request, RawLoadedData>,
    cooldown: std::time::Duration,

    resolvers: ResolverManager<super::sprite::SpriteId, super::sound::SoundId, super::font::FontId>,
}

impl Loader {
    pub fn new(channel: shared::threading::Channel<Request, RawLoadedData>) -> Self {
        Self {
            channel,
            cooldown: std::time::Duration::from_millis(10),
            resolvers: ResolverManager::<
                super::sprite::SpriteId,
                super::sound::SoundId,
                super::font::FontId,
            >::new()
            .unwrap(),
        }
    }

    pub fn run(self) {
        loop {
            spin_sleep::sleep(self.cooldown);

            match self.channel.try_recv() {
                Ok(request) => {
                    let bytes_opt = match request {
                        Request::Sound(id) => self.resolvers.get_sound(&id),
                        Request::Sprite(id) => self.resolvers.get_sprite(&id),
                        Request::Font(id) => self.resolvers.get_font(&id),
                    };

                    let bytes_opt = bytes_opt.clone();

                    if let Some(bytes) = bytes_opt {
                        match self.channel.send(RawLoadedData { request, bytes }) {
                            Ok(a) => {
                                debug!("Successfully send data for {request:?}");
                            }

                            Err(e) => {
                                error!("Asset loader could not send data, missed: {request:?}")
                            }
                        }
                    }
                }
                Err(e) => {
                    match e {
                        std::sync::mpsc::TryRecvError::Empty => {
                            // classic error, we obviously don't receive requests every frame
                        }
                        std::sync::mpsc::TryRecvError::Disconnected => {
                            error!("The loader thread encountered an unexpected error: {e:?}");

                            break;
                        }
                    }
                }
            }
        }
    }
}

pub fn init() -> Handle {
    // loader pair
    let (handle_channel, loader_channel) =
        shared::threading::Channel::<Request, RawLoadedData>::new_pair();

    let loader = Loader::new(loader_channel);

    std::thread::Builder::new()
        .name("Asset loader".to_string())
        .spawn(move || {
            debug!("Starting loader thread");

            loader.run();

            debug!("Exiting loader thread");
        })
        .unwrap();

    Handle::new(handle_channel)
}
