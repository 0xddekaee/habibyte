# Habibyte Core Blockchain

**Habibyte** is a modular Rust blockchain designed for national identity management. It focuses on privacy (Zero Data Leaks), unique identity verification (Zero Duplicates), and secure off-chain data storage.

## 🚀 Getting Started

### Prerequisites

You need the following installed on your system:

1.  **Rust Toolchain**: [Install Rust](https://rustup.rs/)
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
2.  **Build Essentials** (Required for compiling dependencies):
    ```bash
    sudo apt update
    sudo apt install build-essential -y
    ```
    *(Note: If you are on WSL, make sure your package list is updated)*

### 🛠️ Building the Project

Navigate to the `core` directory and build the entire workspace:

```bash
cd core
cargo build
```

### ▶️ Running the Node

To start the main blockchain node entry point:

```bash
cargo run -p habibyte-node
```

You should see logs indicating the Node and Ledger have started.

---

## 📂 Project Structure

The project is organized as a Cargo Workspace with modular crates:

- **`habibyte-node`**: The main executable. It initializes the system, P2P networking, and API.
- **`habibyte-p2p`**: **[New]** Dedicated networking library using `libp2p`. Handles Gossipsub and mDNS interactions cleanly.
- **`habibyte-identity`**: Handles NIK hashing (privacy) and `Identity` logic. Ensures "Zero Duplicate" identities.
- **`habibyte-ledger`**: Defines the Blockchain `Block`, `Transaction`, and in-memory `Ledger`.
- **`habibyte-storage`**: Handles AES-GCM encryption and interfaces for Off-Chain storage (IPFS/Local).
- **`habibyte-api`**: Provides the HTTP/JSON-RPC interface for external apps (e.g., website, mobile app).
- **`habibyte-consensus`**: Logic for block validation (Proof of Authority).

## 🔐 Key Features

- **Privacy First**: Sensitive NIKs are hashed (`SHA-256`) before being stored on-chain.
- **Off-Chain Data**: Real citizen data is encrypted using `AES-GCM` and stored off-chain (IPFS ready), preserving privacy while maintaining data integrity.
- **Modular Design**: Components are decoupled, allowing easy upgrades to consensus or storage mechanisms.

## 🗺️ Roadmap

See [roadmap.txt](./roadmap.txt) for the detailed development plan.
