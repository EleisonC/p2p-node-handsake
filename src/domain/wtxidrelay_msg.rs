use super::*;

/// Creates a `wtxidrelay` message for the Bitcoin Testnet3 handshake.
/// This message tells the peer that the node supports relaying transactions
/// using witness transaction IDs (WTXIDs), which include SegWit data.
/// It has no payload and is sent to enable WTXID-based transaction relay.

pub fn create_wtxidrelay_message() -> Message {
    let payload = Vec::new();
    let checksum = calculate_checksum(&payload);
    Message {
        magic: TESTNET_MAGIC,
        command: String::from("wtxidrelay"),
        payload,
        checksum,
    }
}