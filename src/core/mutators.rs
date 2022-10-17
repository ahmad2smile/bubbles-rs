pub(crate) fn default_fn<'a, T>(_: &'a mut T) {}

pub struct OnMount<'a, T> {
    pub on_mount: &'a dyn Fn(&mut T),
}

impl<'a, T> OnMount<'a, T> {
    pub fn new(on_mount: &'a dyn Fn(&mut T)) -> Self {
        Self { on_mount }
    }

    pub fn default() -> Self {
        Self {
            on_mount: &default_fn,
        }
    }
}

pub struct OnRender<'a, T> {
    pub on_render: &'a dyn Fn(&mut T),
}

impl<'a, T> OnRender<'a, T> {
    pub fn new(on_render: &'a dyn Fn(&mut T)) -> Self {
        Self { on_render }
    }

    pub fn default() -> Self {
        Self {
            on_render: &default_fn,
        }
    }
}

pub struct OnUnmount<'a, T> {
    pub on_unmount: &'a dyn Fn(&mut T),
}

impl<'a, T> OnUnmount<'a, T> {
    pub fn new(on_unmount: &'a dyn Fn(&mut T)) -> Self {
        Self { on_unmount }
    }

    pub fn default() -> Self {
        Self {
            on_unmount: &default_fn,
        }
    }
}
