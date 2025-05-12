use super::*;
pub fn create_wtxidrelay_message() -> Message {
    let payload = Vec::new();
    let checksum = calculate_checksum(&payload);
    Message {
        magic: TESTNET_MAGIC,
        command: "wtxidrelay".to_string(),
        payload,
        checksum,
    }
}