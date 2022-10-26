use crossterm::event::{KeyEvent, MouseEvent};

pub(crate) fn default_view<'a, T>(_: &'a mut T) {}
pub(crate) fn default_key_event<'a, T>(_: &'a mut T, _: KeyEvent) {}
pub(crate) fn default_mouse_event<'a, T>(_: &'a mut T, _: MouseEvent) {}

pub struct OnMount<'a, T>(pub &'a dyn Fn(&mut T));
pub struct OnRender<'a, T>(pub &'a dyn Fn(&mut T));
pub struct OnUnmount<'a, T>(pub &'a dyn Fn(&mut T));
pub struct OnKeyPress<'a, T>(pub &'a dyn Fn(&mut T, KeyEvent));
pub struct OnClickLeft<'a, T>(pub &'a dyn Fn(&mut T, MouseEvent));
pub struct OnClickRight<'a, T>(pub &'a dyn Fn(&mut T, MouseEvent));

pub struct Renderer<'a, T> {
    pub on_mount: &'a dyn Fn(&mut T),
    pub on_render: &'a dyn Fn(&mut T),
    pub on_unmount: &'a dyn Fn(&mut T),

    pub on_key_press: &'a dyn Fn(&mut T, KeyEvent),
    pub on_click_left: &'a dyn Fn(&mut T, MouseEvent),
    pub on_click_right: &'a dyn Fn(&mut T, MouseEvent),
}

impl<'a, T> Renderer<'a, T> {
    pub fn new(
        on_mount: &'a dyn Fn(&mut T),
        on_render: &'a dyn Fn(&mut T),
        on_unmount: &'a dyn Fn(&mut T),
        on_key_press: &'a dyn Fn(&mut T, KeyEvent),
        on_click_left: &'a dyn Fn(&mut T, MouseEvent),
        on_click_right: &'a dyn Fn(&mut T, MouseEvent),
    ) -> Self {
        Self {
            on_mount,
            on_render,
            on_unmount,
            on_key_press,
            on_click_left,
            on_click_right,
        }
    }
}

impl<T> Default for Renderer<'_, T> {
    fn default() -> Self {
        Self {
            on_mount: &default_view,
            on_render: &default_view,
            on_unmount: &default_view,
            on_key_press: &default_key_event,
            on_click_left: &default_mouse_event,
            on_click_right: &default_mouse_event,
        }
    }
}

impl<'a, T> From<OnMount<'a, T>> for Renderer<'a, T> {
    fn from(OnMount(on_mount): OnMount<'a, T>) -> Self {
        Self {
            on_mount,
            ..Renderer::default()
        }
    }
}

impl<'a, T> From<OnRender<'a, T>> for Renderer<'a, T> {
    fn from(OnRender(on_render): OnRender<'a, T>) -> Self {
        Self {
            on_render,
            ..Renderer::default()
        }
    }
}

impl<'a, T> From<OnUnmount<'a, T>> for Renderer<'a, T> {
    fn from(OnUnmount(on_unmount): OnUnmount<'a, T>) -> Self {
        Self {
            on_unmount,
            ..Renderer::default()
        }
    }
}

impl<'a, T> From<OnKeyPress<'a, T>> for Renderer<'a, T> {
    fn from(OnKeyPress(on_key_press): OnKeyPress<'a, T>) -> Self {
        Self {
            on_key_press,
            ..Renderer::default()
        }
    }
}

impl<'a, T> From<OnClickLeft<'a, T>> for Renderer<'a, T> {
    fn from(OnClickLeft(on_click_left): OnClickLeft<'a, T>) -> Self {
        Self {
            on_click_left,
            ..Renderer::default()
        }
    }
}

impl<'a, T> From<OnClickRight<'a, T>> for Renderer<'a, T> {
    fn from(OnClickRight(on_click_right): OnClickRight<'a, T>) -> Self {
        Self {
            on_click_right,
            ..Renderer::default()
        }
    }
}
