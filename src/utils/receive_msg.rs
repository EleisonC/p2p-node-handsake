
use std::net::TcpStream;
use std::io::Read;
use anyhow::Result;
use super::*;
pub fn receive_message(stream: &mut TcpStream) -> Result<Message> {
    let mut header = [0u8; 24];
    let mut total_read = 0;
    while total_read < 24 {
        match stream.read(&mut header[total_read..]) {
            Ok(0) => return Err(anyhow::anyhow!("Peer closed connection unexpectedly")),
            Ok(n) => total_read += n,
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock || e.kind() == std::io::ErrorKind::TimedOut => {
                return Err(anyhow::anyhow!("Timed out waiting for peer response"));
            }
            Err(e) => return Err(e.into()),
        }
    }
    let magic = u32::from_be_bytes(header[0..4].try_into()?);
    let command_bytes = &header[4..16];
    let command = String::from_utf8_lossy(command_bytes)
        .trim_end_matches('\0')
        .to_string();
    let payload_len = u32::from_le_bytes(header[16..20].try_into()?) as usize;
    let header_checksum = header[20..24].try_into()?;
    let mut payload = vec![0u8; payload_len];
    if payload_len > 0 {
        let mut total_read = 0;
        while total_read < payload_len {
            match stream.read(&mut payload[total_read..]) {
                Ok(0) => return Err(anyhow::anyhow!("Peer closed connection while reading payload")),
                Ok(n) => total_read += n,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock || e.kind() == std::io::ErrorKind::TimedOut => {
                    return Err(anyhow::anyhow!("Timed out reading payload"));
                }
                Err(e) => return Err(e.into()),
            }
        }
    }
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