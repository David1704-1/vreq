use crate::app::{App, Mode, Panel};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handle key events in Insert mode
pub fn handle_key(app: &mut App, key: KeyEvent) {
    match key.code {
        // Exit insert mode
        KeyCode::Esc => {
            app.set_mode(Mode::Normal);
            app.clear_pending_command();
        }
        KeyCode::Char('[') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.set_mode(Mode::Normal);
        }

        // Text insertion
        KeyCode::Char(c) => {
            let cursor = app.cursor();
            if let Some(buffer) = app.current_buffer_mut() {
                buffer.insert(cursor, c);
                app.set_cursor(cursor + 1);
            }
        }

        // Text deletion
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

        // Cursor movement (optional in insert mode, but helpful)
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
            // Move up one line
            app.move_cursor_up();
        }
        KeyCode::Down => {
            // Move down one line
            app.move_cursor_down();
        }

        // New line (only in multi-line buffers like Body)
        KeyCode::Enter => {
            // TODO: Insert newline character in appropriate buffers
            // Should only work in Body panel, not URL panel
            let cursor = app.cursor();
            if app.active_panel == Panel::Body && let Some(buffer) = app.current_buffer_mut() {
                buffer.insert(cursor, '\n');
                app.move_cursor_down();
            }
        }
        _ => {
            // Ignore other keys
        }
    }
}
