use std::cmp;

use bubbles::{
    components::progress::Progress,
    core::{
        dimension::Dimension,
        renderer::{OnRender, Renderer},
        style::Style,
    },
};
use crossterm::style::Color;

pub fn update_handler(p: &mut Progress) {
    p.value = cmp::min(p.value + 1, 100);
}

pub fn create_progress_bar<'a>() -> Result<Progress<'a>, ()> {
    let gradient = vec!["#5A56E0", "#EE6FF8"];

    let renderer = Renderer::from(OnRender(&update_handler));

    let root = Progress::new(
        0,
        Style::new(gradient, Color::Grey, Color::Reset),
        Dimension::new(0, 2, 100, 0),
        renderer,
    );

    Ok(root)
}
