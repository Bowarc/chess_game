mod button;
mod graph;

pub enum Widget {
    Button(button::Button),
    Graph(graph::Graph),
}

/// Constructors
impl Widget {
    pub fn new_button() -> Self {
        Self::Button(button::Button::new())
    }
}

/// Getters
impl Widget {
    pub fn inner_button(&mut self) -> &mut button::Button {
        if let Self::Button(inner) = self {
            inner
        } else {
            panic!("This widget is not a Button");
        }
    }
    pub fn inner_graph(&mut self) -> &mut graph::Graph {
        if let Self::Graph(inner) = self {
            inner
        } else {
            panic!("This widget is not a Graph");
        }
    }
}
