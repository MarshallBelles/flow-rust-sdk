# Welcome to the Flow-Rust-SDK!

We're glad to have you here.
There are a few important items that we should cover real quick before you dive in.

## Signing Algorithms

- Only `ECDSA_P256` is supported at this time

## Hashing

- Only `SHA3_256` is supported at this time

## Security

- The cryptography in this SDK is sourced from the public [RustCrypto](https://github.com/RustCrypto) repositories. This is a very mature and widely used library, but the elliptic curve arithmetic contained in these crates has never been independently audited. *Use at your own risk.*
- Remember that you will be dealing with private keys, which can be more powerful and dangerous than passwords. Please treat them as such.
- Consider reading [this whitepaper by Google](https://cloud.google.com/solutions/modern-password-security-for-system-designers.pdf).

## Documentation

See the [docs.rs](https://docs.rs/flow-rust-sdk/latest/flow_rust_sdk/) for full documentation.
Please open an issue in the [GitHub repository](https://github.com/MarshallBelles/flow-rust-sdk) if you find any bugs.
For general questions, please join the [Flow Discord](https://discord.com/invite/flow). There is a flow-rust channel which is an excellent place for discussion!

