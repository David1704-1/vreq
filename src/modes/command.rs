use crate::app::{App, Mode, Panel};
use crate::http::{Method, Request, send_request};
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.set_mode(Mode::Normal);
        }

        KeyCode::Enter => {
            execute_command(app);
            app.set_mode(Mode::Normal);
        }

        KeyCode::Char(c) => {
            app.command_buffer.push(c);
        }
        KeyCode::Backspace => {
            app.command_buffer.pop();
        }

        _ => {}
    }
}

fn execute_command(app: &mut App) {
    let cmd = app.command_buffer.trim();

    match cmd {
        "q" | "quit" => {
            app.should_quit = true;
        }
        "w" | "write" => {
            // TODO: Save current request to a collection
        }
        "wq" => {
            // TODO: Save and quit
        }
        _ if cmd.starts_with("method ") => {
            let method_split: Vec<&str> = cmd.split(' ').collect();
            app.current_request.method = Method::from(method_split[1].to_uppercase());
        }
        "send" => {
            let mut req = Request::new(app.current_request.method, app.url_buffer.clone());
            for (key, value) in app.parsed_headers() {
                req.headers.insert(key, value);
            }
            req.body = app.body_buffer.clone();
            app.current_request = req;
            let response = send_request(&app.current_request).unwrap_or_default();
            app.last_response = Some(response);
            app.update_response_buffer();
            app.set_panel(Panel::Response);
        }
        "clear" => {
            app.last_response = None;
            app.update_response_buffer();
        }
        _ if cmd.starts_with("load ") => {
            // TODO: Load a saved request
        }
        _ if cmd.starts_with("save ") => {
            // TODO: Save request with specific name
        }
        _ => {}
    }
}
