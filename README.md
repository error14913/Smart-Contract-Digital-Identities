git ```markdown
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
# Generate keypair (positional argument in CLI 23.x)
stellar keys generate issuer

# Get public address
stellar keys address issuer
```

→ Copy address → Fund at:  
[https://laboratory.stellar.org/#account-creator?network=test](https://laboratory.stellar.org/#account-creator?network=test)

```powershell
# Verify balance
stellar account show issuer --network testnet
```

---

### 2. Deploy Contract

```powershell
cd D:\theanh\Documents\Smart-Contract-Digital-Identities

stellar contract deploy `
  --wasm target/wasm32v1-none/release/digital_identity.wasm `
  --alias digital-id `
  --network testnet `
  issuer
```

**Success Output:**
```
Contract deployed successfully!
Contract ID: CBE...XYZ
```

---

### 3. Get Contract ID

```powershell
$ID = stellar contract id digital-id --network testnet
echo "Contract ID: $ID"
```

---

## Usage Guide (Invoke Functions)

> All `BytesN<32>` → use hex: `0x...` (64 chars)  
> `Address` → Stellar account: `G...`  
> `Symbol` → short string: `"STUDENT"`, `"DIPLOMA"`

### 1. Register Identity

```powershell
ISSUER=$(stellar keys address issuer)
ID=0x1111111111111111111111111111111111111111111111111111111111111111

stellar contract invoke `
  --id $ID `
  --fn register_identity `
  --arg identity_id="0x1111111111111111111111111111111111111111111111111111111111111111" `
  --arg owner="$ISSUER" `
  --arg identity_type="STUDENT" `
  --arg metadata="Nguyen Van A | Hanoi University" `
  --network testnet `
  issuer
```

### 2. Get Identity

```powershell
stellar contract invoke `
  --id $ID `
  --fn get_identity `
  --arg identity_id="0x1111111111111111111111111111111111111111111111111111111111111111" `
  --network testnet
```

### 3. Issue Credential

```powershell
CRED_ID=0xaaaaaaaabbbbbbbbccccccccddddddddeeeeeeeeffffffffaaaaaaaabbbbbbbb

stellar contract invoke `
  --id $ID `
  --fn issue_credential `
  --arg credential_id="$CRED_ID" `
  --arg identity_id="0x1111111111111111111111111111111111111111111111111111111111111111" `
  --arg issuer="$ISSUER" `
  --arg credential_type="DIPLOMA" `
  --arg issue_date=1734028800 `
  --arg expiry_date=1891708800 `
  --network testnet `
  issuer
```

### 4. Verify Credential

```powershell
stellar contract invoke `
  --id $ID `
  --fn verify_credential `
  --arg credential_id="$CRED_ID" `
  --network testnet
```

---

## Deployed Contract (Testnet)

| Network | Contract ID | Explorer |
|--------|-------------|----------|
| Testnet | `CBE...XYZ` | [View on Stellar Expert](https://stellar.expert/explorer/testnet/contract/CBE...XYZ) |

> *(Replace `CBE...XYZ` with your actual deployed ID)*

---

## Team Members

| Name | Student ID (MSV) | Role |
|------|------------------|------|
| [Your Full Name] | [Your MSV] | Smart Contract Developer |
| [Member 2] | [MSV] | Testing & Documentation |
| [Member 3] | [MSV] | Deployment & Integration |

---

## Troubleshooting

| Issue | Solution |
|------|----------|
| `Failed to find config identity` | Run `stellar keys generate issuer` |
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

