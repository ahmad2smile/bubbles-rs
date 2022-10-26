use crossterm::{
    event::{KeyEvent, MouseEvent},
    style::Stylize,
};

use crate::core::{
    component::Component, dimension::Dimension, events::EventHandler, lifecycle::LifeCycle,
    render::Render, renderer::Renderer, style::Style, view::View,
};

/// Input bar
pub struct Input<'a> {
    pub value: String,
    pub style: Style,
    pub dimension: Dimension,
    pub renderer: Renderer<'a, Self>,
}

impl<'a> Input<'a> {
    const PLACEHOLDER: &'static str = "Type here...";

    pub fn new(
        value: String,
        style: Style,
        dimension: Dimension,
        renderer: Renderer<'a, Self>,
    ) -> Self {
        Self {
            value,
            style,
            dimension,
            renderer,
        }
    }
}

impl Default for Input<'_> {
    fn default() -> Self {
        Self {
            value: Input::PLACEHOLDER.to_string(),
            dimension: Dimension::default(),
            style: Style::default(),
            renderer: Renderer::default(),
        }
    }
}

impl Render for Input<'_> {
    fn render(&self) -> View {
        View {
            color: self.style.color,
            dimension: self.dimension,
            background: self.style.background,
            content: self.value.to_owned().with(self.style.color).to_string(),
        }
    }
}

impl LifeCycle for Input<'_> {
    fn handle_mount(&mut self) {}

    fn handle_render(&mut self) {
        (self.renderer.on_render)(self)
    }

    fn handle_unmount(&mut self) {}
}

impl Component for Input<'_> {}

impl EventHandler for Input<'_> {
    fn handle_key_event(&mut self, event: KeyEvent) {
        (self.renderer.on_key_press)(self, event);
    }

    fn handle_mouse_event(&mut self, _event: MouseEvent) {}

    fn handle_paste_event(&mut self, _event: MouseEvent) {}
}
