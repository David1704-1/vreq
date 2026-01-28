use crate::http::{Request, Response};
use std::collections::HashMap;

/// Application modes (vim-style)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
    Command,
}

/// UI panels that can receive focus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Panel {
    Sidebar,      // Request collections/history
    Url,          // URL input
    Headers,      // Headers table
    Body,         // Request body
    Response,     // Response viewer
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PendingCommand {
    Delete, 
    Yank, 
    Goto,
}

/// Main application state
pub struct App {
    /// Current mode (Normal, Insert, Command)
    pub mode: Mode,

    /// Currently focused panel
    pub active_panel: Panel,

    /// URL input buffer
    pub url_buffer: String,

    /// Request body buffer
    pub body_buffer: String,

    /// Command line buffer (for Command mode)
    pub command_buffer: String,

    /// Headers for the current request
    pub headers: Vec<(String, String)>,

    pub pending_command: Option<PendingCommand>,

    /// Current HTTP request
    pub current_request: Request,

    /// Last HTTP response (if any)
    pub last_response: Option<Response>,

    /// Cursor positions for each panel (for text editing)
    pub cursors: HashMap<Panel, usize>,

    /// Flag to indicate if app should quit
    pub should_quit: bool,

    /// Saved request collections (just names for now)
    pub collections: Vec<String>,

    /// Selected index in sidebar
    pub sidebar_selected: usize,

    /// Selected header row (when editing headers)
    pub headers_selected: usize,

    pub yank_register: Option<String>
}

impl App {
    pub fn new() -> Self {
        let mut cursors = HashMap::new();
        cursors.insert(Panel::Sidebar, 0);
        cursors.insert(Panel::Url, 0);
        cursors.insert(Panel::Headers, 0);
        cursors.insert(Panel::Body, 0);
        cursors.insert(Panel::Response, 0);

        Self {
            mode: Mode::Normal,
            active_panel: Panel::Url,
            url_buffer: String::from("https://api.example.com"),
            body_buffer: String::new(),
            command_buffer: String::new(),
            headers: vec![
                ("Content-Type".to_string(), "application/json".to_string()),
            ],
            current_request: Request::default(),
            last_response: None,
            cursors,
            should_quit: false,
            collections: vec![
                "My Requests".to_string(),
                "API Tests".to_string(),
            ],
            sidebar_selected: 0,
            headers_selected: 0,
            pending_command: None,
            yank_register: None,
        }
    }

