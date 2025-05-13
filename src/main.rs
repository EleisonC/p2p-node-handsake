use anyhow::Result;
use p2p_node_handshake::{HandTool, utils};

#[tokio::main]
async fn main() -> Result<()> {
    utils::hand_logger::init_logger();
    let mut tool = HandTool::default();

    tool.set_max_handshake_attempts(3);
    tool.perform_handshake().await
}