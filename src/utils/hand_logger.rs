use tracing_subscriber::fmt;

pub fn init_logger() {
    fmt()
        .with_writer(std::io::stderr)
        .with_target(false)
        .with_file(false)
        .with_line_number(false)
        .init();

    tracing::info!("######## Welcome to HS ########");
    tracing::info!("Pre configure with two Testnet Bitcoin public nodes:\
        testnet-seed.bitcoin.jonasschnelli.ch:18333\
        seed.testnet.bitcoin.sprovoost.nl:18333\
        Default number of handshake attempts is 2
    ");
}
