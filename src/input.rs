use std::time::Duration;

use crossterm::event::KeyEvent;

pub fn poll(timeout: Duration) -> Option<KeyEvent> {
    None
}
