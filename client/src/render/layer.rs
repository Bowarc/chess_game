// We could switch out the EnumIter derive to a simple array
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)] //EnumIter
pub enum Layer {
    FarBack,        // something that will almost never be seen like a background color etc
    GameBackground, // the background part of the game, like the map, etc...
    Game,           // the actual game
    GameForeground, // some effects over the game,
    UiBackground,   // behind the ui,
    Ui,             // the UI itself
    UiForeground,   // in front of the ui
    TopMost,        // Things above all, like cursor
}

impl Layer {
    pub fn get(idx: u8) -> Option<Self> {
        match idx {
            0 => Some(Layer::FarBack),
            1 => Some(Layer::GameBackground),
            2 => Some(Layer::Game),
            3 => Some(Layer::GameForeground),
            4 => Some(Layer::UiBackground),
            5 => Some(Layer::Ui),
            6 => Some(Layer::UiForeground),
            7 => Some(Layer::TopMost),

            _ => None,
        }
    }
    pub fn idx(&self) -> u8 {
        match self {
            Layer::FarBack => 0,
            Layer::GameBackground => 1,
            Layer::Game => 2,
            Layer::GameForeground => 3,
            Layer::UiBackground => 4,
            Layer::Ui => 5,
            Layer::UiForeground => 6,
            Layer::TopMost => 7,
        }
    }
}
