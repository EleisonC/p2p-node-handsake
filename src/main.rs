use anyhow::{Context, Result};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use rand::Rng;
use sha2::{Digest, Sha256};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const TESTNET_MAGIC: u32 = 0x0B110907;
const PROTOCOL_VERSION: i32 = 70228; // Bitcoin Core v25.0
const START_HEIGHT: i32 = 2_600_000;

// Service identifiers
const NODE_NETWORK: u64 = 0x01;
const NODE_WITNESS: u64 = 0x08;
const NODE_NETWORK_LIMITED: u64 = 0x400;

#[derive(Debug)]
struct Message {
    magic: u32,
    command: String,
    payload: Vec<u8>,
    checksum: [u8; 4],
}

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
    let nonce = rand::thread_rng().gen::<u64>();
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

fn serialize_network_address(addr: SocketAddr, services: u64) -> Result<Vec<u8>> {
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

fn create_verack_message() -> Message {
    let payload = Vec::new();
    let checksum = calculate_checksum(&payload);
    Message {
        magic: TESTNET_MAGIC,
        command: "verack".to_string(),
        payload,
        checksum,
    }
}

fn create_wtxidrelay_message() -> Message {
    let payload = Vec::new();
    let checksum = calculate_checksum(&payload);
    Message {
        magic: TESTNET_MAGIC,
        command: "wtxidrelay".to_string(),
        payload,
        checksum,
    }
}

fn calculate_checksum(payload: &[u8]) -> [u8; 4] {
    let mut hasher = Sha256::new();
    hasher.update(payload);
    let hash1 = hasher.finalize();
    let mut hasher = Sha256::new();
    hasher.update(hash1);
    let hash2 = hasher.finalize();
    let mut checksum = [0u8; 4];
    checksum.copy_from_slice(&hash2[..4]);
    checksum
}

fn send_message(stream: &mut TcpStream, msg: &Message) -> Result<()> {
    let mut buffer = Vec::new();
    buffer.write_u32::<BigEndian>(msg.magic)?;
    let mut command = msg.command.as_bytes().to_vec();
    command.resize(12, 0);
    buffer.write_all(&command)?;
    buffer.write_u32::<LittleEndian>(msg.payload.len() as u32)?;
    buffer.write_all(&msg.checksum)?;
    buffer.write_all(&msg.payload)?;
    stream.write_all(&buffer)?;
    stream.flush()?;
    Ok(())
}

fn receive_message(stream: &mut TcpStream) -> Result<Message> {
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

fn try_handshake(target: &str) -> Result<()> {
    let mut stream = TcpStream::connect_timeout(
        &target.to_socket_addrs()?.next().context("Invalid address")?,
        Duration::from_secs(10),
    )?;
    stream.set_read_timeout(Some(Duration::from_secs(15)))?;
    let version_msg = create_version_message(target, START_HEIGHT, PROTOCOL_VERSION, false)?;
    send_message(&mut stream, &version_msg)?;
    println!("Sent version message to {}", target);
    let mut received_version = false;
    let mut received_verack = false;
    let start_time = SystemTime::now();
    let timeout = Duration::from_secs(30); // Total handshake timeout
    while !received_version || !received_verack {
        if SystemTime::now()
            .duration_since(start_time)
            .unwrap_or(Duration::from_secs(0))
            > timeout
        {
            anyhow::bail!("Handshake timed out after {:?}", timeout);
        }
        let msg = receive_message(&mut stream)?;
        println!("Received {} from {}", msg.command, target);
        match msg.command.as_str() {
            "version" => {
                if !received_version {
                    received_version = true;
                    let verack_msg = create_verack_message();
                    send_message(&mut stream, &verack_msg)?;
                    println!("Sent verack to {}", target);
                }
            }
            "verack" => {
                received_verack = true;
            }
            "wtxidrelay" => {
                let wtxidrelay_msg = create_wtxidrelay_message();
                send_message(&mut stream, &wtxidrelay_msg)?;
                println!("Sent wtxidrelay to {}", target);
            }
            _ => {
                println!("Ignored message: {}", msg.command);
            }
        }
        if received_version && received_verack {
            println!("Handshake successful with {}", target);
            return Ok(());
        }
    }
    anyhow::bail!("Handshake incomplete with {}", target);
}

fn main() -> Result<()> {
    let peers = [
        "testnet-seed.bitcoin.jonasschnelli.ch:18333",
        "seed.testnet.bitcoin.sprovoost.nl:18333",
        "testnet-seed.bluematt.me:18333",
        "testnet3-node.bitcoin.petertodd.org:18333", // Fallback node
    ];
    let max_handshake_attempts = 3;
    for peer in peers.iter() {
        println!("Attempting handshake with peer: {}", peer);
        let mut attempts = 0;
        while attempts < max_handshake_attempts {
            match try_handshake(peer) {
                Ok(()) => break,
                Err(e) => {
                    attempts += 1;
                    let err_msg = match e.downcast_ref::<std::io::Error>() {
                        Some(io_err) => match io_err.kind() {
                            std::io::ErrorKind::ConnectionRefused => "Connection refused".to_string(),
                            std::io::ErrorKind::ConnectionReset => "Connection reset by peer".to_string(),
                            std::io::ErrorKind::TimedOut => "Connection timed out".to_string(),
                            std::io::ErrorKind::NetworkUnreachable => "Network is unreachable".to_string(),
                            _ => format!("IO error: {}", io_err),
                        },
                        None => e.to_string(),
                    };
                    println!("Attempt {} failed: {}. Retrying...", attempts, err_msg);
                    if attempts == max_handshake_attempts {
                        println!("Failed to handshake with {} after {} attempts", peer, attempts);
                    }
                    std::thread::sleep(Duration::from_secs(2));
                }
            }
        }
    }
    Ok(())
}