# Welcome to the Flow-Rust-SDK!

basic usage: in your `Cargo.toml`
```rs
flow-rust-sdk = "1.0.0"
```

Welcome to the Flow-Rust-SDK! We're glad to have you here. 
There are a few important items that we should cover real quick before you dive in.

## Signing Algorithms

-- Only `ECDSA_P256` is supported at this time

## Hashing

-- Only `SHA3_256` is supported at this time

## Documentation

Documentation is currently a work in progress. Please open an issue in the [GitHub repository](https://github.com/MarshallBelles/flow-rust-sdk) if you find any bugs.
For general questions, please join the [Flow Discord](https://discord.com/invite/flow). There is a flow-rust channel which is an excellent place for discussion!

```
// basic usage

let service_account = std::env::vars().filter(|kv| kv.0 == "SERVICE_ACCT").map(|kv| kv.1).collect::<Vec<String>>();
let private_key = std::env::vars().filter(|kv| kv.0 == "PRIV_K").map(|kv| kv.1).collect::<Vec<String>>();
let public_key = std::env::vars().filter(|kv| kv.0 == "PUB_K").map(|kv| kv.1).collect::<Vec<String>>();

// create the account

let network_address = "https://access.devnet.nodes.onflow.org:9000".to_string();
let payer = &service_account[0];
let payer_private_key = &private_key[0];
let public_keys = vec![public_key[0].to_owned()];

let acct = create_account(
    &network_address,
    public_keys.to_vec(),
    &payer,
    &payer_private_key,
    0,
)
.await
.expect("Could not create account");
println!("{:?}", acct);
```
