const NBR_OF_ELEMENTS: usize = 80;

pub struct Graph {
    id: crate::ui::Id,
    position: crate::ui::Position,
    size: ggez::mint::Point2<crate::ui::Value>,
    style: crate::ui::Style,

    values: std::collections::VecDeque<f64>,
    accept_timer: time::SystemTimeDelay,

    max: f64,
}

impl Graph {
    pub fn new(
        position: crate::ui::Position,
        size: ggez::mint::Point2<crate::ui::Value>,
        style: crate::ui::Style,
    ) -> Self {
        Self {
            id: crate::ui::Id::new(),
            position,
            size,
            style,
            values: Default::default(),
            accept_timer: time::SystemTimeDelay::new(100),
            max: 0.,
        }
    }
    pub fn push(&mut self, v: f64) {
        if let time::DelayState::Running = self.accept_timer.ended() {
            return;
        }
        self.accept_timer.restart();

        self.values.push_back(v);

        let mut max = 0.;
        self.values.iter().for_each(|v| {
            if *v > max {
                max = *v
            }
        });
        max *= 1.50;
        self.max = max;

        while self.values.len() > NBR_OF_ELEMENTS {
            self.values.pop_front();
        }
    }
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
            back_mesh.rectangle(
                ggez::graphics::DrawMode::fill(),
                self.get_computed_rect(ctx).into(),
                (*bg.get_color()).into(),
            )?;
        }

        // draw border
        if let Some(border) = self.style.get_border() {
            let r = shared::maths::Rect::new(
                rect.r_topleft() - border.get_size() / 2.,
                rect.size() + *border.get_size(),
                rect.rotation(),
            );

            front_mesh.rectangle(
                ggez::graphics::DrawMode::stroke(*border.get_size() as f32),
                r.into(),
                (*border.get_color()).into(),
            )?;
        };

        let mut saved_height = None;
        for (i, val) in self.values.iter().enumerate() {
            let curr_height = (*val as f32 / self.max as f32) * rect.size().y as f32;
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
    fn get_size_value(&self) -> &ggez::mint::Point2<crate::ui::Value> {
        &self.size
    }
    fn get_pos_value(&self) -> &crate::ui::Position {
        &self.position
    }
    fn get_id(&self) -> shared::id::Id {
        self.id
    }
}
