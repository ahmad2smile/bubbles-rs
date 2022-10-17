use std::cmp;

use crossterm::style::Stylize;

use crate::{
    colors::gradient::Gradients,
    core::{
        dimension::Dimension, lifecycle::LifeCycle, mutators::OnRender, render::Render,
        style::Style, view::View,
    },
};

/// Progress bar
pub struct Progress<'a> {
    pub value: usize,
    pub style: Style,
    pub dimension: Dimension,
    content: &'static str,
    content_empty: &'static str,
    pub renderer: OnRender<'a, Self>,
}

impl<'a> Progress<'a> {
    const CONTENT: &'static str = "█";
    const CONTENT_EMPTY: &'static str = "░";

    pub fn new(
        value: usize,
        style: Style,
        dimension: Dimension,
        renderer: OnRender<'a, Self>,
    ) -> Self {
        Self {
            value,
            style,
            dimension,
            renderer,
            content: Progress::CONTENT,
            content_empty: Progress::CONTENT_EMPTY,
        }
    }

    pub fn default() -> Self {
        Self {
            value: 0,
            dimension: Dimension::default(),
            style: Style::default(),
            content: Progress::CONTENT,
            content_empty: Progress::CONTENT_EMPTY,
            renderer: OnRender::default(),
        }
    }

    pub fn update_value(&mut self, value: usize) {
        self.value = cmp::min(self.dimension.width, value);
    }
}

impl Render for Progress<'_> {
    fn render(&self) -> View {
        let colors = Gradients::from_colors(&self.style.gradient, self.dimension.width);

        let max_colors = cmp::min(self.value, self.dimension.width);

        let bar_content = colors[..max_colors]
            .iter()
            .map(|c| self.content.to_owned().with(c.to_owned()).to_string())
            .collect::<Vec<String>>()
            .join("")
            + &self.content_empty.repeat(self.dimension.width - max_colors);

        View {
            color: self.style.color,
            dimension: self.dimension,
            background: self.style.background,
            content: bar_content + format!(" {}%", self.value).as_str(),
        }
    }
}

impl LifeCycle for Progress<'_> {
    fn handle_mount(&mut self) {}

    fn handle_render(&mut self) {
        (self.renderer.on_render)(self)
    }

    fn handle_unmount(&mut self) {}
}
