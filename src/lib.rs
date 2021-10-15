//! Welcome to the Flow-Rust-SDK!
//! We're glad to have you here.
//! There are a few important items that we should cover real quick before you dive in.
//!
//! ## Signing Algorithms
//!
//! - Only `ECDSA_P256` is supported at this time
//!
//! ## Hashing
//!
//! - Only `SHA3_256` is supported at this time
//!
//! ## Security
//!
//! - The cryptography in this SDK is sourced from the public [`RustCrypto`](https://github.com/RustCrypto) repositories. This is a very mature and widely used library, but the elliptic curve arithmetic contained in these crates has never been independently audited. *Use at your own risk.*
//! - Remember that you will be dealing with private keys, which can be more powerful and dangerous than passwords. Please treat them as such.
//! - Consider reading [`this whitepaper by Google`](https://cloud.google.com/solutions/modern-password-security-for-system-designers.pdf)
//!
//! ## Documentation
//!
//! See the [`docs.rs`](https://docs.rs/flow-rust-sdk/latest/flow_rust_sdk/) for full documentation.
//! Please open an issue in the [`GitHub repository`](https://github.com/MarshallBelles/flow-rust-sdk) if you find any bugs.
//! For general questions, please join the [`Flow Discord`](https://discord.com/invite/flow). There is a flow-rust channel which is an excellent place for discussion!
//!
//! ## Basic Usage
//!
//! In your Cargo.toml
//! ```
//! flow-rust-sdk = "*" // replace * with the highest version available
//! ```
//!
//! You may also wish to add
//! ```
//! tokio = { version = "1.11.0", features = ["full"] }
//! ```
//!
//! ```
//! use flow_rust_sdk::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // check if testnet is available
//!     check_availability(&"grpc://access.devnet.nodes.onflow.org:9000".to_string()).await?;
//!     Ok(())
//! }
//! ```
//!

// ****************************************************
// License: Apache V2.0 OR MIT, at your option
// ****************************************************

// ****************************************************
// External Dependencies
// ****************************************************
use std::error;

use flow::access_api_client::AccessApiClient;

use flow::*;

pub mod flow {
    //! `flow` is an exported module from the flow_rust_sdk.
    //! It's types are generated directly from the gRPC API Protobufs
    //! https://github.com/onflow/flow/tree/master/protobuf
    tonic::include_proto!("flow.access");
}

// for signing transactions
use bytes::Bytes;
use p256_flow::ecdsa::{signature_flow::Signature, signature_flow::Signer, SigningKey};
use p256_flow::elliptic_curve_flow::SecretKey;
pub extern crate hex;
pub extern crate rlp;
use rlp::*;

// ****************************************************
// Connection Object
// ****************************************************

/// The FlowConnection object contains a single API connection.
/// The network transport layer can be optionally substitued by implementing a new FlowConnection<T>
pub struct FlowConnection<T> {
    pub client: AccessApiClient<T>,
}

