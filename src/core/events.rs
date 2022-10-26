use crossterm::event::{KeyEvent, MouseEvent};

pub trait EventHandler {
    fn handle_key_event(&mut self, event: KeyEvent);
    fn handle_mouse_event(&mut self, event: MouseEvent);
    fn handle_paste_event(&mut self, event: MouseEvent);
}
