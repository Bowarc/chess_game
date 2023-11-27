const NBR_OF_ELEMENTS: usize = 80;

pub struct Graph {
    id: crate::ui::Id,
    position: crate::ui::Position,
    size: crate::ui::Vector,
    style: crate::ui::Style,

    values: std::collections::VecDeque<f64>,
    accept_timer: time::DTDelay,

    text: Option<GraphText>,

    max: f64,
}

impl Graph {
    pub fn new(
        id: crate::ui::Id,
        position: crate::ui::Position,
        size: crate::ui::Vector,
        style: crate::ui::Style,
        text: Option<GraphText>,
    ) -> Self {
        Self {
            id,
            position,
            size,
            style,
            values: Default::default(),
            accept_timer: time::DTDelay::new(0.1),
            text,
            max: 0.,
        }
    }
    pub fn push(&mut self, val: f64, dt: f64) {
        self.accept_timer.update(dt);
        if !self.accept_timer.ended() {
            return;
        }
        self.accept_timer.restart();

        self.values.push_back(val);

        self.max = self
            .values
            .iter()
            .fold(std::f64::NEG_INFINITY, |max, &val| val.max(max))
            * 1.50;

        while self.values.len() > NBR_OF_ELEMENTS {
            self.values.pop_front();
        }
    }
}

pub struct GraphText {
    anchor: crate::ui::Anchor,
    offset: shared::maths::Vec2,
    size: f64,
    color: crate::render::Color,
    text: fn(f64) -> String,
}

impl super::TElement for Graph {
    fn draw(
        &mut self,
        ctx: &mut ggez::Context,
        back_mesh: &mut ggez::graphics::MeshBuilder,
        ui_mesh: &mut ggez::graphics::MeshBuilder,
        front_mesh: &mut ggez::graphics::MeshBuilder,
        render_request: &mut crate::render::RenderRequest,
    ) -> ggez::GameResult {
        let rect = self.get_computed_rect(ctx);
        let horizontal_space = rect.size().x as f32 / NBR_OF_ELEMENTS as f32;

        // draw background
        if let Some(bg) = self.style.get_bg() {
            bg.draw(back_mesh, render_request, rect)?
        }

        // draw border
        if let Some(border) = self.style.get_border() {
            border.draw(front_mesh, rect)?;
        };

        // draw debug text
        if let Some(graph_text) = &self.text {
            if !self.values.is_empty() {
                let text = (graph_text.text)(*self.values.back().unwrap()); // Unwraping here is fine as we checked above if the list was empty or not
                let mut ggtext = ggez::graphics::Text::new(text);
                ggtext.set_layout(ggez::graphics::TextLayout::top_left());

                // let p = graph_text
                //     .anchor
                //     .compute(rect.size(), ggtext.measure(ctx).unwrap().into())
                //     + rect.r_topleft();
                let p = rect.aa_topleft();

                render_request.add(
                    ggtext,
                    crate::render::DrawParam::default()
                        .pos(p)
                        .color(graph_text.color),
                    crate::render::Layer::UiForeground,
                );
            }
        }

        let mut saved_height = None;
        for (i, val) in self.values.iter().enumerate() {
            let curr_height = (*val as f32 / self.max as f32) * rect.size().y as f32;
            if curr_height.is_nan() {
                // warn!(
                //     "Could not draw Graph id '{}' because the given value is NAN",
                //     self.id
                // );
                continue;
            }

            // trace!("{curr_height} - {}", self.max);
            if saved_height.is_none() {
                saved_height = Some(curr_height);
                continue;
            }

            ui_mesh.line(
                &[
                    [
                        rect.aa_botleft().x as f32 + horizontal_space * (i - 1) as f32 - 1.,
                        rect.aa_botleft().y as f32 - saved_height.unwrap(),
                    ],
                    [
                        rect.aa_botleft().x as f32 + horizontal_space * i as f32 + 1.,
                        rect.aa_botleft().y as f32 - curr_height,
                    ],
                ],
                3.,
                (*self.style.get_color()).into(),
            )?;

            saved_height = Some(curr_height)
        }

        Ok(())
    }
    fn get_size_value(&self) -> &crate::ui::Vector {
        &self.size
    }
    fn get_pos_value(&self) -> &crate::ui::Position{
        &self.position
    }
    fn get_id(&self) -> crate::ui::Id {
        self.id.clone()
    }
}

impl GraphText {
    pub fn anchor(mut self, new_anchor: crate::ui::Anchor) -> Self {
        self.anchor = new_anchor;
        self
    }

    pub fn offset(mut self, new_offset: impl Into<shared::maths::Vec2>) -> Self {
        self.offset = new_offset.into();
        self
    }
    pub fn size(mut self, new_size: f64) -> Self {
        self.size = new_size;
        self
    }

    pub fn color(mut self, new_color: crate::render::Color) -> Self {
        self.color = new_color;
        self
    }

    pub fn text(mut self, new_text: fn(f64) -> String) -> Self {
        self.text = new_text;
        self
    }
}

impl Default for GraphText {
    fn default() -> Self {
        Self {
            anchor: crate::ui::Anchor::Topleft,
            offset: shared::maths::Vec2::ZERO,
            size: 10.,
            color: crate::render::Color::WHITE,
            text: |val| -> String { format!("{val:.3}") },
        }
    }
}
