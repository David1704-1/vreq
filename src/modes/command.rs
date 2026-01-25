use crate::app::{App, Mode};
use crossterm::event::{KeyCode, KeyEvent};

/// Handle key events in Command mode
pub fn handle_key(app: &mut App, key: KeyEvent) {
    match key.code {
        // Exit command mode
        KeyCode::Esc => {
            app.set_mode(Mode::Normal);
        }

        // Execute command
        KeyCode::Enter => {
            execute_command(app);
            app.set_mode(Mode::Normal);
        }

        // Edit command buffer
        KeyCode::Char(c) => {
            app.command_buffer.push(c);
        }
        KeyCode::Backspace => {
            app.command_buffer.pop();
        }

        _ => {
            // Ignore other keys
        }
    }
}

/// Execute a command from the command buffer
fn execute_command(app: &mut App) {
    let cmd = app.command_buffer.trim();

    // TODO: Implement command parsing and execution
    // Below are skeleton handlers for planned commands

    match cmd {
        "q" | "quit" => {
            // TODO: Quit the application
            // Check if there are unsaved changes first
            app.should_quit = true;
        }
        "w" | "write" => {
            // TODO: Save current request to a collection
            // Steps:
            // 1. Prompt for request name (or use separate command :w <name>)
            // 2. Serialize current request state
            // 3. Save to file using persistence module
        }
        "wq" => {
            // TODO: Save and quit
            // Combine write + quit
        }
        _ if cmd.starts_with("method ") => {
            // TODO: Set HTTP method
            // Parse: "method GET", "method POST", etc.
            // let method = &cmd[7..].trim().to_uppercase();
            // app.current_request.method = method;
        }
        _ if cmd.starts_with("header add ") => {
            // TODO: Add a header
            // Parse: "header add Content-Type application/json"
            // Split into key and value
            // app.headers.push((key, value));
        }
        _ if cmd.starts_with("header rm ") => {
            // TODO: Remove a header
            // Parse: "header rm Content-Type"
            // Find and remove header with matching key
        }
        "send" => {
            // TODO: Send HTTP request
            // Same as pressing Enter in normal mode
        }
        "clear" => {
            // TODO: Clear response panel
            // app.last_response = None;
        }
        _ if cmd.starts_with("load ") => {
            // TODO: Load a saved request
            // Parse: "load my-request"
            // Look up request in collections
            // Populate app state with loaded request
        }
        _ if cmd.starts_with("save ") => {
            // TODO: Save request with specific name
            // Parse: "save my-request"
        }
        _ => {
            // Unknown command - could show error message in status line
            // For now, just ignore
        }
    }
}
