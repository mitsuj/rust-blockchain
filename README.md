# Rust Blockchain Implementation

A simple blockchain implementation written in Rust, demonstrating core blockchain concepts and Rust's systems programming capabilities.

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## Installation

```bash
git clone https://github.com/yourusername/rust-blockchain.git
cd rust-blockchain
cargo build
```

## Usage

```bash
cargo run
```

## Project Structure

```
rust-blockchain/
├── src/
│   ├── main.rs        # Application entry point
│   ├── block.rs       # Block implementation
│   ├── blockchain.rs  # Blockchain logic
│   └── utils.rs       # Utility functions
├── Cargo.toml         # Project dependencies
└── README.md         # This file
```

## Features

- Basic blockchain structure
- Proof of Work consensus
- SHA-256 hashing
- Block validation
- Chain validity verification

## Details

- **Core Data Structures:**

  - Designed and implemented the `Block` struct, which includes fields for block index, timestamp, data, previous block hash, a `nonce`, and the block's hash.

- **Hashing and Cryptography:**

  - Implemented the SHA-256 hashing algorithm using the `sha2` crate to ensure block integrity.
  - Understood the fundamentals of cryptographic hashing and its role in blockchain technology.
  - Implemented a proof-of-work mechanism to make the block creation process computationally intensive.

- **Blockchain Logic:**

  - Developed core blockchain functionalities, such as block creation, linking, and validation, using the `blockchain.rs` module.
  - Implemented mechanisms to create the genesis block and append subsequent blocks, and to validate that the chain is correct and has not been tampered with.
  - Created functions for calculating the hash of a block, and validating a complete chain.

- **Mining:**

  - Implemented a basic mining algorithm, to generate new blocks according to a difficulty target.

- **Modularity and Separation of Concerns:**

  - Designed the project using a modular approach, separating concerns into `block.rs`, `blockchain.rs`, and `mining.rs`.
  - Used Rust's module system to create clear interfaces and separation of concerns.

- **Logging:**

  - Implemented a basic logging infrastructure using the `log` and `env_logger` crates to track the state of the blockchain and facilitate debugging.
  - Implemented proper logging using the log crate.

**Future Goals:**

- Implementing transaction management.
- Building a peer-to-peer networking layer.
- Adding transaction signing.
- Improving security features.
- Implementing persistence for the chain.
- Adding a CLI for the chain.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
