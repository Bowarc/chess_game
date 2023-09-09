#[derive(PartialEq)]
pub struct RawLoadedData {
    pub request: super::Request,
    pub bytes: std::borrow::Cow<'static, [u8]>,
}
