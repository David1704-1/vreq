use crate::app::{App, Mode, Panel, PendingCommand};
use crossterm::event::{KeyCode, KeyEvent};

/// Handle key events in Normal mode
pub fn handle_key(app: &mut App, key: KeyEvent) {
    match key.code {
        // Mode switching
        KeyCode::Char('i') => {
            app.set_mode(Mode::Insert);
            // TODO: Position cursor based on 'i', 'a', 'I', 'A' variants
        }
        KeyCode::Char('a') => {
            app.set_mode(Mode::Insert);
            // TODO: Move cursor one position right (insert after)
        }
        KeyCode::Char('I') => {
            app.set_mode(Mode::Insert);
            // TODO: Move cursor to start of line
        }
        KeyCode::Char('A') => {
            app.set_mode(Mode::Insert);
            // TODO: Move cursor to end of line
        }
        KeyCode::Char(':') => {
            app.set_mode(Mode::Command);
        }

        // Panel navigation
        KeyCode::Tab => {
            // Sidebar -> Url -> Headers -> Body -> Response -> Sidebar
            match app.active_panel {
                Panel::Sidebar => app.set_panel(Panel::Url),
                Panel::Url => app.set_panel(Panel::Headers),
                Panel::Headers => app.set_panel(Panel::Body),
                Panel::Body => app.set_panel(Panel::Response),
                Panel::Response => app.set_panel(Panel::Sidebar),
            }
        }
        KeyCode::BackTab => {
            match app.active_panel {
                Panel::Sidebar => app.set_panel(Panel::Response),
                Panel::Url => app.set_panel(Panel::Sidebar),
                Panel::Headers => app.set_panel(Panel::Url),
                Panel::Body => app.set_panel(Panel::Headers),
                Panel::Response => app.set_panel(Panel::Body),
            }
        }
        KeyCode::Char('1') => app.set_panel(Panel::Sidebar),
        KeyCode::Char('2') => app.set_panel(Panel::Url),
        KeyCode::Char('3') => app.set_panel(Panel::Headers),
        KeyCode::Char('4') => app.set_panel(Panel::Body),
        KeyCode::Char('5') => app.set_panel(Panel::Response),

        // Vim motions
        KeyCode::Char('h') => {
            // Move cursor left
            let cursor = app.cursor();
            if cursor > 0 {
                app.set_cursor(cursor - 1);
            }
        }
        KeyCode::Char('j') => {
            // Move cursor down (multi-line aware)
            app.move_cursor_down();
        }
        KeyCode::Char('k') => {
            // Move cursor up (multi-line aware)
            app.move_cursor_up();
        }
        KeyCode::Char('l') => {
            // Move cursor right
            let cursor = app.cursor();
            let buffer_len = app.current_buffer().len();
            if cursor < buffer_len {
                app.set_cursor(cursor + 1);
            }
        }
        KeyCode::Char('w') => {
            app.move_word_forward();
        }
        KeyCode::Char('b') => {
            app.move_word_backwards();
        }
        KeyCode::Char('0') => {
            // Move cursor to start of current line
            let buffer = app.current_buffer();
            let cursor = app.cursor();
            let (line, _) = app.cursor_to_line_col(buffer, cursor);
            let new_cursor = app.line_col_to_cursor(buffer, line, 0);
            app.set_cursor(new_cursor);
        }
        KeyCode::Char('$') => {
            // Move cursor to end of current line
            let buffer = app.current_buffer();
            let cursor = app.cursor();
            let (line, _) = app.cursor_to_line_col(buffer, cursor);

            let lines: Vec<&str> = buffer.lines().collect();
            if line < lines.len() {
                let line_len = lines[line].len();
                let new_cursor = app.line_col_to_cursor(buffer, line, line_len);
                app.set_cursor(new_cursor);
            }
        }
        KeyCode::Char('g') => {
            if let Some(cmd) = app.pending_command && cmd == PendingCommand::Goto {
                app.set_cursor(0) 
            } else if app.pending_command.is_none() {
                app.set_pending_command(PendingCommand::Goto);
            }
        }
        KeyCode::Char('G') => {
            let buffer = app.current_buffer();

            let lines: Vec<&str> = buffer.lines().collect();
            let last_line = lines.len() - 1;
            let new_cursor = app.line_col_to_cursor(buffer, last_line, 0);
            app.set_cursor(new_cursor);
        }

        // Actions
        KeyCode::Enter => {
            // TODO: Send HTTP request
            // This should:
            // 1. Build request from current state
            // 2. Send it using http::send_request()
            // 3. Store response in app.last_response
            // 4. Switch active panel to Response
        }
        KeyCode::Char('d') => {
            // TODO: Enter delete mode or delete based on motion
            // dd - delete line
            // dw - delete word
            // d$ - delete to end of line
            app.set_pending_command(PendingCommand::Delete);
        }
        KeyCode::Char('y') => {
            // TODO: Enter yank/copy mode
            // yy - yank line
            // yw - yank word
            app.set_pending_command(PendingCommand::Yank);
        }
        KeyCode::Char('p') => {
            // TODO: Paste from clipboard/register
        }
        KeyCode::Char('u') => {
            // TODO: Undo last change
        }

        // Quit shortcut (in normal mode, 'q' could quit)
        KeyCode::Char('q') => {
            app.should_quit = true;
        }

        _ => {
            // Ignore unhandled keys in normal mode
        }
    }
}
