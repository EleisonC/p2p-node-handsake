# Bitcoin Testnet3 P2P Node Handshake

## Overview
This project enables a handshake with Bitcoin Testnet3 nodes to establish a connection for testing purposes. It provides a Rust-based implementation that can be run natively with Cargo or inside a Docker container.

## Requirements
- **Rust**: Install via [rustup](https://rustup.rs/) (required for non-Docker usage).
- **Docker**: Install [Docker](https://docs.docker.com/get-docker/) (required for Docker usage).
- **Internet Connection**: Necessary for communicating with Testnet3 nodes.
- **Operating System**: Linux or a compatible OS.
- **Testnet3 Nodes**: Access to public Bitcoin Testnet3 nodes, such as:
   - `testnet-seed.bitcoin.jonasschnelli.ch:18333`
   - `seed.testnet.bitcoin.sprovoost.nl:18333`

## Pre-Setup
1. **Default Testnet3 Nodes**:
   - `testnet-seed.bitcoin.jonasschnelli.ch:18333`
   - `seed.testnet.bitcoin.sprovoost.nl:18333`

2. **Alternative Nodes**:
   - If default nodes are unreachable, use any publicly available Bitcoin Testnet3 node.

## Setup
1. Clone the repository:
   ```bash
   git clone https://github.com/EleisonC/p2p-node-handsake.git
   ```
2. Navigate to the project directory:
   ```bash
   cd p2p-node-handsake
   ```

## Running Options

### Option 1: Using Cargo
Run the program with:
```bash
cargo run
```
This command builds the project (on the first run) and performs the handshake with the configured Testnet3 nodes.

### Option 2: Using Docker
1. Build the Docker image:
   ```bash
   docker build -t p2p-node-handshake .
   ```
2. Run the Docker container:
   ```bash
   docker run --rm p2p-node-handshake
   ```
This executes the handshake process inside a Docker container, connecting to the default Testnet3 nodes. The `--rm` flag ensures the container is removed after execution.

**Note**: Ensure Docker is running and you have an active internet connection. The Docker image includes all dependencies, so Rust does not need to be installed locally.

## Verification
The program attempts to perform a handshake with Bitcoin Testnet3 nodes. Check the output for:
- **Success**:
  ```
  INFO: Handshake successful with testnet-seed.bitcoin.jonasschnelli.ch:18333
  ```
- **Failure**:
  ```
  ERROR: Failed to handshake with seed.testnet.bitcoin.sprovoost.nl:18333 after 3 attempts
  ```

Ensure the listed peers (e.g., `testnet-seed.bitcoin.jonasschnelli.ch:18333`) are reachable. If issues persist, try alternative Testnet3 nodes.