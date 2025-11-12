```
Contract ID: CACBGNEA2CQ5AM6HG2LSNU2Y7PBPHU4PUOUGOJUYDQXSFDHUVZGSL7VU
```

**Link Explorer:**  
[https://stellar.expert/explorer/testnet/contract/CACBGNEA2CQ5AM6HG2LSNU2Y7PBPHU4PUOUGOJUYDQXSFDHUVZGSL7VU](https://stellar.expert/explorer/testnet/contract/CACBGNEA2CQ5AM6HG2LSNU2Y7PBPHU4PUOUGOJUYDQXSFDHUVZGSL7VU)

---

# **README.md HOÀN CHỈNH – DÁN NGAY VÀO REPO**

```markdown
# Decentralized Digital Identity Management on Stellar (Soroban)

[![Stellar](https://img.shields.io/badge/Stellar-Soroban-blue?style=flat&logo=stellar)](https://soroban.stellar.org)
[![Rust](https://img.shields.io/badge/Rust-1.84+-orange?style=flat&logo=rust)](https://rust-lang.org)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat)](LICENSE)
[![Testnet](https://img.shields.io/badge/Network-Testnet-yellow)](https://stellar.expert/explorer/testnet)

A **self-sovereign digital identity system** built on **Stellar Soroban** using **Rust**. Enables secure registration, ownership transfer, and verifiable credential issuance — all on-chain, without intermediaries.

> **Use Case**: Universities, governments, or enterprises issue **verifiable IDs** (student ID, citizen ID, employee badge) that users **own and control**.

---

## Features

| Function | Description |
|--------|-------------|
| `register_identity` | Register a unique digital identity |
| `get_identity` | Retrieve full identity details |
| `transfer_identity` | Transfer ownership securely |
| `issue_credential` | Attach diploma, license, certificate |
| `verify_credential` | Check validity & issuer |
| `revoke_credential` | Revoke issued credentials |

---

## Prerequisites

| Tool | Version | Install |
|------|--------|--------|
| **Rust** | `>= 1.84.0` | `rustup update` |
| **Wasm Target** | `wasm32v1-none` | `rustup target add wasm32v1-none` |
| **Stellar CLI** | `23.1.4` | `cargo install --locked stellar-cli --features soroban` |

```powershell
# Verify
rustc --version
stellar --version
```

---

## Project Structure

```
.
├── contracts/
│   └── digital_identity/
│       ├── src/lib.rs      ← Core contract logic
│       └── Cargo.toml
├── target/                     ← Build output (auto-generated)
├── Cargo.toml                  ← Workspace
└── README.md                   ← This file
```

---

## Build the Contract

```powershell
cd contracts/digital_identity

# Clean & build
Remove-Item -Recurse -Force target -ErrorAction SilentlyContinue
stellar contract build
```

**Output:**
```
Artifact: target/wasm32v1-none/release/digital_identity.wasm
```

> **File size**: ~12-15 KB (optimized with `wasm32v1-none`)

---

## Deploy to Stellar Testnet

### 1. Create & Fund Account

```powershell
# Generate keypair
stellar keys generate issuer

# Get public address
stellar keys address issuer
# Output: GDCK4FOHRQVVE2SSAYB44KYENP4UBRGEEF3HKJ6TAO6AJCMQZNTIGLVZ
```

**Fund at:**  
[https://laboratory.stellar.org/#account-creator?network=test](https://laboratory.stellar.org/#account-creator?network=test)

**Verify Balance:**  
[https://stellar.expert/explorer/testnet/account/GDCK4FOHRQVVE2SSAYB44KYENP4UBRGEEF3HKJ6TAO6AJCMQZNTIGLVZ](https://stellar.expert/explorer/testnet/account/GDCK4FOHRQVVE2SSAYB44KYENP4UBRGEEF3HKJ6TAO6AJCMQZNTIGLVZ)

---

### 2. Deploy Contract

```powershell
cd D:\theanh\Documents\Smart-Contract-Digital-Identities

stellar contract deploy `
  --wasm target/wasm32v1-none/release/digital_identity.wasm `
  --alias digital-id `
  --network testnet `
  --source-account issuer
```

**Success Output:**
```
Deployed!
CACBGNEA2CQ5AM6HG2LSNU2Y7PBPHU4PUOUGOJUYDQXSFDHUVZGSL7VU
```

---

### 3. Get Contract ID

```powershell
CONTRACT_ID=$(stellar contract id digital-id --network testnet)
echo "Contract ID: $CONTRACT_ID"
```

---

## Deployed Contract (Testnet)

| Network | Contract ID | Explorer |
|--------|-------------|----------|
| Testnet | `CACBGNEA2CQ5AM6HG2LSNU2Y7PBPHU4PUOUGOJUYDQXSFDHUVZGSL7VU` | [View on Stellar Expert](https://stellar.expert/explorer/testnet/contract/CACBGNEA2CQ5AM6HG2LSNU2Y7PBPHU4PUOUGOJUYDQXSFDHUVZGSL7VU) |

> **Issuer Account:** [GDCK...LVZ](https://stellar.expert/explorer/testnet/account/GDCK4FOHRQVVE2SSAYB44KYENP4UBRGEEF3HKJ6TAO6AJCMQZNTIGLVZ)

---

## Usage Guide (Invoke Functions)

> All `BytesN<32>` → use hex: `0x...` (64 chars)  
> `Address` → Stellar account: `G...`  
> `Symbol` → short string: `"STUDENT"`, `"DIPLOMA"`



---

## Team Members

| Name | Student ID (MSV) | Role |
|------|------------------|------|
| [Trần Thế Anh] | [22010240] | Smart Contract Developer |
| [Bùi Trọng Hiếu] | [22010187] | Testing & Documentation |


---

## Troubleshooting

| Issue | Solution |
|------|----------|
| `Account not found` | Fund account at [Laboratory](https://laboratory.stellar.org/#account-creator?network=test) |
| `File not found` | Run `stellar contract build` in `contracts/digital_identity` |
| `symbol too long` | Use `symbol_short!("reg")` (max 9 chars) |
| `use of moved value` | Use `.clone()` for `BytesN<32>` |

---

## License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file.

---

**Built with Rust + Soroban + Stellar Testnet**
```

---
