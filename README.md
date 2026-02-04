# vreq

A vim-style HTTP client for the terminal, built with [ratatui](https://github.com/ratatouille-rs/ratatui).

## Layout

```
┌────────────┬─────────────────────┬─────────────────────┐
│            │ URL(GET)            │                     │
│ Collections│─────────────────────│                     │
│            │ Headers             │   Response          │
│            │─────────────────────│                     │
│            │ Body                │                     │
│            │                     │                     │
├────────────┴─────────────────────┴─────────────────────┤
│ -- NORMAL --                                            │
└─────────────────────────────────────────────────────────┘
```

Five panels: **Sidebar**, **URL**, **Headers**, **Body**, **Response**. The focused panel gets a cyan border. The status line at the bottom shows the current mode.

## Modes

| Mode    | Description                                          |
|---------|------------------------------------------------------|
| Normal  | Navigation, operators, panel switching               |
| Insert  | Text editing in the focused panel                    |
| Visual  | Character selection in the Response panel            |
| Command | Ex-style commands entered after `:`                  |

## Key Bindings

### Normal Mode

#### Mode switching

| Key       | Action                                  |
|-----------|-----------------------------------------|
| `i`       | Enter Insert mode                       |
| `a`       | Enter Insert mode (after cursor)        |
| `I`       | Enter Insert mode (start of line)       |
| `A`       | Enter Insert mode (end of line)         |
| `v`       | Enter Visual mode (focuses Response)    |
| `:`       | Enter Command mode                      |
| `q`       | Quit                                    |
| `Ctrl+C`  | Quit (any mode)                         |

#### Panel navigation

| Key            | Action                                  |
|----------------|-----------------------------------------|
| `Tab`          | Next panel                              |
| `Shift+Tab`    | Previous panel                          |
| `1`            | Focus Sidebar                           |
| `2`            | Focus URL                               |
| `3`            | Focus Headers                           |
| `4`            | Focus Body                              |
| `5`            | Focus Response                          |

#### Cursor motion

| Key    | Action                              |
|--------|-------------------------------------|
| `h`    | Left                                |
| `j`    | Down                                |
| `k`    | Up                                  |
| `l`    | Right                               |
| `w`    | Word forward                        |
| `b`    | Word backward                       |
| `0`    | Start of line                       |
| `$`    | End of line                         |
| `gg`   | Start of buffer                     |
| `G`    | End of buffer                       |

#### Operators

| Key    | Action                              |
|--------|-------------------------------------|
| `dd`   | Delete current line                 |
| `d$`   | Delete from cursor to end of line   |
| `yy`   | Yank current line                   |
| `y$`   | Yank from cursor to end of line     |
| `p`    | Paste yanked text below cursor line |

#### Sending requests

| Key     | Action                                          |
|---------|-------------------------------------------------|
| `Enter` | Send the current request, show response         |

### Insert Mode

| Key            | Action                                  |
|----------------|-----------------------------------------|
| `Esc` / `Ctrl+[` | Return to Normal mode                |
| Printable char | Insert character at cursor            |
| `Backspace`    | Delete character before cursor          |
| `Delete`       | Delete character at cursor              |
| `Enter`        | New line (Headers and Body only)        |
| Arrow keys     | Move cursor                             |

### Visual Mode

Entered with `v` in Normal mode. Automatically focuses the Response panel and anchors the selection at the current cursor position. All motions extend or shrink the selection.

| Key            | Action                                  |
|----------------|-----------------------------------------|
| `h` `j` `k` `l` | Move cursor / extend selection       |
| `w` `b`        | Word forward / backward                 |
| `0` `$`        | Start / end of line                     |
| `y`            | Yank selection to clipboard and register, return to Normal |
| `Esc` / `Ctrl+[` | Cancel selection, return to Normal   |

Yanking in Visual mode copies to both the internal yank register (paste with `p`) and the system clipboard via `xclip`.

### Command Mode

Entered with `:`. Type a command and press `Enter` to execute, or `Esc` to cancel.

| Command            | Action                              |
|--------------------|-------------------------------------|
| `:q` / `:quit`     | Quit                                |
| `:send`            | Send the current request            |
| `:clear`           | Clear the response                  |
| `:method <METHOD>` | Set HTTP method (GET, POST, PUT, DELETE, PATCH) |
| `:w` / `:write`    | Save current request (pending)      |
| `:wq`              | Save and quit (pending)             |
| `:save <name>`     | Save request with a name (pending)  |
| `:load <name>`     | Load a saved request (pending)      |

## Panels

### URL

Single-line buffer. Shows the current HTTP method in the panel title (e.g. `URL(GET)`). Change the method with `:method POST`.

### Headers

Multi-line buffer. One header per line in `Key: Value` format. Lines that don't contain a `:` are ignored when sending the request.

### Body

Multi-line buffer. Sent as the request body for POST, PUT, and PATCH requests. Empty bodies are not sent.

### Response

Read-only panel. Displays the status code and response body after a request is sent. Supports full cursor navigation in Normal mode and text selection in Visual mode. Scrolls automatically as the cursor moves.

### Sidebar

Lists saved request collections. Navigation and loading are pending implementation.

## Scrolling

All multi-line panels (Headers, Body, Response) scroll automatically to keep the cursor visible. Scrolling only moves when the cursor would leave the visible area.

## Debugging

A `log!` macro is available in all modules. It appends to `vreq.log` in the working directory:

```rust
log!("cursor={} mode={:?}", app.cursor(), app.mode);
```

Monitor live while the app is running:

```sh
tail -f vreq.log
```

## Building

```sh
cargo build
cargo run
```

Requires `xclip` to be installed for clipboard support in Visual mode.
