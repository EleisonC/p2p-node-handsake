use anyhow::Result;
use p2p_node_handshake::{HandTool, utils};

#[tokio::main]
async fn main() -> Result<()> {
    utils::hand_logger::init_logger();
    tracing::info!("Welcome to the Handshake");
    tracing::info!("Pre configure with two Testnet Bitcoin public nodes:\
        testnet-seed.bitcoin.jonasschnelli.ch:18333\
        seed.testnet.bitcoin.sprovoost.nl:18333\
        Default number of handshake attempts is 2
    ");
    let mut tool = HandTool::default();

    tool.set_max_handshake_attempts(3);
    tool.perform_handshake().await
}