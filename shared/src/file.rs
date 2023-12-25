#[derive(rust_embed::RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/../resources/internal/"]
struct Resolver;

#[derive(Debug, Copy, Clone)]
pub enum FileSystem {
    Internal,
    External,
}

#[derive(Debug)]
pub struct Path {
    fs: FileSystem,
    p: String,
}

#[derive(Debug)]
pub struct ConsPath {
    fs: FileSystem,
    p: &'static str,
}

impl Path {
    pub fn new(fs: FileSystem, p: String) -> Self {
        Self { fs, p }
    }
    pub fn fs(&self) -> FileSystem {
        self.fs
    }
    pub fn p(&self) -> String {
        self.p.clone()
    }
}

impl ConsPath {
    pub const fn new(fs: FileSystem, p: &'static str) -> Self {
        Self { fs, p }
    }
    pub fn fs(&self) -> FileSystem {
        self.fs
    }
    pub fn p(&self) -> &'static str {
        self.p
    }
}

impl std::convert::From<ConsPath> for Path {
    fn from(o: ConsPath) -> Path {
        Path {
            fs: o.fs,
            p: o.p.to_string(),
        }
    }
}

impl std::ops::Add<&str> for Path {
    type Output = Self;

    fn add(self, other: &str) -> Path {
        Self {
            fs: self.fs,
            p: self.p + other,
        }
    }
}

pub fn list() {
    debug!(
        "Internal files: {:#?}",
        Resolver::iter().collect::<Vec<std::borrow::Cow<'static, str>>>()
    );
}

pub fn try_bytes(path: Path) -> Result<std::borrow::Cow<'static, [u8]>, std::io::Error> {
    let stopwatch = time::Stopwatch::start_new();
    let start_info_message = format!("Loading {:?} {}", path.fs, path.p);
    match path.fs {
        FileSystem::Internal => match Resolver::get(&path.p) {
            Some(file) => {
                debug!("{} . . success in {stopwatch}", start_info_message,);
                Ok(file.data)
            }
            None => {
                let error = std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Could not open path: {:?}, {:?}", path.fs, path.p),
                );
                error!("{} . . error: {error}", start_info_message);
                Err(error)
            }
        },
        FileSystem::External => {
            match std::fs::File::open(format!("resources/external/{}", path.p)) {
                Ok(mut file) => {
                    use std::io::Read as _;

                    let mut bytes: Vec<u8> = Vec::new();
                    let _ = file.read_to_end(&mut bytes);
                    debug!("{} . . success in {stopwatch}", start_info_message,);
                    Ok(bytes.into())
                }
                Err(e) => {
                    // format!("Could not open path: {:?}, {}", path.fs, path.p);
                    error!("{} . . error: {e}", start_info_message);
                    Err(e)
                }
            }
        }
    }
}
pub fn bytes(path: Path) -> std::borrow::Cow<'static, [u8]> {
    try_bytes(path).unwrap()
}
