use std::net::ToSocketAddrs;
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::{Context, Result};
use byteorder::{BigEndian, WriteBytesExt};
use rand::Rng;
use super::*;

pub fn create_version_message(
    target_node: &str,
    start_height: i32,
    protocol_version: i32,
    relay: bool,
) -> Result<Message> {
    if start_height < 0 {
        return Err(anyhow::anyhow!("start_height must be non-negative"));
    }
    if protocol_version < 70001 && relay {
        println!("Warning: relay field included but protocol_version {} < 70001", protocol_version);
    }
    let target_addr = target_node
        .to_socket_addrs()
        .context("Failed to parse target_node")?
        .next()
        .context("No valid addresses for target_node")?;
    let version = protocol_version;
    let services = NODE_NETWORK | NODE_WITNESS | NODE_NETWORK_LIMITED; // 0x409
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("Failed to get system time")?
        .as_secs() as i64;
    let mut rnd = rand::rng();
    let nonce: u64 = rnd.random();
    let user_agent = b"/Satoshi:25.0.0/"; // Match Bitcoin Core v25.0
    let addr_recv = serialize_network_address(target_addr, services)?;
    let addr_from = serialize_network_address("0.0.0.0:0".parse()?, services)?;
    let mut payload = Vec::new();
    payload.write_i32::<BigEndian>(version)?;
    payload.write_u64::<BigEndian>(services)?;
    payload.write_i64::<BigEndian>(timestamp)?;
    payload.extend_from_slice(&addr_recv);
    payload.extend_from_slice(&addr_from);
    payload.write_u64::<BigEndian>(nonce)?;
    payload.write_u8(user_agent.len() as u8)?;
    payload.extend_from_slice(user_agent);
    payload.write_i32::<BigEndian>(start_height)?;
    if protocol_version >= 70001 {
        payload.write_u8(relay as u8)?;
    }
    let checksum = calculate_checksum(&payload);
    Ok(Message {
        magic: TESTNET_MAGIC,
        command: "version".to_string(),
        payload,
        checksum,
    })
}
