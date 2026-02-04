#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        use std::io::Write;
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("vreq.log")
        {
            let _ = writeln!(f, $($arg)*);
        }
    }};
}
