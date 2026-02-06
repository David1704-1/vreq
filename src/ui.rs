use crate::app::{App, Mode, Panel};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(f.area());

    render_main_content(f, app, chunks[0]);
    render_status_line(f, app, chunks[1]);
    set_cursor(f, app);
}

fn render_main_content(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
        ])
        .split(area);

    render_sidebar(f, app, chunks[0]);
    render_request_builder(f, app, chunks[1]);
    render_response_viewer(f, app, chunks[2]);
}

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
        .map(|name| {
            let display_name = name.strip_suffix(".json").unwrap_or(name);
            ListItem::new(display_name)
        })
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
                .bg(Color::DarkGray)
                .fg(Color::White),
        )
        .highlight_symbol(">> ");

    // Create a ListState to track the selected item
    let mut list_state = ListState::default();
    list_state.select(Some(app.selected_collection_index));

    f.render_stateful_widget(list, area, &mut list_state);
}

fn render_request_builder(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(8),
            Constraint::Min(0),
        ])
        .split(area);

    render_url_input(f, app, chunks[0]);
    render_headers_table(f, app, chunks[1]);
    render_body_input(f, app, chunks[2]);
}

fn render_url_input(f: &mut Frame, app: &App, area: Rect) {
    let is_focused = app.active_panel == Panel::Url;
    let border_style = if is_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let title = format!("URL({})", app.current_request.method);
    let url_widget = Paragraph::new(app.url_buffer.as_str())
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(border_style),
        );

    f.render_widget(url_widget, area);
}

fn render_headers_table(f: &mut Frame, app: &mut App, area: Rect) {
    let is_focused = app.active_panel == Panel::Headers;
    let border_style = if is_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let visible_rows = area.height.saturating_sub(2);
    let cursor_line = {
        let (line, _) = app.cursor_to_line_col(&app.headers_buffer, app.cursor());
        line as u16
    };
    app.update_scroll(Panel::Headers, cursor_line, visible_rows);
    let scroll = app.scroll_offset(Panel::Headers);

    let headers_widget = Paragraph::new(app.headers_buffer.as_str())
        .scroll((scroll, 0))
        .block(
            Block::default()
                .title("Headers")
                .borders(Borders::ALL)
                .border_style(border_style),
        );

    f.render_widget(headers_widget, area);
}

fn render_body_input(f: &mut Frame, app: &mut App, area: Rect) {
    let is_focused = app.active_panel == Panel::Body;
    let border_style = if is_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let visible_rows = area.height.saturating_sub(2);
    let cursor_line = {
        let (line, _) = app.cursor_to_line_col(&app.body_buffer, app.cursor());
        line as u16
    };
    app.update_scroll(Panel::Body, cursor_line, visible_rows);
    let scroll = app.scroll_offset(Panel::Body);

    let body_widget = Paragraph::new(app.body_buffer.as_str())
        .scroll((scroll, 0))
        .block(
            Block::default()
                .title("Body")
                .borders(Borders::ALL)
                .border_style(border_style),
        );

    f.render_widget(body_widget, area);
}

fn render_response_viewer(f: &mut Frame, app: &mut App, area: Rect) {
    let is_focused = app.active_panel == Panel::Response;
    let border_style = if is_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let visible_rows = area.height.saturating_sub(2);
    let cursor_line = {
        let cursor = *app.cursors.get(&Panel::Response).unwrap_or(&0);
        let (line, _) = app.cursor_to_line_col(&app.response_buffer, cursor);
        line as u16
    };
    app.update_scroll(Panel::Response, cursor_line, visible_rows);
    let scroll = app.scroll_offset(Panel::Response);

    let raw = &app.response_buffer;
    let text = if app.mode == Mode::Visual {
        if let Some(anchor) = app.visual_anchor {
            let cursor = *app.cursors.get(&Panel::Response).unwrap_or(&0);
            let start = anchor.min(cursor);
            let end = (anchor.max(cursor) + 1).min(raw.len());
            build_highlighted_text(raw, start, end)
        } else {
            Text::raw(raw.as_str())
        }
    } else {
        Text::raw(raw.as_str())
    };

    let response_widget = Paragraph::new(text)
        .scroll((scroll, 0))
        .block(
            Block::default()
                .title("Response")
                .borders(Borders::ALL)
                .border_style(border_style),
        );

    f.render_widget(response_widget, area);
}

