# About
Personally developed rust architecture for HITsz Cryptography Lab.

Will be continuously updating with the progress of the lab course.

## Feature

1. A uniform command line user interface for Cryptography algorithm demo.
    - See `src/demo.rs` and `src/cypher.rs`.

2. Currently support these algorithms:
    - AES
    - RSA

## Files

`../HITszCryptoLab-Rust 💡 tree .`

```
.
├── Cargo.toml
├── README.md
└── src
    ├── aes
    │   ├── block.rs
    │   ├── consts.rs
    │   ├── process.rs
    │   ├── round_key.rs
    │   └── row.rs
    ├── aes.rs
    ├── console
    │   ├── err.rs
    │   ├── io.rs
    │   └── mod.rs
    ├── cypher.rs
    ├── demo.rs
    ├── main.rs
    ├── rsa
    │   ├── algorithm.rs
    │   └── key.rs
    └── rsa.rs

```