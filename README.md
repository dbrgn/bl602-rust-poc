# Rust + RISCV + BL602 Demo

This demo runs Rust firmware on the BL602.

Tested with the BL602 Evalboard by Sipeed (pre-production units).

## Setup

    rustup target add riscv32imac-unknown-none-elf
    cargo install cargo-embed --features ftdi

## Links

- [Sipeed BL602 Rust Guide](https://github.com/sipeed/bl602-rust-guide)
- [Lupyuen's PineCone BL602 + Rust Docs](https://lupyuen.github.io/pinecone-rust/)
- [pine64 BL602 Docs](https://github.com/pine64/bl602-docs)
