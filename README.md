# Digital Identity Management on Stellar (Soroban)

[![Stellar](https://img.shields.io/badge/Stellar-Soroban-blue)](https://soroban.stellar.org)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange)](https://rust-lang.org)
[![License](https://img.shields.io/badge/License-MIT-green)](LICENSE)

A **decentralized, self-sovereign digital identity system** on Stellar using Soroban smart contracts. Users own and control their identity. Issuers (schools, governments, companies) can issue verifiable credentials.

> **Use Case**: Student IDs, Citizen IDs, Employee Badges, Professional Licenses — all on-chain, verifiable, and portable.

---

## Features

| Function | Description |
|--------|-------------|
| `register_identity` | Create a unique digital ID |
| `get_identity` | Retrieve identity details |
| `transfer_identity` | Transfer ownership |
| `issue_credential` | Issue diploma, license, etc. |
| `verify_credential` | Verify validity |
| `revoke_credential` | Revoke issued credential |

---

## Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update

# Add Wasm target
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
cargo install --locked stellar-cli --features soroban