// ****************************************************
// Welcome to the flow-rust-sdk!
// License: Apache V2.0 OR MIT, at your option
// ****************************************************

// ****************************************************
// External Dependencies
// ****************************************************
use std::error;

use flow::access_api_client::AccessApiClient;

use flow::{
    AccountResponse, BlockResponse, CollectionResponse, EventsResponse,
    ExecuteScriptAtLatestBlockRequest, ExecuteScriptResponse, GetAccountAtLatestBlockRequest,
    GetBlockByHeightRequest, GetBlockByIdRequest, GetCollectionByIdRequest,
    GetEventsForBlockIdsRequest, GetEventsForHeightRangeRequest, GetLatestBlockRequest,
    GetTransactionRequest, PingRequest, SendTransactionRequest, SendTransactionResponse,
    Transaction, TransactionProposalKey, TransactionResponse, TransactionResultResponse,
    TransactionSignature,
};

pub mod flow {
    tonic::include_proto!("flow.access");
}

// for signing transactions
use bytes::Bytes;
use p256::ecdsa::{signature::Signature, signature::Signer, SigningKey};
use p256::elliptic_curve::SecretKey;
pub extern crate hex;
extern crate rlp;
use rlp::*;

// ****************************************************
// Public Methods
// ****************************************************

/// check the availability of the node at network_address
/// if this times out, it's probably because the endpoint timed out.
pub async fn check_availability(network_address: &String) -> Result<(), Box<dyn error::Error>> {
    let mut client = AccessApiClient::connect(network_address.clone()).await?;

    let request = tonic::Request::new(PingRequest {});

    client.ping(request).await?;

    Ok(())
}

/// get_account expects the address and will return the Account or an Err
pub async fn get_account(
    network_address: &String,
    account_address: String,
) -> Result<AccountResponse, Box<dyn error::Error>> {
    let mut client = AccessApiClient::connect(network_address.clone()).await?;

    let request = tonic::Request::new(GetAccountAtLatestBlockRequest {
        address: hex::decode(account_address).unwrap(),
    });

    let response = client.get_account_at_latest_block(request).await?;

    Ok(response.into_inner())
}

/// execute_script will attempt to run the script and return the result as T or Error
pub async fn execute_script(
    network_address: &String,
    script: Vec<u8>,
) -> Result<ExecuteScriptResponse, Box<dyn error::Error>> {
    let mut client = AccessApiClient::connect(network_address.clone()).await?;

    let request = tonic::Request::new(ExecuteScriptAtLatestBlockRequest { script });

    let response = client.execute_script_at_latest_block(request).await?;

    Ok(response.into_inner())
}

/// build
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

pub struct Sign {
    pub address: String,
    pub key_id: u32,
    pub private_key: String,
}

pub struct CanPaySig {
    pub signer_index: u32,
    pub key_id: u32,
    pub signature: Vec<u8>,
}

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

fn sign(message: Vec<u8>, private_key: String) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let secret_key = SecretKey::from_be_bytes(&hex::decode(private_key)?)?;
    let sig_key = SigningKey::from(secret_key);
    let signature = sig_key.sign(&message);
    println!("msg {}", hex::encode(message));
    println!("sig {}", hex::encode(signature.as_bytes()));

    Ok(signature.as_bytes().to_vec())
}

fn padding(vec: &mut Vec<u8>, count: usize) {
    let mut i: usize = count;
    i = i - vec.len();
    while i > 0 {
        vec.push(0);
        i = i - 1;
    }
}

/// sign
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

/// Sends the transaction to the blockchain.
pub async fn send_transaction(
    network_address: &String,
    transaction: Option<Transaction>,
) -> Result<SendTransactionResponse, Box<dyn error::Error>> {
    // send to blockchain
    let mut client = AccessApiClient::connect(network_address.clone()).await?;

    let request = tonic::Request::new(SendTransactionRequest { transaction });

    let response = client.send_transaction(request).await?;

    Ok(response.into_inner())
}

/// get transaction result
pub async fn get_transaction_result(
    network_address: &String,
    id: Vec<u8>,
) -> Result<TransactionResultResponse, Box<dyn error::Error>> {
    // send to blockchain
    let mut client = AccessApiClient::connect(network_address.clone()).await?;

    let request = tonic::Request::new(GetTransactionRequest { id });

    let response = client.get_transaction_result(request).await?;

    Ok(response.into_inner())
}

/// get transaction result
pub async fn get_transaction(
    network_address: &String,
    id: Vec<u8>,
) -> Result<TransactionResponse, Box<dyn error::Error>> {
    // send to blockchain
    let mut client = AccessApiClient::connect(network_address.clone()).await?;

    let request = tonic::Request::new(GetTransactionRequest { id });

    let response = client.get_transaction(request).await?;

    Ok(response.into_inner())
}

