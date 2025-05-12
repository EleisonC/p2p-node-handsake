use anyhow::Result;
use p2p_node_handshake::{HandTool, utils};

fn main() -> Result<()> {
    utils::hand_logger::init_logger();
    tracing::trace!("Welcome to the Handshake");
    tracing::trace!("Pre configure with two Testnet Bitcoin public nodes:\
    testnet-seed.bitcoin.jonasschnelli.ch:18333\
    seed.testnet.bitcoin.sprovoost.nl:18333
    ");
    tracing::trace!("Default number of handshake attempts is 2");
    let mut tool = HandTool::default();
    
    tool.set_max_handshake_attempts(3);
    tool.perform_handshake()
}