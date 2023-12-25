pub struct ResolverManager<Sprite, Sound, Font> {
    internal_sprite: Option<Resolver<Sprite>>,
    external_sprite: Option<Resolver<Sprite>>,

    internal_sound: Option<Resolver<Sound>>,
    external_sound: Option<Resolver<Sound>>,

    internal_font: Option<Resolver<Font>>,
    external_font: Option<Resolver<Font>>,
}

/// Im not sure about the type is should use here so i'll alis it until im sure
type AssetPath = String;
type AssetPathPrefix = String;

/// A resolver is a direct link to filesystem (internal or external)
/// It loads and returns the byte content of the file of a given asset
/// See shared/file.rs for more information about the different types of filesystem
/// Each resolver has it's own file in it's filesystem that it uses to create that map
struct Resolver<AssetType> {
    fs: shared::file::FileSystem,
    path_prefix: AssetPathPrefix,
    inner: std::collections::HashMap<AssetType, AssetPath>,
}

impl<AssetType: serde::de::DeserializeOwned + std::cmp::Eq + std::hash::Hash + std::fmt::Debug>
    Resolver<AssetType>
{
    /// Loads the right resolver.ron file and builds the map;
    /// Returns a ggez::GameError::ResourceLoadError if it can't find or deserialise its resolver.ron file
    fn new(path: shared::file::ConsPath, path_prefix: &str) -> ggez::GameResult<Self> {
        let fs = path.fs();
        let path_prefix = path_prefix.to_string();

        // Load the resolver file's bytes
        let bytes = match shared::file::try_bytes(path.into()) {
            Ok(bytes) => bytes,
            Err(e) => {
                error!("Could not create resolver for: {path_prefix:?}: {e}");
                return Err(e.into());
            }
        };

        // Convert the laoded bytes into a usable map
        let inner =
            match ron::de::from_bytes::<std::collections::HashMap<AssetType, String>>(&bytes) {
                Ok(resolver) => resolver,
                Err(e) => {
                    error!("Resolver could not deserialize resolver file bytes: {e}");

                    return Err(ggez::GameError::ResourceLoadError(format!("{e:?}")));
                }
            };

        Ok(Self {
            fs,
            path_prefix,
            inner,
        })
    }

    /// Same as new but ignore the error and return an option
    fn try_new(path: shared::file::ConsPath, path_prefix: &str) -> Option<Self> {
        Self::new(path, path_prefix).ok()
    }

    /// Checks if the resolver has the given asset
    fn has(&self, asset: &AssetType) -> bool {
        self.inner.contains_key(asset)
    }

    /// Tries to load the given asset
    fn try_get(
        &self,
        asset: &AssetType,
    ) -> Result<std::borrow::Cow<'static, [u8]>, std::io::Error> {
        let path = format!("{}{}", self.path_prefix, self.inner.get(asset).unwrap());
        debug!("Requesting file at {path:?} for {:?} filesystem", self.fs);
        shared::file::try_bytes(shared::file::Path::new(self.fs, path))
    }

    /// try_get but .unwrap()ed
    fn get(&self, asset: &AssetType) -> std::borrow::Cow<'static, [u8]> {
        self.try_get(asset).unwrap()
    }
}

impl<
        Sprite: serde::de::DeserializeOwned + std::cmp::Eq + std::hash::Hash + std::fmt::Debug,
        Sound: serde::de::DeserializeOwned + std::cmp::Eq + std::hash::Hash + std::fmt::Debug,
        Font: serde::de::DeserializeOwned + std::cmp::Eq + std::hash::Hash + std::fmt::Debug,
    > ResolverManager<Sprite, Sound, Font>
{
    /// Builds all the resolvers
    /// Fails if any of the resolvers fails to initialise
    pub fn new() -> ggez::GameResult<Self> {
        Ok(Self {
            internal_sprite: Resolver::<Sprite>::try_new(
                shared::file::ConsPath::new(
                    shared::file::FileSystem::Internal,
                    "sprite/resolver.ron",
                ),
                "sprite/",
            ),
            external_sprite: Resolver::<Sprite>::try_new(
                shared::file::ConsPath::new(
                    shared::file::FileSystem::External,
                    "sprite/resolver.ron",
                ),
                "sprite/",
            ),

            internal_sound: Resolver::<Sound>::try_new(
                shared::file::ConsPath::new(
                    shared::file::FileSystem::Internal,
                    "sound/resolver.ron",
                ),
                "sound/",
            ),
            external_sound: Resolver::<Sound>::try_new(
                shared::file::ConsPath::new(
                    shared::file::FileSystem::External,
                    "sound/resolver.ron",
                ),
                "sound/",
            ),
            internal_font: Resolver::<Font>::try_new(
                shared::file::ConsPath::new(
                    shared::file::FileSystem::Internal,
                    "font/resolver.ron",
                ),
                "font/",
            ),
            external_font: Resolver::<Font>::try_new(
                shared::file::ConsPath::new(
                    shared::file::FileSystem::External,
                    "font/resolver.ron",
                ),
                "font/",
            ),
        })
    }

    /// Queries the resolvers for the given asset and tries to load it
    /// Returning its bytes on sucess
    /// or None on fail
    fn get<Asset>(
        &self,
        internal_resolver: Option<&Resolver<Asset>>,
        external_resolver: Option<&Resolver<Asset>>,
        asset: &Asset,
    ) -> Option<std::borrow::Cow<'static, [u8]>>
    where
        Asset: serde::de::DeserializeOwned + std::cmp::Eq + std::hash::Hash + std::fmt::Debug,
    {
        // The implicit .unwrap of resolver.get is fine as we test before if that resolver has the asset
        // No it's not, see #36

        // About the explicit one, we check if it is Some right before
        if internal_resolver.is_some_and(|r| r.has(asset)) {
            internal_resolver.unwrap().try_get(asset).ok()
        } else if external_resolver.is_some_and(|r| r.has(asset)) {
            external_resolver.unwrap().try_get(asset).ok()
        } else {
            None
        }
    }

    /// Loads the given sprite from any of the filesystem
    pub fn get_sprite(&self, sprite: &Sprite) -> Option<std::borrow::Cow<'static, [u8]>> {
        let res = self.get(
            self.internal_sprite.as_ref(),
            self.external_sprite.as_ref(),
            sprite,
        );

        if res.is_none() {
            error!("None of the sprite resolvers have the asset: {sprite:?}");
        }
        res
    }

    /// Loads the given sound from any of the filesystem
    pub fn get_sound(&self, sound: &Sound) -> Option<std::borrow::Cow<'static, [u8]>> {
        let res = self.get(
            self.internal_sound.as_ref(),
            self.external_sound.as_ref(),
            sound,
        );

        if res.is_none() {
            error!("None of the sound resolvers have the asset: {sound:?}");
        }
        res
    }

    /// Loads the given font from any of the filesystem
    pub fn get_font(&self, font: &Font) -> Option<std::borrow::Cow<'static, [u8]>> {
        let res = self.get(
            self.internal_font.as_ref(),
            self.external_font.as_ref(),
            font,
        );

        if res.is_none() {
            error!("None of the font resolvers have the asset: {font:?}");
        }
        res
    }
}
