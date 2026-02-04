use crate::app::{App, Mode};
use crate::modes;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_event(app: &mut App, key: KeyEvent) -> bool {
    if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
        return false;
    }

    match app.mode {
        Mode::Normal => modes::normal::handle_key(app, key),
        Mode::Insert => modes::insert::handle_key(app, key),
        Mode::Command => modes::command::handle_key(app, key),
        Mode::Visual => modes::visual::handle_key(app, key),
    }

    !app.should_quit
}
