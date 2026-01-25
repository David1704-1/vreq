use crate::app::{App, Mode};
use crate::modes;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Main key event dispatcher - routes key events to the appropriate mode handler
/// Returns false if the app should quit, true otherwise
pub fn handle_key_event(app: &mut App, key: KeyEvent) -> bool {
    // Global quit handler (Ctrl+C in any mode)
    if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
        return false;
    }

    // Dispatch to mode-specific handlers
    match app.mode {
        Mode::Normal => modes::normal::handle_key(app, key),
        Mode::Insert => modes::insert::handle_key(app, key),
        Mode::Command => modes::command::handle_key(app, key),
    }

    !app.should_quit
}

// =============================================================================
// PLANNED KEYBINDINGS (for reference when implementing)
// =============================================================================
//
// NORMAL MODE:
//   Navigation:
//     h, j, k, l      - Move cursor left, down, up, right (vim-style)
//     w, b            - Move word forward/backward
//     0, $            - Move to start/end of line
//     gg, G           - Move to top/bottom of buffer
//     Ctrl+d, Ctrl+u  - Scroll down/up half page
//
//   Panel switching:
//     Tab             - Next panel
//     Shift+Tab       - Previous panel
//     1-5             - Jump to specific panel
//
//   Mode switching:
//     i               - Enter Insert mode
//     a               - Enter Insert mode (after cursor)
//     I               - Enter Insert mode at start of line
//     A               - Enter Insert mode at end of line
//     o               - Insert new line below
//     O               - Insert new line above
//     :               - Enter Command mode
//     /               - Search mode (future feature)
//
//   Actions:
//     Enter           - Send HTTP request
//     d               - Delete (dd for line, dw for word, etc.)
//     y               - Yank/copy (yy for line)
//     p               - Paste
//     u               - Undo
//     Ctrl+r          - Redo
//
// INSERT MODE:
//     Esc             - Return to Normal mode
//     Ctrl+[          - Return to Normal mode (alternative)
//     All printable chars - Insert text
//     Backspace       - Delete character before cursor
//     Delete          - Delete character at cursor
//     Arrow keys      - Move cursor
//     Enter           - New line (in body panel)
//
// COMMAND MODE:
//     Esc             - Cancel and return to Normal mode
//     Enter           - Execute command
//     Backspace       - Delete character
//
//   Commands:
//     :q              - Quit
//     :w              - Save current request
//     :wq             - Save and quit
//     :method <GET|POST|PUT|DELETE|PATCH> - Set HTTP method
//     :header add <key> <value> - Add header
//     :header rm <key> - Remove header
//     :send           - Send request
//     :clear          - Clear response
//     :load <name>    - Load saved request
//
// =============================================================================
