use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, SystemTime};
use anyhow::{Context, Result};
use super::*;
pub fn try_handshake(target: &str) -> Result<()> {
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