    /// Switch to a different mode
    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        // Clear command buffer when leaving command mode
        if mode != Mode::Command {
            self.command_buffer.clear();
        }
    }

    /// Switch to a different panel
    pub fn set_panel(&mut self, panel: Panel) {
        self.active_panel = panel;
    }

    /// Get the current cursor position for the active panel
    pub fn cursor(&self) -> usize {
        *self.cursors.get(&self.active_panel).unwrap_or(&0)
    }

    /// Set the cursor position for the active panel
    pub fn set_cursor(&mut self, pos: usize) {
        self.cursors.insert(self.active_panel, pos);
    }

    pub fn set_pending_command(&mut self, cmd: PendingCommand) {
        self.pending_command = Some(cmd);
    }

    pub fn clear_pending_command(&mut self) {
        self.pending_command = None;
    }

    pub fn set_yank_register(&mut self, register: String) {
        self.yank_register = Some(register)
    }

    /// Get the current buffer content for the active panel
    pub fn current_buffer(&self) -> &str {
        match self.active_panel {
            Panel::Url => &self.url_buffer,
            Panel::Body => &self.body_buffer,
            _ => "",
        }
    }

    /// Get mutable reference to current buffer
    pub fn current_buffer_mut(&mut self) -> Option<&mut String> {
        match self.active_panel {
            Panel::Url => Some(&mut self.url_buffer),
            Panel::Body => Some(&mut self.body_buffer),
            _ => None,
        }
    }

    /// Move cursor up one line (j in vim)
    pub fn move_cursor_down(&mut self) {
        let buffer = self.current_buffer();
        let cursor = self.cursor();

        // Find current line and column
        let (current_line, current_col) = self.cursor_to_line_col(buffer, cursor);

        // Split buffer into lines
        let lines: Vec<&str> = buffer.lines().collect();

        // If not on last line, move down
        if current_line + 1 < lines.len() {
            let next_line = lines[current_line + 1];
            // Try to maintain column, but clamp to line length
            let new_col = current_col.min(next_line.len());

            // Calculate new cursor position
            let new_cursor = self.line_col_to_cursor(buffer, current_line + 1, new_col);
            self.set_cursor(new_cursor);
        }
    }

    /// Move cursor up one line (k in vim)
    pub fn move_cursor_up(&mut self) {
        let buffer = self.current_buffer();
        let cursor = self.cursor();

        // Find current line and column
        let (current_line, current_col) = self.cursor_to_line_col(buffer, cursor);

        // If not on first line, move up
        if current_line > 0 {
            let lines: Vec<&str> = buffer.lines().collect();
            let prev_line = lines[current_line - 1];
            // Try to maintain column, but clamp to line length
            let new_col = current_col.min(prev_line.len());

            // Calculate new cursor position
            let new_cursor = self.line_col_to_cursor(buffer, current_line - 1, new_col);
            self.set_cursor(new_cursor);
        }
    }

    /// Convert linear cursor position to (line, column)
    pub fn cursor_to_line_col(&self, buffer: &str, cursor: usize) -> (usize, usize) {
        let mut current_pos = 0;

        for (line_num, line) in buffer.lines().enumerate() {
            let line_end = current_pos + line.len();

            if cursor <= line_end {
                // Cursor is on this line
                return (line_num, cursor - current_pos);
            }

            // +1 for the newline character
            current_pos = line_end + 1;
        }

        // If buffer is empty or cursor is at the very end
        let line_count = buffer.lines().count().max(1) - 1;
        (line_count, 0)
    }

    /// Convert (line, column) to linear cursor position
    pub fn line_col_to_cursor(&self, buffer: &str, line: usize, col: usize) -> usize {
        let mut cursor = 0;

        for (line_num, line_text) in buffer.lines().enumerate() {
            if line_num == line {
                return cursor + col;
            }
            // +1 for the newline character
            cursor += line_text.len() + 1;
        }

        cursor
    }

    pub fn move_word_forward(&mut self) {
        let buffer = self.current_buffer();
        let mut cursor = self.cursor();
        let chars: Vec<char> = buffer.chars().collect();

        if cursor >= chars.len() {
            return;
        }

        let current_char = chars[cursor];

        let is_word_char = |c: char| c.is_alphanumeric() || c == '_';

        if is_word_char(current_char) {
            while cursor < chars.len() && is_word_char(chars[cursor]) {
                cursor += 1;
            }
        } else if !current_char.is_whitespace() {
            while cursor < chars.len()
            && !chars[cursor].is_whitespace()
            && !is_word_char(chars[cursor]) {
                cursor += 1;
            }
        }

        while cursor < chars.len() && chars[cursor].is_whitespace() {
            cursor += 1;
        }

        self.set_cursor(cursor);
    }

    pub fn move_word_backwards(&mut self) {
        let buffer = self.current_buffer();
        let mut cursor = self.cursor();

        if cursor == 0 {
            return;
        }

        let chars: Vec<char> = buffer.chars().collect();

        cursor = cursor.saturating_sub(1);

        let is_word_char = |c: char| c.is_alphanumeric() || c == '_';

        while cursor > 0 && cursor < chars.len() && chars[cursor].is_whitespace() {
            cursor -= 1;
        }

        if cursor < chars.len() && !chars[cursor].is_whitespace() {
            let in_word = is_word_char(chars[cursor]);

            while cursor > 0 {
                let prev_char = chars[cursor - 1];
                if is_word_char(prev_char) != in_word || prev_char.is_whitespace() {
                    break;
                }
                cursor -= 1;
            }
        }

        self.set_cursor(cursor);
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
