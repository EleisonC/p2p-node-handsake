use std::net::{ToSocketAddrs};
use tokio::net::TcpStream;
use std::time::{Duration, SystemTime};
use anyhow::{Context, Result};
use tokio::io::AsyncWriteExt;
use super::*;
pub async fn try_handshake(target: &str) -> Result<()> {
    let mut stream = TcpStream::connect(
        &target.to_socket_addrs()?.next().context("Invalid address")?
    ).await?;

    let version_msg = create_version_message(target, START_HEIGHT, PROTOCOL_VERSION, false)?;
    
    let buff = send_message(&version_msg)?;
    stream.write_all(&buff).await?;
    tracing::info!("Sent version message to {}", target);
    let mut received_version = false;
    let mut received_verack = false;
    let start_time = SystemTime::now();
    let timeout = Duration::from_secs(5); // Total handshake timeout
    while !received_version || !received_verack {
        if SystemTime::now()
            .duration_since(start_time)
            .unwrap_or(Duration::from_secs(0))
            > timeout
        {
            anyhow::bail!("Handshake timed out after {:?}", timeout);
        }
        let msg = receive_message(&mut stream).await?;
        tracing::warn!("Received {} from {}", msg.command, target);
        match msg.command.as_str() {
            "version" => {
                if !received_version {
                    received_version = true;
                    let verack_msg = create_verack_message();
                    let buff = send_message(&verack_msg)?;
                    stream.write_all(&buff).await?;
                    tracing::trace!("Sent verack to {}", target);
                }
            }
            "verack" => {
                received_verack = true;
            }
            "wtxidrelay" => {
                let wtxidrelay_msg = create_wtxidrelay_message();
                let buff = send_message(&wtxidrelay_msg)?;
                stream.write_all(&buff).await?;
                tracing::info!("Sent wtxidrelay to {}", target);
            }
            _ => {
                tracing::info!("Ignored message: {}", msg.command);
            }
        }
        if received_version && received_verack {
            tracing::info!("Handshake successful with {}", target);
            return Ok(());
        }
    }
    anyhow::bail!("Handshake incomplete with {}", target);
}