/// The default implementation of a FlowConnection, using `tonic::transport::Channel`
impl FlowConnection<tonic::transport::Channel> {
    /// Initializes a new connection and checks the availability of the node at the provided address
    pub async fn new(network_address: &str) -> Result<FlowConnection<tonic::transport::Channel>, Box<dyn error::Error>> {
        let mut client = AccessApiClient::connect(network_address.to_owned()).await?;
        let request = tonic::Request::new(PingRequest {});
        client.ping(request).await?;
        Ok(FlowConnection::<tonic::transport::Channel> {client})
    }
    /// get_account will return the `flow::AccountResponse` of `network_address`, else an error if it could not be accessed.
    pub async fn get_account(
        &mut self,
        account_address: String,
    ) -> Result<AccountResponse, Box<dyn error::Error>> {
        let request = tonic::Request::new(GetAccountAtLatestBlockRequest {
            address: hex::decode(account_address).unwrap(),
        });
        let response = self.client.get_account_at_latest_block(request).await?;
        Ok(response.into_inner())
    }
    /// execute_script will attempt to run the provided script (as bytes) and return the `flow::ExecuteScriptResponse` or Error
    pub async fn execute_script(
        &mut self,
        script: Vec<u8>,
    ) -> Result<ExecuteScriptResponse, Box<dyn error::Error>> {
        let request = tonic::Request::new(ExecuteScriptAtLatestBlockRequest { script });
        let response = self.client.execute_script_at_latest_block(request).await?;
        Ok(response.into_inner())
    }
    /// Sends the transaction to the blockchain.
    /// Make sure you signed the transactionsign_transaction first.
    pub async fn send_transaction(
        &mut self,
        transaction: Option<Transaction>,
    ) -> Result<SendTransactionResponse, Box<dyn error::Error>> {
        // send to blockchain
        let request = tonic::Request::new(SendTransactionRequest { transaction });
        let response = self.client.send_transaction(request).await?;
        Ok(response.into_inner())
    }
    /// get transaction result
    pub async fn get_transaction_result(
        &mut self,
        id: Vec<u8>,
    ) -> Result<TransactionResultResponse, Box<dyn error::Error>> {
        // send to blockchain
        let request = tonic::Request::new(GetTransactionRequest { id });
        let response = self.client.get_transaction_result(request).await?;
        Ok(response.into_inner())
    }
    /// get transaction result
    pub async fn get_transaction(
        &mut self,
        id: Vec<u8>,
    ) -> Result<TransactionResponse, Box<dyn error::Error>> {
        // send to blockchain
        let request = tonic::Request::new(GetTransactionRequest { id });
        let response = self.client.get_transaction(request).await?;
        Ok(response.into_inner())
    }
    /// get_block accepts either the block_id or block_height. If neither are defined it returns the latest block.
    pub async fn get_block(
        &mut self,
        block_id: Option<String>,
        block_height: Option<u64>,
        is_sealed: Option<bool>,
    ) -> Result<BlockResponse, Box<dyn error::Error>> {
        if block_id.is_some() {
            // IF block_id, use this
            let request = tonic::Request::new(GetBlockByIdRequest {
                id: block_id.unwrap().as_bytes().to_vec(),
            });
            let response = self.client.get_block_by_id(request).await?;
            Ok(response.into_inner())
        } else if block_height.is_some() {
            // else IF block_height, use that
            let request = tonic::Request::new(GetBlockByHeightRequest {
                height: block_height.unwrap(),
            });
            let response = self.client.get_block_by_height(request).await?;
            Ok(response.into_inner())
        } else {
            // else, just get latest block
            if is_sealed.is_some() {
                let request = tonic::Request::new(GetLatestBlockRequest {
                    is_sealed: is_sealed.unwrap(),
                });
                let response = self.client.get_latest_block(request).await?;
                Ok(response.into_inner())
            } else {
                let request = tonic::Request::new(GetLatestBlockRequest { is_sealed: false });
                let response = self.client.get_latest_block(request).await?;
                Ok(response.into_inner())
            }
        }
    }
    /// retrieve the specified events by type for the given height range
    pub async fn get_events_for_height_range(
        &mut self,
        event_type: String,
        start_height: u64,
        end_height: u64,
    ) -> Result<EventsResponse, Box<dyn error::Error>> {
        let request = tonic::Request::new(GetEventsForHeightRangeRequest {
            r#type: event_type,
            start_height,
            end_height,
        });
        let response = self.client.get_events_for_height_range(request).await?;
        Ok(response.into_inner())
    }
    /// retrieve the specified events by type for the given blocks
    pub async fn get_events_for_block_ids(
        &mut self,
        event_type: String,
        ids: Vec<Vec<u8>>,
    ) -> Result<EventsResponse, Box<dyn error::Error>> {
        let request = tonic::Request::new(GetEventsForBlockIdsRequest {
            r#type: event_type,
            block_ids: ids,
        });
        let response = self.client.get_events_for_block_i_ds(request).await?;
        Ok(response.into_inner())
    }
    /// retrieve the specified collections
    pub async fn get_collection(
        &mut self,
        collection_id: Vec<u8>,
    ) -> Result<CollectionResponse, Box<dyn error::Error>> {
        let request = tonic::Request::new(GetCollectionByIdRequest { id: collection_id });
        let response = self.client.get_collection_by_id(request).await?;
        Ok(response.into_inner())
    }
    pub async fn create_account(
        &mut self,
        account_keys: Vec<String>,
        payer: &String,
        payer_private_key: &String,
        key_id: u32,
    ) -> Result<flow::Account, Box<dyn error::Error>> {
        let create_account_template = b"
        transaction(publicKeys: [String], contracts: {String: String}) {
            prepare(signer: AuthAccount) {
                let acct = AuthAccount(payer: signer)
        
                for key in publicKeys {
                    acct.addPublicKey(key.decodeHex())
                }
        
                for contract in contracts.keys {
                    acct.contracts.add(name: contract, code: contracts[contract]!.decodeHex())
                }
            }
        }";

        let latest_block: BlockResponse =
            self.get_block(None, None, Some(false)).await?;
        let account: flow::Account = self.get_account(payer.clone())
            .await?
            .account
            .unwrap();
        let proposer = TransactionProposalKey {
            address: hex::decode(payer).unwrap(),
            key_id,
            sequence_number: account.keys[key_id as usize].sequence_number as u64,
        };
        let keys_arg = process_keys_args(account_keys);
        // empty contracts for now - will implement in the future
        let contracts_arg = Argument::dictionary(vec![]);
        let keys_arg = json!(keys_arg);
        let contracts_arg = json!(contracts_arg);
        let transaction: Transaction = build_transaction(
            create_account_template.to_vec(),
            vec![
                serde_json::to_vec(&keys_arg)?,
                serde_json::to_vec(&contracts_arg)?,
            ],
            latest_block.block.unwrap().id,
            1000,
            proposer,
            vec![payer.clone()],
            payer.clone(),
        )
        .await?;
        let signature = Sign {
            address: payer.clone(),
            key_id,
            private_key: payer_private_key.clone(),
        };
        let transaction: Option<Transaction> =
            sign_transaction(transaction, vec![], vec![&signature]).await?;
        let transaction: SendTransactionResponse =
            self.send_transaction(transaction).await?;
        // poll for transaction completion
        let mut time: u64 = 50;
        let mut i = 0;
        println!("{}", hex::encode(transaction.id.to_vec()));
        while i < 50 {
            i = i + 1;
            sleep(Duration::from_millis(time)).await;
            let res = self.get_transaction_result(transaction.id.to_vec()).await?;
            match res.status {
                0 | 1 | 2 | 3 => {
                    time = time + 200;
                }
                4 => {
                    if res.status_code == 1 {
                        // stop execution, error.
                        assert_ne!(res.error_message, res.error_message);
                    }
                    let new_account_address: flow::Event = res
                        .events
                        .into_iter()
                        .filter(|x| x.r#type == "flow.AccountCreated")
                        .collect::<Vec<flow::Event>>()
                        .pop()
                        .unwrap();
                    let payload: Value = serde_json::from_slice(&new_account_address.payload)?;
                    let address: String = payload["value"]["fields"][0]["value"]["value"]
                        .to_string()
                        .split_at(3)
                        .1
                        .to_string()
                        .split_at(16)
                        .0
                        .to_string();
                    let acct: flow::Account = self.get_account(address)
                        .await?
                        .account
                        .expect("could not get newly created account");
                    return Ok(acct);
                }
                _ => {
                    return Err("Cadence Runtime Error")?;
                }
            }
        }
        return Err("Could not produce result")?;
    }
}

// ****************************************************
// Utility Functionality
// ****************************************************

use serde::Serialize;
use serde_json::{json, Value};
use tokio::time::{sleep, Duration};

/// This is our argument builder.
#[derive(Serialize)]
pub struct Argument<T> {
    r#type: String,
    value: T,
}

impl Argument<Vec<Value>> {
    pub fn array(values: Vec<Value>) -> Argument<Vec<Value>> {
        return Argument {
            r#type: "Array".to_string(),
            value: values,
        };
    }
    pub fn dictionary(values: Vec<(String, String)>) -> Argument<Vec<Value>> {
        return Argument {
            r#type: "Dictionary".to_string(),
            value: values
                .into_iter()
                .map(|(x, y)| json!({"Key":x, "Value":y}))
                .collect(),
        };
    }
}

impl Argument<String> {
    pub fn string(value: String) -> Argument<String> {
        return Argument {
            r#type: "String".to_string(),
            value,
        };
    }
}

/// Utility function. Provides the ability to
fn padding(vec: &mut Vec<u8>, count: usize) {
    let mut i: usize = count;
    i = i - vec.len();
    while i > 0 {
        vec.push(0);
        i = i - 1;
    }
}

/// Construct a signature object. Pass this into the payload
/// or envelope signatures when signing a transaction.
pub struct Sign {
    pub address: String,
    pub key_id: u32,
    pub private_key: String,
}

/// build_transaction will construct a `flow::Transaction` with the provided script and arguments.
/// See the `Argument` struct for details on how to construct arguments.
pub async fn build_transaction(
    script: Vec<u8>,
    arguments: Vec<Vec<u8>>,
    reference_block_id: Vec<u8>,
    gas_limit: u64,
    proposer: TransactionProposalKey,
    authorizers: Vec<String>,
    payer: String,
) -> Result<Transaction, Box<dyn error::Error>> {
    Ok(Transaction {
        script,
        arguments: arguments,
        reference_block_id: reference_block_id,
        gas_limit: gas_limit,
        proposal_key: Some(proposer),
        authorizers: authorizers
            .iter()
            .map(|x| hex::decode(x).unwrap())
            .collect(),
        payload_signatures: vec![],
        envelope_signatures: vec![],
        payer: hex::decode(payer).unwrap(),
    })
}

/// Provides an envelope of the given transaction
fn envelope_from_transaction(
    transaction: Transaction,
    payload_signatures: &Vec<TransactionSignature>,
) -> Vec<u8> {
    let proposal_key = transaction.proposal_key.unwrap();
    let mut proposal_address = proposal_key.address;
    padding(&mut proposal_address, 8);
    let mut ref_block = transaction.reference_block_id;
    padding(&mut ref_block, 32);
    let mut stream = RlpStream::new_list(2);

    stream.begin_list(9);
    stream.append(&Bytes::from(transaction.script).to_vec());
    stream.begin_list(transaction.arguments.len());
    for (_i, arg) in transaction.arguments.into_iter().enumerate() {
        stream.append(&Bytes::from(arg).to_vec());
    }

    stream.append(&Bytes::from(ref_block).to_vec());
    stream.append(&transaction.gas_limit);
    stream.append(&Bytes::from(proposal_address).to_vec());
    stream.append(&proposal_key.key_id);
    stream.append(&proposal_key.sequence_number);
    stream.append(&Bytes::from(transaction.payer).to_vec());

    stream.begin_list(transaction.authorizers.len());
    for (_i, auth) in transaction.authorizers.into_iter().enumerate() {
        stream.append(&Bytes::from(auth).to_vec());
    }

    stream.begin_list(payload_signatures.len());
    for (i, sig) in payload_signatures.into_iter().enumerate() {
        let signature = sig.signature.to_vec();
        stream.begin_list(3);
        stream.append(&(i as u32));
        stream.append(&sig.key_id);
        stream.append(&signature);
    }

    let out = stream.out().to_vec();

    return out;
}

/// Provides a payload from a transaction
fn payload_from_transaction(transaction: Transaction) -> Vec<u8> {
    let proposal_key = transaction.proposal_key.unwrap();
    let mut proposal_address = proposal_key.address;
    padding(&mut proposal_address, 8);
    let mut ref_block = transaction.reference_block_id;
    padding(&mut ref_block, 32);

    let mut stream = RlpStream::new_list(9);
    stream.append(&Bytes::from(transaction.script).to_vec());
    stream.begin_list(transaction.arguments.len());
    for (_i, arg) in transaction.arguments.into_iter().enumerate() {
        stream.append(&Bytes::from(arg).to_vec());
    }

    stream.append(&Bytes::from(ref_block).to_vec());
    stream.append(&transaction.gas_limit);
    stream.append(&Bytes::from(proposal_address).to_vec());
    stream.append(&proposal_key.key_id);
    stream.append(&proposal_key.sequence_number);
    stream.append(&Bytes::from(transaction.payer).to_vec());

    stream.begin_list(transaction.authorizers.len());
    for (_i, auth) in transaction.authorizers.into_iter().enumerate() {
        stream.append(&Bytes::from(auth).to_vec());
    }

    let out = stream.out().to_vec();

    return out;
}

/// Returns the provided message as bytes, signed by the private key.
fn sign(message: Vec<u8>, private_key: String) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let secret_key = SecretKey::from_be_bytes(&hex::decode(private_key)?)?;
    let sig_key = SigningKey::from(secret_key);
    let signature = sig_key.sign(&message);
    Ok(signature.as_bytes().to_vec())
}
pub fn process_keys_args(account_keys: Vec<String>) -> Argument<Vec<Value>> {
    // do special processing for the keys, wrapping with algo, hash, and weight information:
    // algo: ECDSA_P256
    // hash: SHA3_256
    // weight: 1000
    return Argument::array(
        account_keys
            .into_iter()
            .map(|x| json!(Argument::string(format!("f847b840{}02038203e8", x))))
            .collect::<Vec<Value>>(),
    );
}
/// Sign the provided transaction.
/// You will first need to `build_transaction`.
pub async fn sign_transaction(
    built_transaction: Transaction,
    payload_signatures: Vec<&Sign>,
    envelope_signatures: Vec<&Sign>,
) -> Result<Option<Transaction>, Box<dyn error::Error>> {
    let mut payload: Vec<TransactionSignature> = vec![];
    let mut envelope: Vec<TransactionSignature> = vec![];
    // for each of the payload private keys, sign the transaction
    for signer in payload_signatures {
        let encoded_payload: &[u8] = &payload_from_transaction(built_transaction.clone());
        let mut domain_tag: Vec<u8> = b"FLOW-V0.0-transaction".to_vec();
        // we need to pad 0s at the end of the domain_tag
        padding(&mut domain_tag, 32);

        let fully_encoded: Vec<u8> = [&domain_tag, encoded_payload].concat();
        let mut addr = hex::decode(signer.address.clone()).unwrap();
        padding(&mut addr, 8);

        payload.push(TransactionSignature {
            address: addr,
            key_id: signer.key_id,
            signature: sign(fully_encoded, signer.private_key.clone())?,
        });
    }
    // for each of the envelope private keys, sign the transaction
    for signer in envelope_signatures {
        let encoded_payload: &[u8] =
            &envelope_from_transaction(built_transaction.clone(), &payload);
        let mut domain_tag: Vec<u8> = b"FLOW-V0.0-transaction".to_vec();
        // we need to pad 0s at the end of the domain_tag
        padding(&mut domain_tag, 32);

        let fully_encoded: Vec<u8> = [&domain_tag, encoded_payload].concat();
        let mut addr = hex::decode(signer.address.clone()).unwrap();
        padding(&mut addr, 8);

        envelope.push(TransactionSignature {
            address: addr,
            key_id: signer.key_id,
            signature: sign(fully_encoded, signer.private_key.clone())?,
        });
    }
    let signed_transaction = Some(Transaction {
        script: built_transaction.script,
        arguments: built_transaction.arguments,
        reference_block_id: built_transaction.reference_block_id,
        gas_limit: built_transaction.gas_limit,
        proposal_key: built_transaction.proposal_key,
        authorizers: built_transaction.authorizers,
        payload_signatures: payload,
        envelope_signatures: envelope,
        payer: built_transaction.payer,
    });
    Ok(signed_transaction)
}