/// get_block accepts either the block_id or block_height. If neither are defined it returns the latest block.
pub async fn get_block(
    network_address: &String,
    block_id: Option<String>,
    block_height: Option<u64>,
    is_sealed: Option<bool>,
) -> Result<BlockResponse, Box<dyn error::Error>> {
    if block_id.is_some() {
        // IF block_id, use this
        let mut client = AccessApiClient::connect(network_address.clone()).await?;
        let request = tonic::Request::new(GetBlockByIdRequest {
            id: block_id.unwrap().as_bytes().to_vec(),
        });
        let response = client.get_block_by_id(request).await?;
        Ok(response.into_inner())
    } else if block_height.is_some() {
        // else IF block_height, use that
        let mut client = AccessApiClient::connect(network_address.clone()).await?;
        let request = tonic::Request::new(GetBlockByHeightRequest {
            height: block_height.unwrap(),
        });
        let response = client.get_block_by_height(request).await?;
        Ok(response.into_inner())
    } else {
        // else, just get latest block
        if is_sealed.is_some() {
            let mut client = AccessApiClient::connect(network_address.clone()).await?;
            let request = tonic::Request::new(GetLatestBlockRequest {
                is_sealed: is_sealed.unwrap(),
            });
            let response = client.get_latest_block(request).await?;
            Ok(response.into_inner())
        } else {
            let mut client = AccessApiClient::connect(network_address.clone()).await?;
            let request = tonic::Request::new(GetLatestBlockRequest { is_sealed: false });
            let response = client.get_latest_block(request).await?;
            Ok(response.into_inner())
        }
    }
}

/// retrieve the specified events by type for the given height range
pub async fn get_events_for_height_range(
    network_address: &String,
    event_type: String,
    start_height: u64,
    end_height: u64,
) -> Result<EventsResponse, Box<dyn error::Error>> {
    let mut client = AccessApiClient::connect(network_address.clone()).await?;
    let request = tonic::Request::new(GetEventsForHeightRangeRequest {
        r#type: event_type,
        start_height,
        end_height,
    });
    let response = client.get_events_for_height_range(request).await?;
    Ok(response.into_inner())
}

/// retrieve the specified events by type for the given blocks
pub async fn get_events_for_block_ids(
    network_address: &String,
    event_type: String,
    ids: Vec<Vec<u8>>,
) -> Result<EventsResponse, Box<dyn error::Error>> {
    let mut client = AccessApiClient::connect(network_address.clone()).await?;
    let request = tonic::Request::new(GetEventsForBlockIdsRequest {
        r#type: event_type,
        block_ids: ids,
    });
    let response = client.get_events_for_block_i_ds(request).await?;
    Ok(response.into_inner())
}

/// retrieve the specified collections
pub async fn get_collection(
    network_address: &String,
    collection_id: Vec<u8>,
) -> Result<CollectionResponse, Box<dyn error::Error>> {
    let mut client = AccessApiClient::connect(network_address.clone()).await?;
    let request = tonic::Request::new(GetCollectionByIdRequest { id: collection_id });
    let response = client.get_collection_by_id(request).await?;
    Ok(response.into_inner())
}

// ****************************************************
// Utility Functionality
// ****************************************************

use serde::Serialize;
use serde_json::{json, Value};
use tokio::time::{sleep, Duration};

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

fn process_keys_args(account_keys: Vec<String>) -> Argument<Vec<Value>> {
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

pub async fn create_account(
    network_address: &String,
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

    let latest_block: BlockResponse = get_block(network_address, None, None, Some(false)).await?;

    let account: flow::Account = get_account(network_address, payer.clone())
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
        send_transaction(network_address, transaction).await?;

    // poll for transaction completion
    let mut time: u64 = 50;
    let mut i = 0;
    println!("{}", hex::encode(transaction.id.to_vec()));
    while i < 50 {
        i = i + 1;
        sleep(Duration::from_millis(time)).await;
        let res = get_transaction_result(network_address, transaction.id.to_vec()).await?;
        match res.status {
            0 | 1 | 2 | 3 => {
                time = time + 200;
            }
            4 => {
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
                let acct: flow::Account = get_account(network_address, address)
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

// ****************************************************
// Testing
// ****************************************************

#[cfg(test)]
mod tests {
    use super::*;
    extern crate secret_service;
    use secret_service::EncryptionType;
    use secret_service::SecretService;
    use std::error::Error;

    #[tokio::test]
    async fn comprehensive_usage_case() {
        // get secrets from github
        let ss = SecretService::new(EncryptionType::Dh)?;

        // get default collection
        let collection = ss.get_default_collection()?;

        // search items by properties
        let search_items = ss.search_items(vec![("TESTNET_1_ADDRESS", "TESTNET_1_PRIVATE_KEY", "TESTNET_1_PUBLIC_KEY")])?;

        let service_account_address = search_items.get(0)?.get_secret()?;
        let service_account_priv = search_items.get(1)?.get_secret()?;
        let service_account_pub = search_items.get(2)?.get_secret()?;

        // create the public and private keys

        // TODO

        // create the account

        let network_address = "https://access.devnet.nodes.onflow.org:9000".to_string();
        let payer = service_account_address.to_string();
        let payer_private_key = service_account_priv.to_string();
        let new_account_keys = vec![service_account_pub.to_string()];

        let acct = create_account(
            &network_address,
            new_account_keys.to_vec(),
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
