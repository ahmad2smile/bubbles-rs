use super::view::View;

pub trait Render {
    fn render(&self) -> View;
}
