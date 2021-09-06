// ****************************************************
// Welcome to the flow-rust-sdk!
// Please read the license file
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
    PingRequest, SendTransactionRequest, SendTransactionResponse, Transaction,
    TransactionProposalKey, TransactionSignature,
};

pub mod flow {
    tonic::include_proto!("flow.access");
}

// ****************************************************
// Public Methods
// ****************************************************

// check the availability of the node at network_address
// if this times out, it's probably because the endpoint timed out.
pub async fn check_availability(network_address: String) -> Result<(), Box<dyn error::Error>> {
    let mut client = AccessApiClient::connect(network_address).await?;

    let request = tonic::Request::new(PingRequest {});

    client.ping(request).await?;

    Ok(())
}

// get_account expects the address and will return the Account or an Err
pub async fn get_account(
    network_address: String,
    address: String,
) -> Result<AccountResponse, Box<dyn error::Error>> {
    let mut client = AccessApiClient::connect(network_address).await?;

    let request = tonic::Request::new(GetAccountAtLatestBlockRequest {
        address: address.as_bytes().to_vec(),
    });

    let response = client.get_account_at_latest_block(request).await?;

    Ok(response.into_inner())
}

// execute_script will attempt to run the script and return the result as T or Error
pub async fn execute_script(
    network_address: String,
    script: Vec<u8>,
) -> Result<ExecuteScriptResponse, Box<dyn error::Error>> {
    let mut client = AccessApiClient::connect(network_address).await?;

    let request = tonic::Request::new(ExecuteScriptAtLatestBlockRequest { script });

    let response = client.execute_script_at_latest_block(request).await?;

    Ok(response.into_inner())
}

// build
pub async fn build_transaction(
    script: Vec<u8>,
    arguments: Vec<Vec<u8>>,
    reference_block_id: Vec<u8>,
    gas_limit: u64,
    proposer: TransactionProposalKey,
    authorizers: Vec<Vec<u8>>,
    payer: Vec<u8>,
) -> Result<Transaction, Box<dyn error::Error>> {
    Ok(Transaction {
        script,
        arguments: arguments,
        reference_block_id: reference_block_id,
        gas_limit: gas_limit,
        proposal_key: Some(proposer),
        authorizers: authorizers,
        payload_signatures: vec![],
        envelope_signatures: vec![],
        payer: payer,
    })
}

// sign
pub async fn sign_transaction(
    built_transaction: Transaction,
    payload_signatures: Vec<TransactionSignature>,
    envelope_signatures: Vec<TransactionSignature>,
) -> Result<Option<Transaction>, Box<dyn error::Error>> {
    let signed_transaction = Some(Transaction {
        script: built_transaction.script,
        arguments: built_transaction.arguments,
        reference_block_id: built_transaction.reference_block_id,
        gas_limit: built_transaction.gas_limit,
        proposal_key: built_transaction.proposal_key,
        authorizers: built_transaction.authorizers,
        payload_signatures: payload_signatures,
        envelope_signatures: envelope_signatures,
        payer: built_transaction.payer,
    });
    Ok(signed_transaction)
}

// execute transaction
pub async fn execute_transaction(
    network_address: String,
    transaction: Option<Transaction>,
) -> Result<SendTransactionResponse, Box<dyn error::Error>> {
    // send to blockchain
    let mut client = AccessApiClient::connect(network_address).await?;

    let request = tonic::Request::new(SendTransactionRequest { transaction });

    let response = client.send_transaction(request).await?;

    Ok(response.into_inner())
}

// get_block accepts either the block_id or block_height. If neither are defined it returns the latest block.
pub async fn get_block(
    network_address: String,
    block_id: Option<String>,
    block_height: Option<u64>,
    is_sealed: Option<bool>,
) -> Result<BlockResponse, Box<dyn error::Error>> {
    if block_id.is_some() {
        // IF block_id, use this
        let mut client = AccessApiClient::connect(network_address).await?;
        let request = tonic::Request::new(GetBlockByIdRequest {
            id: block_id.unwrap().as_bytes().to_vec(),
        });
        let response = client.get_block_by_id(request).await?;
        Ok(response.into_inner())
    } else if block_height.is_some() {
        // else IF block_height, use that
        let mut client = AccessApiClient::connect(network_address).await?;
        let request = tonic::Request::new(GetBlockByHeightRequest {
            height: block_height.unwrap(),
        });
        let response = client.get_block_by_height(request).await?;
        Ok(response.into_inner())
    } else {
        // else, just get latest block
        if is_sealed.is_some() {
            let mut client = AccessApiClient::connect(network_address).await?;
            let request = tonic::Request::new(GetLatestBlockRequest {
                is_sealed: is_sealed.unwrap(),
            });
            let response = client.get_latest_block(request).await?;
            Ok(response.into_inner())
        } else {
            let mut client = AccessApiClient::connect(network_address).await?;
            let request = tonic::Request::new(GetLatestBlockRequest { is_sealed: true });
            let response = client.get_latest_block(request).await?;
            Ok(response.into_inner())
        }
    }
}

// retrieve the specified events by type for the given height range
pub async fn get_events_for_height_range(
    network_address: String,
    event_type: String,
    start_height: u64,
    end_height: u64,
) -> Result<EventsResponse, Box<dyn error::Error>> {
    let mut client = AccessApiClient::connect(network_address).await?;
    let request = tonic::Request::new(GetEventsForHeightRangeRequest {
        r#type: event_type,
        start_height,
        end_height,
    });
    let response = client.get_events_for_height_range(request).await?;
    Ok(response.into_inner())
}

// retrieve the specified events by type for the given blocks
pub async fn get_events_for_block_ids(
    network_address: String,
    event_type: String,
    ids: Vec<Vec<u8>>,
) -> Result<EventsResponse, Box<dyn error::Error>> {
    let mut client = AccessApiClient::connect(network_address).await?;
    let request = tonic::Request::new(GetEventsForBlockIdsRequest {
        r#type: event_type,
        block_ids: ids,
    });
    let response = client.get_events_for_block_i_ds(request).await?;
    Ok(response.into_inner())
}

// retrieve the specified collections
pub async fn get_collection(
    network_address: String,
    collection_id: Vec<u8>,
) -> Result<CollectionResponse, Box<dyn error::Error>> {
    let mut client = AccessApiClient::connect(network_address).await?;
    let request = tonic::Request::new(GetCollectionByIdRequest { id: collection_id });
    let response = client.get_collection_by_id(request).await?;
    Ok(response.into_inner())
}

// ****************************************************
// Testing
// ****************************************************

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
