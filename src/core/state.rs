pub trait State<'a> {
    fn on_update(&mut self, handler: &'a dyn Fn(&mut Self));
    fn handle_update(&mut self);
}
