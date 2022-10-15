use super::{render::Render, state::State, view::View};

pub struct Component {
    pub(crate) view: View,
    pub(crate) handlers: Vec<Box<dyn Fn(&mut Self)>>,
}

impl Component {
    pub fn new(view: View) -> Self {
        Self {
            view,
            handlers: Vec::new(),
        }
    }

    pub fn default() -> Self {
        Self {
            view: View::default(),
            handlers: Vec::new(),
        }
    }
}

impl Render for Component {
    fn render(&self) -> View {
        self.view.to_owned()
    }
}

// impl State for Component {
//     fn on_update(&mut self, handler: &'static dyn Fn(&mut Self)) {
//         self.handlers.push(Box::new(handler));
//     }
// }
