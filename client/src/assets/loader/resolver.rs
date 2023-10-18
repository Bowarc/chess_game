pub struct ResolverManager<Sprite, Sound, Font> {
    internal_sprite: Resolver<Sprite>,
    external_sprite: Resolver<Sprite>,

    internal_sound: Resolver<Sound>,
    external_sound: Resolver<Sound>,

    internal_font: Resolver<Font>,
    external_font: Resolver<Font>,
}

struct Resolver<AssetType> {
    fs: shared::file::FileSystem,
    path_prefix: String,
    inner: std::collections::HashMap<AssetType, String>,
}

impl<AssetType: serde::de::DeserializeOwned + std::cmp::Eq + std::hash::Hash + std::fmt::Debug>
    Resolver<AssetType>
{
    fn new(path: shared::file::ConsPath, path_prefix: &str) -> ggez::GameResult<Self> {
        let fs = path.fs();
        let path_prefix = path_prefix.to_string();
        let bytes = match shared::file::try_bytes(path.into()) {
            Ok(bytes) => bytes,
            Err(e) => {
                // error!("Asset loader could not open resolver file path: {e}");
                return Err(e.into());
            }
        };

        let inner =
            match ron::de::from_bytes::<std::collections::HashMap<AssetType, String>>(&bytes) {
                Ok(resolver) => resolver,
                Err(e) => {
                    error!("Asset loader could not deserialize resolver file bytes: {e}");

                    return Err(ggez::GameError::ResourceLoadError(format!("{e:?}")));
                }
            };

        Ok(Self {
            fs,
            path_prefix,
            inner,
        })
    }
    fn has(&self, asset: &AssetType) -> bool {
        self.inner
            .keys()
            .collect::<Vec<&AssetType>>()
            .contains(&asset)
    }

    fn get(&self, asset: &AssetType) -> Result<std::borrow::Cow<'static, [u8]>, std::io::Error> {
        let path = format!("{}{}", self.path_prefix, self.inner.get(asset).unwrap());

        debug!("Requesting file at {path:?} for {:?} filesystem", self.fs);
        shared::file::try_bytes(shared::file::Path::new(self.fs, path))
    }
}

impl<
        Sprite: serde::de::DeserializeOwned + std::cmp::Eq + std::hash::Hash + std::fmt::Debug,
        Sound: serde::de::DeserializeOwned + std::cmp::Eq + std::hash::Hash + std::fmt::Debug,
        Font: serde::de::DeserializeOwned + std::cmp::Eq + std::hash::Hash + std::fmt::Debug,
    > ResolverManager<Sprite, Sound, Font>
{
    pub fn new() -> ggez::GameResult<Self> {
        Ok(Self {
            internal_sprite: Resolver::<Sprite>::new(
                shared::file::ConsPath::new(
                    shared::file::FileSystem::Internal,
                    "sprite\\resolver.ron",
                ),
                "sprite\\",
            )?,
            external_sprite: Resolver::<Sprite>::new(
                shared::file::ConsPath::new(
                    shared::file::FileSystem::External,
                    "sprite\\resolver.ron",
                ),
                "sprite\\",
            )?,

            internal_sound: Resolver::<Sound>::new(
                shared::file::ConsPath::new(
                    shared::file::FileSystem::Internal,
                    "sound\\resolver.ron",
                ),
                "sound\\",
            )?,
            external_sound: Resolver::<Sound>::new(
                shared::file::ConsPath::new(
                    shared::file::FileSystem::External,
                    "sound\\resolver.ron",
                ),
                "sound\\",
            )?,

            internal_font: Resolver::<Font>::new(
                shared::file::ConsPath::new(
                    shared::file::FileSystem::Internal,
                    "font\\resolver.ron",
                ),
                "font\\",
            )?,
            external_font: Resolver::<Font>::new(
                shared::file::ConsPath::new(
                    shared::file::FileSystem::External,
                    "font\\resolver.ron",
                ),
                "font\\",
            )?,
        })
    }

    pub fn get_sprite(&self, sprite: &Sprite) -> Option<std::borrow::Cow<'static, [u8]>> {
        if self.internal_sprite.has(sprite) {
            Some(self.internal_sprite.get(sprite).unwrap())
        } else if self.external_sprite.has(sprite) {
            Some(self.external_sprite.get(sprite).unwrap())
        } else {
            error!("None of the sprite resolvers have the asset: {sprite:?}");
            None
        }
    }

    pub fn get_sound(&self, sound: &Sound) -> Option<std::borrow::Cow<'static, [u8]>> {
        if self.internal_sound.has(sound) {
            Some(self.internal_sound.get(sound).unwrap())
        } else if self.external_sound.has(sound) {
            Some(self.external_sound.get(sound).unwrap())
        } else {
            error!("None of the sound resolvers have the asset: {sound:?}");
            None
        }
    }

    pub fn get_font(&self, font: &Font) -> Option<std::borrow::Cow<'static, [u8]>> {
        if self.internal_font.has(font) {
            Some(self.internal_font.get(font).unwrap())
        } else if self.external_font.has(font) {
            Some(self.external_font.get(font).unwrap())
        } else {
            error!("None of the font resolvers have the asset: {font:?}");
            None
        }
    }
}
