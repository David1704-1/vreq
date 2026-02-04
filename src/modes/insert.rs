use crate::app::{App, Mode, Panel};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.set_mode(Mode::Normal);
            app.clear_pending_command();
        }
        KeyCode::Char('[') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.set_mode(Mode::Normal);
        }

        KeyCode::Char(c) => {
            let cursor = app.cursor();
            if let Some(buffer) = app.current_buffer_mut() {
                buffer.insert(cursor, c);
                app.set_cursor(cursor + 1);
            }
        }

        KeyCode::Backspace => {
            let cursor = app.cursor();
            if let Some(buffer) = app.current_buffer_mut() && cursor > 0 {
                buffer.remove(cursor - 1);
                app.set_cursor(cursor - 1);
            }
        }
        KeyCode::Delete => {
            let cursor = app.cursor();
            if let Some(buffer) = app.current_buffer_mut() && !buffer.is_empty() {
                buffer.remove(cursor);
            }
        }

        KeyCode::Left => {
            let cursor = app.cursor();
            if cursor > 0 {
                app.set_cursor(cursor - 1);
            }
        }
        KeyCode::Right => {
            let cursor = app.cursor();
            let buffer_len = app.current_buffer().len();
            if cursor < buffer_len {
                app.set_cursor(cursor + 1);
            }
        }
        KeyCode::Up => {
            app.move_cursor_up();
        }
        KeyCode::Down => {
            app.move_cursor_down();
        }

        KeyCode::Enter => {
            let cursor = app.cursor();
            if (app.active_panel == Panel::Headers || app.active_panel == Panel::Body)
                && let Some(buffer) = app.current_buffer_mut()
            {
                buffer.insert(cursor, '\n');
                app.set_cursor(cursor + 1);
            }
        }
        _ => {}
    }
}
