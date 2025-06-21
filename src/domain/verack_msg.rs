use super::*;

/// Creates a `verack` message for the Bitcoin Testnet3 handshake.
/// This message acknowledges a successful `version` message from the peer,
/// confirming that the connection is established.
/// It has no payload and completes the handshake process.

pub fn create_verack_message() -> Message {
    let payload = Vec::new();
    let checksum = calculate_checksum(&payload);
    Message {
        magic: TESTNET_MAGIC,
        command: String::from("verack"),
        payload,
        checksum,
    }
}