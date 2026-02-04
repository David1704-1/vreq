use crate::app::{App, Mode, Panel};
use crate::http::{Method, Request, send_request};
use crate::persistence::{Collection, save_collection, load_collection};
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.set_mode(Mode::Normal);
        }

        KeyCode::Enter => {
            let _ = execute_command(app);
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

fn execute_command(app: &mut App) -> Result<(), Box<dyn std::error::Error>>{
    let cmd = app.command_buffer.trim();

    match cmd {
        "q" | "quit" => {
            app.should_quit = true;
            Ok(())
        }
        _ if cmd.starts_with("method ") => {
            let method_split: Vec<&str> = cmd.split(' ').collect();
            app.current_request.method = Method::from(method_split[1].to_uppercase());
            Ok(())
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

            Ok(())
        }
        "clear" => {
            app.last_response = None;
            app.update_response_buffer();
            Ok(())
        }
        _ if cmd.starts_with("load ") => {
            let cmd_split: Vec<&str> = cmd.split(' ').collect();
            let collection: Collection = load_collection(cmd_split[1])?;
            let req = collection.saved_request.request;
            app.url_buffer = req.url.clone();
            app.headers_buffer = req.headers.iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<_>>()
                .join("\n");
            app.body_buffer = req.body.clone();
            app.current_request = req;
            Ok(())
        }
        _ if cmd.starts_with("save ") => {
            let cmd_split: Vec<&str> = cmd.split(' ').collect();
            app.current_request.body = app.body_buffer.clone();
            app.current_request.url = app.url_buffer.clone();

            for (key, value) in app.parsed_headers() {
                app.current_request.headers.insert(key, value);
            }
            let collection = Collection::new(String::from(cmd_split[1]), app.current_request.clone());
            save_collection(&collection)
        }
        _ => {
            Ok(())
        }
    }
}
