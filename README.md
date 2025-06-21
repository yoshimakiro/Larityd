# Larity

A proof-of-work blockchain with:
- Blake3-512 mining algorithm
- DAGKnight consensus protocol
- SPHINCS+ transaction signatures
- zk-SNARKs for state compression and privacy

## Getting Started

1. Install Rust: https://rustup.rs/
2. Build the node:
   ```sh
   cargo build --release
   ```
3. Run a node:
   ```sh
   cargo run -p Larityd
   ```

## Project Structure

- `crates/consensus`: DAGKnight consensus implementation
- `crates/crypto`: Cryptographic primitives (Blake3, SPHINCS+)
- `crates/model`: Core data structures
- `crates/network`: P2P networking
- `crates/node`: Main node implementation
- `crates/prover`: zk-SNARK proof generation
- `crates/snarks`: zk-SNARK circuits
- `crates/state`: Compressed state management
