use tracing_subscriber::fmt;

pub fn init_logger() {
    fmt()
        .with_writer(std::io::stderr)
        .with_target(false)
        .with_file(false)
        .with_line_number(false)
        .init();
}
