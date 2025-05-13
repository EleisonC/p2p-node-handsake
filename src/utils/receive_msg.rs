use anyhow::Result;
use tokio::io::{AsyncReadExt, ReadBuf};
use tokio::net::TcpStream;
use crate::{Message, TESTNET_MAGIC, calculate_checksum};

pub async fn receive_message(stream: &mut TcpStream) -> Result<Message> {
    // Read 24-byte header
    let mut header = [0u8; 24];
    let mut header_buf = ReadBuf::new(&mut header);
    let mut total_read = 0;
    while total_read < 24 {
        let n = stream
            .read_buf(&mut header_buf)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to read header: {}", e))?;
        if n == 0 {
            return Err(anyhow::anyhow!("Peer closed connection unexpectedly"));
        }
        total_read += n;
    }

    // Parse header
    let magic = u32::from_be_bytes(header[0..4].try_into()?);
    let command_bytes = &header[4..16];
    let command = String::from_utf8_lossy(command_bytes)
        .trim_end_matches('\0')
        .to_string();
    let payload_len = u32::from_le_bytes(header[16..20].try_into()?) as usize;
    let header_checksum = header[20..24].try_into()?;

    // Read payload
    let mut payload = vec![0u8; payload_len];
    let mut payload_buf = ReadBuf::new(&mut payload);
    let mut total_read = 0;
    while total_read < payload_len {
        let n = stream
            .read_buf(&mut payload_buf)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to read payload: {}", e))?;
        if n == 0 {
            return Err(anyhow::anyhow!("Peer closed connection while reading payload"));
        }
        total_read += n;
    }

    // Validate magic and checksum
    if magic != TESTNET_MAGIC {
        anyhow::bail!("Invalid magic: {:x}", magic);
    }
    let calculated_checksum = calculate_checksum(&payload);
    if header_checksum != calculated_checksum {
        anyhow::bail!("Checksum mismatch");
    }

    Ok(Message {
        magic,
        command,
        payload,
        checksum: header_checksum,
    })
}