use ggegui::egui;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ArchitectureBit<State> {
    me: State,
    parent: State,
    childs: Vec<State>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Architecture<State: std::cmp::PartialEq + Clone + std::fmt::Debug> {
    uis: Vec<ArchitectureBit<State>>,
    pub actual: State,
}

impl<State: std::cmp::PartialEq + Clone + std::fmt::Debug> Architecture<State> {
    pub fn reset(&mut self) {
        self.actual = self.uis.first().unwrap().me.clone();
    }
    pub fn forward(&mut self, new_ui: &State) {
        for bit in &self.uis {
            if bit.me == self.actual && bit.childs.contains(new_ui) {
                self.actual = new_ui.clone();
                break;
            }
        }
        // if new_ui in self.uis.get(self.actual){}
    }
    pub fn backward(&mut self) {
        for bit in &self.uis {
            if bit.me == self.actual {
                self.actual = bit.parent.clone();
                break;
            }
        }
    }
    pub fn draw_childs(&mut self, ui: &mut egui::Ui) {
        let current = self.uis.iter().find(|bit| bit.me == self.actual).cloned();
        if current.is_none() {
            error!(
                "The current ui does not appear in the ui list: '{:?}'",
                self.actual
            );
            return;
        }
        let current = current.unwrap();
        for child in current.childs.iter() {
            if ui.button(format!("{child:?}")).clicked() {
                self.forward(child)
            }
        }

        if ui.button("Back").clicked() {
            self.backward()
        }
    }
}
