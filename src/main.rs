use std::io::{Read, Write};
use std::net::TcpStream;
use anyhow::Result;
use byteorder::{LittleEndian, WriteBytesExt};
use std::time::{SystemTime, UNIX_EPOCH};


fn main()-> Result<()> {
    //Step 0: Establish a TCP connection to a node
    let target_node = "216.219.91.82:18333";
    let mut stream  = TcpStream::connect(target_node)?;

    // Step 1: Send a version message
    let version_msg = create_version_message()?;
    stream.write_all(&version_msg)?;
    println!("Sent version message");

    Ok(())
}

fn create_version_message() -> Result<Vec<u8>> {
    let mut msg = Vec::new();

    //add magic bytes (testnet3)
    msg.extend_from_slice(&[0x0B, 0x11, 0x09, 0x07]);

    // add version number
    msg.extend_from_slice(b"version\0\0\0\0\0");

    // Payload length
    let payload_start = msg.len();
    msg.write_u32::<LittleEndian>(0)?;

    msg.write_u32::<LittleEndian>(0)?;

    let mut payload= Vec::new();
    // Protocol version
    payload.write_i32::<LittleEndian>(0)?;
    // services
    payload.write_i32::<LittleEndian>(0)?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)?.as_secs();
    payload.write_i64::<LittleEndian>(timestamp as i64)?;
    payload.extend_from_slice(&[0; 26]);
    payload.extend_from_slice(&[0; 26]);
    payload.write_u64::<LittleEndian>(rand::random())?; // Nonce
    payload.push(0); // User agent (empty)
    payload.write_i32::<LittleEndian>(0)?; // Start height

    let length = payload.len() as u32;
    msg[payload_start..payload_start + 4].copy_from_slice(&length.to_le_bytes());
    let checksum = calculate_checksum(&payload);
    msg[payload_start + 4..payload_start + 8].copy_from_slice(&checksum);
    msg.extend(payload);
    Ok(msg)
}