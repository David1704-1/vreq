use crate::app::{App, Mode, Panel};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

/// Main render function - draws the entire UI
pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),      // Main content
            Constraint::Length(1),   // Status line
        ])
        .split(f.area());

    // Render main content (3-column layout)
    render_main_content(f, app, chunks[0]);

    // Render status line at bottom (mode indicator + command line)
    render_status_line(f, app, chunks[1]);

    // Set cursor position based on mode and active panel
    set_cursor(f, app);
}

/// Render the main 3-column layout
fn render_main_content(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),  // Sidebar
            Constraint::Percentage(40),  // Request builder
            Constraint::Percentage(40),  // Response viewer
        ])
        .split(area);

    render_sidebar(f, app, chunks[0]);
    render_request_builder(f, app, chunks[1]);
    render_response_viewer(f, app, chunks[2]);
}

/// Render the sidebar (collections/history)
fn render_sidebar(f: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.active_panel == Panel::Sidebar;
    let border_style = if is_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let items: Vec<ListItem> = app
        .collections
        .iter()
        .map(|name| ListItem::new(name.as_str()))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Collections")
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::DarkGray),
        );

    f.render_widget(list, area);

    // TODO: Implement sidebar selection highlighting
    // TODO: Add keyboard navigation (j/k to move selection)
    // TODO: Add ability to load saved requests
}

/// Render the request builder (URL, headers, body)
fn render_request_builder(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),      // URL input
            Constraint::Length(8),      // Headers
            Constraint::Min(0),         // Body
        ])
        .split(area);

    render_url_input(f, app, chunks[0]);
    render_headers_table(f, app, chunks[1]);
    render_body_input(f, app, chunks[2]);
}

/// Render the URL input panel
fn render_url_input(f: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.active_panel == Panel::Url;
    let border_style = if is_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let url_widget = Paragraph::new(app.url_buffer.as_str())
        .block(
            Block::default()
                .title("URL (GET)")
                .borders(Borders::ALL)
                .border_style(border_style),
        );

    f.render_widget(url_widget, area);

    // TODO: Show cursor position when in Insert mode
    // TODO: Implement text editing (insert, delete, move cursor)
    // TODO: Add HTTP method selector (GET, POST, PUT, DELETE, etc.)
}

/// Render the headers table
fn render_headers_table(f: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.active_panel == Panel::Headers;
    let border_style = if is_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    // Format headers as list items
    let items: Vec<ListItem> = app
        .headers
        .iter()
        .map(|(key, value)| {
            ListItem::new(format!("{}: {}", key, value))
        })
        .collect();

    let headers_list = List::new(items)
        .block(
            Block::default()
                .title("Headers")
                .borders(Borders::ALL)
                .border_style(border_style),
        );

    f.render_widget(headers_list, area);

    // TODO: Implement table-like editing (navigate rows, edit key/value)
    // TODO: Add ability to add/remove headers
    // TODO: Highlight selected row
}

/// Render the request body input
fn render_body_input(f: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.active_panel == Panel::Body;
    let border_style = if is_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let body_widget = Paragraph::new(app.body_buffer.as_str())
        .block(
            Block::default()
                .title("Body")
                .borders(Borders::ALL)
                .border_style(border_style),
        );

    f.render_widget(body_widget, area);

    // TODO: Multi-line text editing
    // TODO: Syntax highlighting for JSON
    // TODO: Body type selector (JSON, form-data, raw, etc.)
}

/// Render the response viewer
fn render_response_viewer(f: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.active_panel == Panel::Response;
    let border_style = if is_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let response_text = if let Some(ref response) = app.last_response {
        // TODO: Format response nicely (status, headers, body)
        // TODO: Syntax highlighting for JSON responses
        // TODO: Add tabs for different views (body, headers, timing)
        format!("Status: {}\n\n{}", response.status, response.body)
    } else {
        "No response yet. Press Enter to send request.".to_string()
    };

    let response_widget = Paragraph::new(response_text)
        .block(
            Block::default()
                .title("Response")
                .borders(Borders::ALL)
                .border_style(border_style),
        );

    f.render_widget(response_widget, area);
}

/// Render the status line (shows mode and command input)
fn render_status_line(f: &mut Frame, app: &App, area: Rect) {
    let mode_text = match app.mode {
        Mode::Normal => "-- NORMAL --",
        Mode::Insert => "-- INSERT --",
        Mode::Command => "",  // Command line will show the actual command
    };

    let content = if app.mode == Mode::Command {
        format!(":{}", app.command_buffer)
    } else {
        mode_text.to_string()
    };

    let status_line = Paragraph::new(content)
        .style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(status_line, area);
}

/// Set cursor position based on current mode and active panel
fn set_cursor(f: &mut Frame, app: &App) {
    // Only show cursor in Normal and Insert modes (not Command mode)
    // Command mode has its own cursor in the status line
    if app.mode != Mode::Normal && app.mode != Mode::Insert {
        return;
    }

    // Only show cursor for editable text panels
    match app.active_panel {
        Panel::Url | Panel::Body => {
            if let Some((x, y)) = calculate_cursor_position(f, app) {
                f.set_cursor_position((x, y));
            }
        }
        _ => {
            // No cursor for sidebar, headers, or response panels
        }
    }
}

/// Calculate the screen position of the cursor
fn calculate_cursor_position(f: &mut Frame, app: &App) -> Option<(u16, u16)> {
    let area = f.area();

    // Recreate the layout to get panel positions
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),      // Main content
            Constraint::Length(1),   // Status line
        ])
        .split(area);

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),  // Sidebar
            Constraint::Percentage(40),  // Request builder
            Constraint::Percentage(40),  // Response viewer
        ])
        .split(main_chunks[0]);

    let request_builder_area = horizontal_chunks[1];

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),      // URL input
            Constraint::Length(8),      // Headers
            Constraint::Min(0),         // Body
        ])
        .split(request_builder_area);

    match app.active_panel {
        Panel::Url => {
            let url_area = vertical_chunks[0];
            let cursor = app.cursor();

            // Cursor is inside the border: +1 for left border, +1 for top border
            // Cursor position is clamped to buffer length
            let cursor_offset = cursor.min(app.url_buffer.len()) as u16;

            Some((
                url_area.x + 1 + cursor_offset,
                url_area.y + 1,
            ))
        }
        Panel::Body => {
            let body_area = vertical_chunks[2];
            let cursor = app.cursor();
            let buffer = &app.body_buffer;

            // Convert cursor position to (line, column)
            let (line, col) = app.cursor_to_line_col(buffer, cursor);

            // Calculate screen position
            // x = left border (1) + column position
            // y = top border (1) + line number
            Some((
                body_area.x + 1 + col as u16,
                body_area.y + 1 + line as u16,
            ))
        }
        _ => None,
    }
}
