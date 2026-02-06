use crate::http::{Request, Response};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Insert,
    Command,
    Visual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Panel {
    Sidebar,
    Url,
    Headers,
    Body,
    Response,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PendingCommand {
    Delete,
    Yank,
    Goto,
}

pub struct App {
    pub mode: Mode,
    pub active_panel: Panel,
    pub url_buffer: String,
    pub body_buffer: String,
    pub command_buffer: String,
    pub headers_buffer: String,
    pub pending_command: Option<PendingCommand>,
    pub current_request: Request,
    pub last_response: Option<Response>,
    pub response_buffer: String,
    pub cursors: HashMap<Panel, usize>,
    pub scroll_offsets: HashMap<Panel, u16>,
    pub should_quit: bool,
    pub collections: Vec<String>,
    pub yank_register: Option<String>,
    pub visual_anchor: Option<usize>,
    pub selected_collection_index: usize,
}

impl App {
    pub fn new() -> Self {
        let mut cursors = HashMap::new();
        cursors.insert(Panel::Sidebar, 0);
        cursors.insert(Panel::Url, 0);
        cursors.insert(Panel::Headers, 0);
        cursors.insert(Panel::Body, 0);
        cursors.insert(Panel::Response, 0);

        let scroll_offsets = HashMap::new();

        Self {
            mode: Mode::Normal,
            active_panel: Panel::Url,
            url_buffer: String::from("https://api.example.com"),
            body_buffer: String::new(),
            command_buffer: String::new(),
            headers_buffer: String::from("Content-Type: application/json"),
            current_request: Request::default(),
            last_response: None,
            response_buffer: String::from("No response yet. Press Enter to send request."),
            cursors,
            scroll_offsets,
            should_quit: false,
            collections: vec![
                "My Requests".to_string(),
                "API Tests".to_string(),
            ],
            pending_command: None,
            yank_register: None,
            visual_anchor: None,
            selected_collection_index: 0,
        }
    }

    pub fn response_text(&self) -> &str {
        &self.response_buffer
    }

    pub fn set_collections(&mut self, collections: Vec<String>) {
        self.collections = collections;

    }
    pub fn update_response_buffer(&mut self) {
        self.response_buffer = if let Some(ref response) = self.last_response {
            format!("Status: {}\n\n{}", response.status, response.body)
        } else {
            "No response yet. Press Enter to send request.".to_string()
        };
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        if mode != Mode::Command {
            self.command_buffer.clear();
        }
    }

    pub fn set_panel(&mut self, panel: Panel) {
        self.active_panel = panel;
    }

    pub fn cursor(&self) -> usize {
        *self.cursors.get(&self.active_panel).unwrap_or(&0)
    }

    pub fn set_cursor(&mut self, pos: usize) {
        self.cursors.insert(self.active_panel, pos);
    }

    pub fn update_scroll(&mut self, panel: Panel, cursor_line: u16, visible_rows: u16) {
        let scroll = self.scroll_offsets.entry(panel).or_insert(0);
        if cursor_line < *scroll {
            *scroll = cursor_line;
        } else if cursor_line >= *scroll + visible_rows {
            *scroll = cursor_line - visible_rows + 1;
        }
    }

    pub fn scroll_offset(&self, panel: Panel) -> u16 {
        *self.scroll_offsets.get(&panel).unwrap_or(&0)
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

    pub fn current_buffer(&self) -> &str {
        match self.active_panel {
            Panel::Url => &self.url_buffer,
            Panel::Headers => &self.headers_buffer,
            Panel::Body => &self.body_buffer,
            Panel::Response => &self.response_buffer,
            _ => "",
        }
    }

    pub fn current_buffer_mut(&mut self) -> Option<&mut String> {
        match self.active_panel {
            Panel::Url => Some(&mut self.url_buffer),
            Panel::Headers => Some(&mut self.headers_buffer),
            Panel::Body => Some(&mut self.body_buffer),
            _ => None,
        }
    }

    pub fn parsed_headers(&self) -> Vec<(String, String)> {
        self.headers_buffer
            .lines()
            .filter_map(|line| {
                let (key, value) = line.split_once(':')?;
                Some((key.trim().to_string(), value.trim().to_string()))
            })
            .collect()
    }

    pub fn move_cursor_down(&mut self) {
        let buffer = self.current_buffer();
        let cursor = self.cursor();
        let (current_line, current_col) = self.cursor_to_line_col(buffer, cursor);
        let lines: Vec<&str> = buffer.lines().collect();

        if current_line + 1 < lines.len() {
            let next_line = lines[current_line + 1];
            let new_col = current_col.min(next_line.len());
            let new_cursor = self.line_col_to_cursor(buffer, current_line + 1, new_col);
            self.set_cursor(new_cursor);
        }
    }

    pub fn move_cursor_up(&mut self) {
        let buffer = self.current_buffer();
        let cursor = self.cursor();
        let (current_line, current_col) = self.cursor_to_line_col(buffer, cursor);

        if current_line > 0 {
            let lines: Vec<&str> = buffer.lines().collect();
            let prev_line = lines[current_line - 1];
            let new_col = current_col.min(prev_line.len());
            let new_cursor = self.line_col_to_cursor(buffer, current_line - 1, new_col);
            self.set_cursor(new_cursor);
        }
    }

    pub fn cursor_to_line_col(&self, buffer: &str, cursor: usize) -> (usize, usize) {
        let mut current_pos = 0;

        for (line_num, line) in buffer.lines().enumerate() {
            let line_end = current_pos + line.len();

            if cursor <= line_end {
                return (line_num, cursor - current_pos);
            }

            current_pos = line_end + 1;
        }

        let line_count = buffer.lines().count().max(1) - 1;
        (line_count, 0)
    }

    pub fn line_col_to_cursor(&self, buffer: &str, line: usize, col: usize) -> usize {
        let mut cursor = 0;

        for (line_num, line_text) in buffer.lines().enumerate() {
            if line_num == line {
                return cursor + col;
            }
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

    pub fn select_next_collection(&mut self) {
        if !self.collections.is_empty() && self.selected_collection_index < self.collections.len() - 1 {
            self.selected_collection_index += 1;
        }
    }

    pub fn select_previous_collection(&mut self) {
        if self.selected_collection_index > 0 {
            self.selected_collection_index -= 1;
        }
    }

    pub fn selected_collection(&self) -> Option<&str> {
        self.collections.get(self.selected_collection_index).map(|s| s.as_str())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
