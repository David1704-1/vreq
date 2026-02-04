use crate::app::{App, Mode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::process::Command;

pub fn handle_key(app: &mut App, key: KeyEvent) {
    let text = app.response_text();
    let len = text.len();

    match key.code {
        KeyCode::Esc => {
            app.visual_anchor = None;
            app.set_mode(Mode::Normal);
        }

        KeyCode::Char('h') => {
            let cursor = app.cursor();
            if cursor > 0 {
                app.set_cursor(cursor - 1);
            }
        }
        KeyCode::Char('l') => {
            let cursor = app.cursor();
            if cursor < len {
                app.set_cursor(cursor + 1);
            }
        }
        KeyCode::Char('j') => {
            let cursor = app.cursor();
            let (line, col) = app.cursor_to_line_col(&text, cursor);
            let lines: Vec<&str> = text.lines().collect();
            if line + 1 < lines.len() {
                let new_col = col.min(lines[line + 1].len());
                let new_cursor = app.line_col_to_cursor(&text, line + 1, new_col);
                app.set_cursor(new_cursor);
            }
        }
        KeyCode::Char('k') => {
            let cursor = app.cursor();
            let (line, col) = app.cursor_to_line_col(&text, cursor);
            if line > 0 {
                let lines: Vec<&str> = text.lines().collect();
                let new_col = col.min(lines[line - 1].len());
                let new_cursor = app.line_col_to_cursor(&text, line - 1, new_col);
                app.set_cursor(new_cursor);
            }
        }
        KeyCode::Char('w') => {
            let cursor = app.cursor();
            let chars: Vec<char> = text.chars().collect();
            let mut pos = cursor;
            if pos < chars.len() {
                let is_word = |c: char| c.is_alphanumeric() || c == '_';
                if is_word(chars[pos]) {
                    while pos < chars.len() && is_word(chars[pos]) { pos += 1; }
                } else if !chars[pos].is_whitespace() {
                    while pos < chars.len() && !chars[pos].is_whitespace() && !is_word(chars[pos]) { pos += 1; }
                }
                while pos < chars.len() && chars[pos].is_whitespace() { pos += 1; }
            }
            app.set_cursor(pos);
        }
        KeyCode::Char('b') => {
            let cursor = app.cursor();
            let chars: Vec<char> = text.chars().collect();
            let mut pos = cursor.saturating_sub(1);
            let is_word = |c: char| c.is_alphanumeric() || c == '_';
            while pos > 0 && pos < chars.len() && chars[pos].is_whitespace() { pos -= 1; }
            if pos < chars.len() && !chars[pos].is_whitespace() {
                let in_word = is_word(chars[pos]);
                while pos > 0 {
                    let prev = chars[pos - 1];
                    if is_word(prev) != in_word || prev.is_whitespace() { break; }
                    pos -= 1;
                }
            }
            app.set_cursor(pos);
        }
        KeyCode::Char('0') => {
            let cursor = app.cursor();
            let (line, _) = app.cursor_to_line_col(&text, cursor);
            let new_cursor = app.line_col_to_cursor(&text, line, 0);
            app.set_cursor(new_cursor);
        }
        KeyCode::Char('$') => {
            let cursor = app.cursor();
            let (line, _) = app.cursor_to_line_col(&text, cursor);
            let lines: Vec<&str> = text.lines().collect();
            if line < lines.len() {
                let new_cursor = app.line_col_to_cursor(&text, line, lines[line].len());
                app.set_cursor(new_cursor);
            }
        }

        KeyCode::Char('y') => {
            if let Some(anchor) = app.visual_anchor {
                let cursor = app.cursor();
                let start = anchor.min(cursor);
                let end = (anchor.max(cursor) + 1).min(len);
                let selected = &text[start..end];
                copy_to_clipboard(selected);
                app.set_yank_register(selected.to_string());
            }
            app.visual_anchor = None;
            app.set_mode(Mode::Normal);
        }

        KeyCode::Char('[') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            app.visual_anchor = None;
            app.set_mode(Mode::Normal);
        }

        _ => {}
    }
}

fn copy_to_clipboard(text: &str) {
    let _ = Command::new("xclip")
        .args(["-selection", "clipboard"])
        .stdin(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            child.stdin.as_mut().unwrap().write_all(text.as_bytes())?;
            child.wait()
        });
}
