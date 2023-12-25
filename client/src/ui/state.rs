#[derive(Default, Copy, Clone)]
pub struct SpecificState {
    inner: bool,
    thisframe: bool,
}

#[derive(Default, Copy, Clone)]
pub struct State {
    hovered: SpecificState,
    clicked: SpecificState,
    focussed: SpecificState,
}

impl State {
    pub fn new_frame(&mut self) {
        self.hovered.new_frame();
        self.clicked.new_frame();
        self.focussed.new_frame();
    }

    pub fn mouse_press_self(&mut self) {
        self.hovered.set_off();
        self.clicked.set_on();
    }
    pub fn mouse_press_not_self(&mut self) {
        self.clicked.set_off();
        self.hovered.set_off();
        self.focussed.set_off();
    }

    pub fn mouse_release_self(&mut self) {
        self.hovered.set_on();
        self.clicked.set_off();
        self.focussed.set_on();
    }
    pub fn mouse_release_not_self(&mut self) {
        self.clicked.set_off();
        self.focussed.set_off();
    }

    pub fn mouse_hover_self(&mut self) {
        self.hovered.set_on();
    }
    pub fn mouse_hover_not_self(&mut self) {
        self.hovered.set_off()
    }
}

impl State {
    pub fn hovered(&self) -> bool {
        self.hovered.inner
    }
    pub fn hovered_this_frame(&self) -> bool {
        self.hovered.inner && self.hovered.thisframe
    }
    pub fn clicked(&self) -> bool {
        self.clicked.inner
    }
    pub fn clicked_this_frame(&self) -> bool {
        self.clicked.inner && self.clicked.thisframe
    }
    pub fn focussed(&self) -> bool {
        self.focussed.inner
    }
    pub fn focussed_this_frame(&self) -> bool {
        self.focussed.inner && self.focussed.thisframe
    }
}

impl SpecificState {
    fn new_frame(&mut self) {
        self.thisframe = false;
    }
    fn set_on(&mut self) {
        if !self.inner {
            self.thisframe = true;
        }
        self.inner = true;
    }
    fn set_off(&mut self) {
        if self.inner {
            self.thisframe = true
        }
        self.inner = false;
    }
}
