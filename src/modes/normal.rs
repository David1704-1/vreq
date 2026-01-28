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
            match app.pending_command {
                Some(cmd) => {
                    match cmd {
                        PendingCommand::Delete => {
                            let cursor = app.cursor();
                            let (current_line, line_cursor) = {
                                let buffer = app.current_buffer();
                                app.cursor_to_line_col(buffer, cursor)
                            };

                            if let Some(buffer) = app.current_buffer_mut() {
                                let mut lines: Vec<String> = buffer.lines().map(|s| s.to_string()).collect();
                                if current_line < lines.len() {
                                    if let Some(delete_line) = lines[current_line].get(0..line_cursor) {
                                        lines[current_line] =  delete_line.to_string();
                                    } 
                                    *buffer = lines.join("\n");
                                }
                            }
                            app.clear_pending_command();
                        },
                        PendingCommand::Yank => {
                            let cursor = app.cursor();
                            let (current_line, line_cursor) = {
                                let buffer = app.current_buffer();
                                app.cursor_to_line_col(buffer, cursor)
                            };

                            if let Some(buffer) = app.current_buffer_mut() {
                                let lines: Vec<String> = buffer.lines().map(|s| s.to_string()).collect();
                                if let Some(yank_line) = lines[current_line].get(0..line_cursor) {
                                    app.set_yank_register(yank_line.to_string());
                                } 
                            }
                            app.clear_pending_command();
                        },
                        PendingCommand::Goto => {},
                    }
                },
                _ => {
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
            }
        }
        KeyCode::Char('g') => {
            if let Some(cmd) = app.pending_command && cmd == PendingCommand::Goto {
                app.set_cursor(0);
                app.clear_pending_command();
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
            // TODO: Delete with motions (dw - delete word, d$ - delete to end of line)
            if let Some(cmd) = app.pending_command && cmd == PendingCommand::Delete {
                // dd - delete current line
                let cursor = app.cursor();
                let current_line = {
                    let buffer = app.current_buffer();
                    let (line, _) = app.cursor_to_line_col(buffer, cursor);
                    line
                };

                // Perform the deletion
                if let Some(buffer) = app.current_buffer_mut() {
                    let mut lines: Vec<String> = buffer.lines().map(|s| s.to_string()).collect();

                    if current_line < lines.len() {
                        lines.remove(current_line);
                        *buffer = lines.join("\n");
                    }
                }

                // Calculate new cursor position after deletion (mutable borrow has ended)
                let buffer = app.current_buffer();
                let lines: Vec<&str> = buffer.lines().collect();
                let new_line = current_line.min(lines.len().saturating_sub(1));
                let new_cursor = if buffer.is_empty() {
                    0
                } else {
                    app.line_col_to_cursor(buffer, new_line, 0)
                };
                app.set_cursor(new_cursor);

                app.clear_pending_command();
            } else if app.pending_command.is_none() {
                app.set_pending_command(PendingCommand::Delete);
            }
        }
        KeyCode::Char('y') => {
            if let Some(cmd) = app.pending_command && cmd == PendingCommand::Yank {
                let cursor = app.cursor();
                let current_line = {
                    let buffer = app.current_buffer();
                    let (line, _) = app.cursor_to_line_col(buffer, cursor);
                    line
                };

                if let Some(buffer) = app.current_buffer_mut() {
                    let lines: Vec<String> = buffer.lines().map(|s| s.to_string()).collect();

                    if current_line < lines.len() {
                        app.set_yank_register(lines[current_line].to_string());

                    }
                }

                app.clear_pending_command();
            } else if app.pending_command.is_none() {
                app.set_pending_command(PendingCommand::Yank);
            }
        }
        KeyCode::Char('p') => {
            if let Some(yanked_content) = &app.yank_register.clone() {
                let cursor = app.cursor();
                let current_line = {
                    let buffer = app.current_buffer();
                    let (line, _) = app.cursor_to_line_col(buffer, cursor);
                    line
                };

                if let Some(buffer) = app.current_buffer_mut() {
                    let mut lines: Vec<String> = buffer.lines().map(|s| s.to_string()).collect();

                    if lines.is_empty() {
                        lines.push(yanked_content.clone());
                    } else if current_line < lines.len() {
                        lines.insert(current_line + 1, yanked_content.clone());
                    } else {
                        lines.push(yanked_content.clone());
                    }

                    *buffer = lines.join("\n");
                }

                let buffer = app.current_buffer();
                let new_line = (current_line + 1).min(buffer.lines().count().saturating_sub(1));
                let new_cursor = app.line_col_to_cursor(buffer, new_line, 0);
                app.set_cursor(new_cursor);
            }
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
