use std::io::Write;
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use std::net::TcpStream;
use anyhow::Result;
use super::*;

pub fn send_message(stream: &mut TcpStream, msg: &Message) -> Result<()> {
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