fn render_status_line(f: &mut Frame, app: &App, area: Rect) {
    let mode_text = match app.mode {
        Mode::Normal => "-- NORMAL --",
        Mode::Insert => "-- INSERT --",
        Mode::Visual => "-- VISUAL --",
        Mode::Command => "",
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

fn set_cursor(f: &mut Frame, app: &App) {
    if app.mode == Mode::Command {
        return;
    }

    match app.active_panel {
        Panel::Url | Panel::Headers | Panel::Body | Panel::Response => {
            if let Some((x, y)) = calculate_cursor_position(f, app) {
                f.set_cursor_position((x, y));
            }
        }
        _ => {}
    }
}

fn calculate_cursor_position(f: &mut Frame, app: &App) -> Option<(u16, u16)> {
    let area = f.area();

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
        ])
        .split(main_chunks[0]);

    let request_builder_area = horizontal_chunks[1];

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(8),
            Constraint::Min(0),
        ])
        .split(request_builder_area);

    match app.active_panel {
        Panel::Url => {
            let url_area = vertical_chunks[0];
            let cursor = app.cursor();
            let cursor_offset = cursor.min(app.url_buffer.len()) as u16;

            Some((
                url_area.x + 1 + cursor_offset,
                url_area.y + 1,
            ))
        }
        Panel::Headers => {
            let headers_area = vertical_chunks[1];
            let cursor = app.cursor();
            let (line, col) = app.cursor_to_line_col(&app.headers_buffer, cursor);
            let scroll = app.scroll_offset(Panel::Headers);

            Some((
                headers_area.x + 1 + col as u16,
                headers_area.y + 1 + line as u16 - scroll,
            ))
        }
        Panel::Body => {
            let body_area = vertical_chunks[2];
            let cursor = app.cursor();
            let (line, col) = app.cursor_to_line_col(&app.body_buffer, cursor);
            let scroll = app.scroll_offset(Panel::Body);

            Some((
                body_area.x + 1 + col as u16,
                body_area.y + 1 + line as u16 - scroll,
            ))
        }
        Panel::Response => {
            let response_area = horizontal_chunks[2];
            let cursor = *app.cursors.get(&Panel::Response).unwrap_or(&0);
            let buffer = app.response_text();
            let (line, col) = app.cursor_to_line_col(buffer, cursor);
            let scroll = app.scroll_offset(Panel::Response);

            Some((
                response_area.x + 1 + col as u16,
                response_area.y + 1 + line as u16 - scroll,
            ))
        }
        _ => None,
    }
}

fn build_highlighted_text(raw: &str, start: usize, end: usize) -> Text<'_> {
    let normal_style = Style::default();
    let sel_style = Style::default()
        .bg(Color::Cyan)
        .fg(Color::Black)
        .add_modifier(Modifier::BOLD);

    let mut lines = Vec::new();
    let mut pos = 0;

    for line_str in raw.lines() {
        let line_start = pos;
        let line_end = pos + line_str.len();
        pos = line_end + 1;

        if end <= line_start || start >= line_end {
            lines.push(Line::from(Span::styled(line_str, normal_style)));
            continue;
        }

        let sel_start = start.max(line_start) - line_start;
        let sel_end = end.min(line_end) - line_start;

        let mut spans = Vec::new();
        if sel_start > 0 {
            spans.push(Span::styled(&line_str[..sel_start], normal_style));
        }
        spans.push(Span::styled(&line_str[sel_start..sel_end], sel_style));
        if sel_end < line_str.len() {
            spans.push(Span::styled(&line_str[sel_end..], normal_style));
        }
        lines.push(Line::from(spans));
    }

    Text::from(lines)
}
