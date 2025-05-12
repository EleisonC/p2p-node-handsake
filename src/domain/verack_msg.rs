use super::*;

pub fn create_verack_message() -> Message {
    let payload = Vec::new();
    let checksum = calculate_checksum(&payload);
    Message {
        magic: TESTNET_MAGIC,
        command: "verack".to_string(),
        payload,
        checksum,
    }
}