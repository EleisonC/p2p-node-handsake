use std::net::SocketAddr;
use byteorder::{BigEndian, WriteBytesExt};
use anyhow::Result;

pub fn serialize_network_address(addr: SocketAddr, services: u64) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    buffer.write_u64::<BigEndian>(services)?;
    match addr {
        SocketAddr::V4(addr) => {
            buffer.extend_from_slice(&[0u8; 10]);
            buffer.extend_from_slice(&[0xFF, 0xFF]);
            buffer.extend_from_slice(&addr.ip().octets());
        }
        SocketAddr::V6(addr) => {
            buffer.extend_from_slice(&addr.ip().octets());
        }
    }
    buffer.write_u16::<BigEndian>(addr.port())?;
    Ok(buffer)
}