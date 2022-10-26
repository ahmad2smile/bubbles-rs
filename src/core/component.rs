use super::{events::EventHandler, lifecycle::LifeCycle, render::Render};

pub trait Component: Render + LifeCycle + EventHandler {}
