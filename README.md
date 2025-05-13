# Bitcoin Testnet3 P2P Node Handshake

## Overview
This project implements a Rust-based tool to perform asynchronous handshakes with Bitcoin Testnet3 nodes using the Tokio runtime. It establishes connections for testing purposes, allowing users to connect to predefined or custom Testnet3 nodes.

## Requirements
- **Rust**: Install via [rustup](https://rustup.rs/) (required for running the project).
- **Internet Connection**: Necessary for communicating with Testnet3 nodes.
- **Operating System**: Linux, macOS, or any OS compatible with Rust and Tokio.
- **Testnet3 Nodes**: Access to public Bitcoin Testnet3 nodes, such as:
    - `testnet-seed.bitcoin.jonasschnelli.ch:18333`
    - `seed.testnet.bitcoin.sprovoost.nl:18333`
- **Optional - Docker**: Install [Docker](https://docs.docker.com/get-docker/) for containerized execution.

## Pre-Setup
1. **Default Testnet3 Nodes**:
    - `testnet-seed.bitcoin.jonasschnelli.ch:18333`
    - `seed.testnet.bitcoin.sprovoost.nl:18333`

2. **Alternative Nodes**:
    - If default nodes are unreachable, you can add custom Bitcoin Testnet3 nodes (see "Adding a New Node").

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
1. Run the program with:
    ```bash
    cargo run
    ```
This command builds the project (on the first run) and performs handshakes with the configured Testnet3 nodes. The program attempts to connect to each node up to 3 times, as specified in `main.rs`.

### Option 2: Using Docker
1. Build the Docker image:
   ```bash
   docker build -t p2p-node-handshake .
   ```
2. Run the Docker container:
   ```bash
   docker run --rm p2p-node-handshake
   ```
This executes the handshake process inside a Docker container, connecting to the configured Testnet3 nodes. The `--rm` flag ensures the container is removed after execution.

**Note**: Ensure Docker is running and you have an active internet connection. The Docker image includes all dependencies, so Rust does not need to be installed locally.

## Verification
The program attempts to perform a handshake with each configured Bitcoin Testnet3 node. Check the console output for:
- **Success**:
  ```
  INFO: Attempting handshake with peer: testnet-seed.bitcoin.jonasschnelli.ch:18333
  ```
  (Followed by no error messages, indicating a successful handshake.)
- **Failure**:
  ```
  ERROR: Failed to handshake with seed.testnet.bitcoin.sprovoost.nl:18333 after 3 attempts
  ```

If a handshake fails, ensure the node is reachable and supports Bitcoin Testnet3. Try adding alternative nodes as described in "Adding a New Node."

## Adding a New Node
To connect to a custom Bitcoin Testnet3 node, modify the `main.rs` file to add the node to the `HandTool` instance:

1. Open `src/main.rs` in a text editor.
2. Modify the code to add a new node using `tool.add_node()`. For example, to add `new-node.example.com:18333`:
   ```rust
   use anyhow::Result;
   use p2p_node_handshake::{HandTool, utils};

   #[tokio::main]
   async fn main() -> Result<()> {
       utils::hand_logger::init_logger();
       let mut tool = HandTool::default();

       // Add a new node
       tool.add_node("new-node.example.com:18333".to_string());

       tool.set_max_handshake_attempts(3);
       tool.perform_handshake().await
   }
   ```
3. Save the file and run the program as described in "Running Options."

**Note**: Ensure the new node is a valid Bitcoin Testnet3 node. You can find public nodes through community resources or run your own Testnet3 node using Bitcoin Core. The project does not currently support configuration files or command-line arguments for node addition.
