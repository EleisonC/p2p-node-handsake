use std::collections::HashSet;
use std::io::{ErrorKind, Error};
use anyhow::Result;

pub mod domain;
pub mod utils;

use std::time::Duration;
use domain::*;
use utils::*;


pub const TESTNET_MAGIC: u32 = 0x0B110907;
pub const PROTOCOL_VERSION: i32 = 70228; // Bitcoin Core v25.0
pub const START_HEIGHT: i32 = 2_600_000;

// Service identifiers
pub const NODE_NETWORK: u64 = 0x01;
pub const NODE_WITNESS: u64 = 0x08;
pub const NODE_NETWORK_LIMITED: u64 = 0x400;

#[derive(Debug)]
pub struct Message {
    pub magic: u32, 
    pub command: String,
    pub payload: Vec<u8>,
    pub checksum: [u8; 4],
}

#[derive(Debug)]
pub struct HandTool {
    pub node_list: HashSet<String>,
    pub max_handshake_attempts: i8
}

impl Default for HandTool {
    fn default() -> Self {
        Self {
            node_list: HashSet::from([
                String::from("testnet-seed.bitcoin.jonasschnelli.ch:18333"), 
                String::from("seed.testnet.bitcoin.sprovoost.nl:18333")
            ]),
            max_handshake_attempts: 2
        }
    }
} 

impl HandTool {
    pub fn new() -> Self {
        Self {
            node_list: HashSet::new(),
            max_handshake_attempts: 2
        }
    }
    
    pub fn add_node(&mut self, node: &String) {
        self.node_list.insert(node.to_string());
    }
    
    pub fn remove_node(&mut self, node: String) {
        self.node_list.remove(&node);  
    }
    
    pub fn get_nodes(&self) -> &HashSet<String> {
        &self.node_list
    }
    
    pub fn get_max_handshake_attempts(&self) -> i8 {
        self.max_handshake_attempts
    }
    
    pub fn set_max_handshake_attempts(&mut self, max_handshake_attempts: i8) {
        if max_handshake_attempts < 1 {
            return;
        }
        self.max_handshake_attempts = max_handshake_attempts;
    }
    
    pub fn perform_handshake(&self) -> Result<()> {
        let peers = self.get_nodes();
        let max_handshake_attempts = self.get_max_handshake_attempts();

        for peer in peers.iter() {
            println!("Attempting handshake with peer: {}", peer);
            let mut attempts = 0;
            while attempts < max_handshake_attempts {
                match try_handshake(peer) {
                    Ok(()) => break,
                    Err(e) => {
                        attempts += 1;
                        let err_msg = match e.downcast_ref::<Error>() {
                            Some(io_err) => match io_err.kind() {
                                ErrorKind::ConnectionRefused => "Connection refused".to_string(),
                                ErrorKind::ConnectionReset => "Connection reset by peer".to_string(),
                                ErrorKind::TimedOut => "Connection timed out".to_string(),
                                ErrorKind::NetworkUnreachable => "Network is unreachable".to_string(),
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
    
}