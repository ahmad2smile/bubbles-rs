pub trait LifeCycle {
    fn handle_mount(&mut self);
    fn handle_render(&mut self);
    fn handle_unmount(&mut self);
}
