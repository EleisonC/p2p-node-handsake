# Bitcoin Testnet Handshake

## Overview
This project facilitates a handshake with Bitcoin Testnet3 nodes to establish a connection for testing purposes.

## Requirements
- **Rust**: Install via [rustup](https://rustup.rs/).
- **Internet Connection**: Required for node communication.
- **Operating System**: Linux or a compatible OS.
- **Testnet3 Nodes**: Access to public Bitcoin Testnet3 nodes (e.g., `seed.testnet.bitcoin.sprovoost.nl:18333`).

## Pre-Setup
1. **Default Testnet3 Nodes**:
   - `testnet-seed.bitcoin.jonasschnelli.ch:18333`
   - `seed.testnet.bitcoin.sprovoost.nl:18333`

2. **Alternative Nodes**:
   - If the default nodes are unreachable, you can use any publicly available Bitcoin node running on Testnet3.


## Setup
1. Clone the repository:
   ```bash
   git clone https://github.com/EleisonC/p2p-node-handsake.git
   ```
2. Navigate to the project directory:
   ```bash
   cd p2p-node-handsake
   ```
4. Run the program:
   ```bash
   cargo run
   ```


## Verification
- The program attempts to perform a handshake with Bitcoin testnet nodes.
- **Success**: `Handshake successful with <peer>`,
- **Failure**: `Connection refused` or `Timed out` indicate issues, with up to 3 retries per peer.
- Ensure the listed peers (e.g., `testnet-seed.bitcoin.jonasschnelli.ch:18333`) are reachable.