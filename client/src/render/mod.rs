mod color;
mod draw_param;
mod render_log;

pub use color::Color;
pub use draw_param::DrawParam;
pub use render_log::RenderLog;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self) -> RenderLog {
        RenderLog::new()
    }
}