// ****************************************************
// Testing
// ****************************************************

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn comprehensive_usage_case() {
        let service_account = std::env::vars()
            .filter(|kv| kv.0 == "SERVICE_ACCT")
            .map(|kv| kv.1)
            .collect::<Vec<String>>();
        let private_key = std::env::vars()
            .filter(|kv| kv.0 == "PRIV_K")
            .map(|kv| kv.1)
            .collect::<Vec<String>>();
        let public_key = std::env::vars()
            .filter(|kv| kv.0 == "PUB_K")
            .map(|kv| kv.1)
            .collect::<Vec<String>>();
        // let's first create an account

        // create the public and private keys

        // TODO

        // create the account

        // initiate the connection
        let mut connection = FlowConnection::new("https://access.devnet.nodes.onflow.org:9000").await.expect("Could not establish a connection");

        let payer = &service_account[0];
        let payer_private_key = &private_key[0];
        let public_keys = vec![public_key[0].to_owned()];

        let acct = connection.create_account(
            public_keys.to_vec(),
            &payer,
            &payer_private_key,
            0,
        )
        .await
        .expect("Could not create account");
        println!("{:?}", acct);

        // create a token contract

        // TODO

        // add contract to the newly created account

        // TODO

        // execute minting transaction

        // TODO

        // trade token for flow transaction

        // TODO

        // verify new balances

        // TODO
    }
}
