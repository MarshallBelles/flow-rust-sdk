// ****************************************************
// Welcome to the flow-rust-sdk!
// Please read the license file
// ****************************************************

// ****************************************************
// External Dependencies
// ****************************************************
use std::error;

use flow::api_client::ApiClient;

use flow::{
    AccountKey, AccountResponse, BlockResponse, CollectionResponse, EventsResponse,
    ExecuteScriptAtLatestBlockRequest, ExecuteScriptResponse, GetAccountAtLatestBlockRequest,
    GetBlockByHeightRequest, GetBlockByIdRequest, GetCollectionByIdRequest,
    GetEventsForBlockIdsRequest, GetEventsForHeightRangeRequest, GetLatestBlockRequest,
    PingRequest, SendTransactionRequest, SendTransactionResponse, Transaction,
};

pub mod flow {
    tonic::include_proto!("flowrust");
}

// ****************************************************
// Public Methods
// ****************************************************

// check the availability of the blockchain
// if this times out, it's probably because the endpoint timed out.
pub async fn check_availability(network_address: String) -> Result<(), Box<dyn error::Error>> {
    let mut client = ApiClient::connect(network_address).await?;

    let request = tonic::Request::new(PingRequest {});

    client.ping(request).await?;

    Ok(())
}

// get_account expects the address and will return the Account or an Err
pub async fn get_account(
    network_address: String,
    address: String,
) -> Result<AccountResponse, Box<dyn error::Error>> {
    let mut client = ApiClient::connect(network_address).await?;

    let request = tonic::Request::new(GetAccountAtLatestBlockRequest {
        address: address.as_bytes().to_vec(),
    });

    let response = client.get_account_at_latest_block(request).await?;

    Ok(response.into_inner())
}

// TODO: create account with provided public key
pub async fn create_account(
    network_address: String,
    key: AccountKey,
) -> Result<AccountResponse, Box<dyn error::Error>> {
    // Send to chain
    let mut client = ApiClient::connect(network_address).await?;

    //
    //
    //
    // find the replacement for GetAccountAtLatestBlockRequest that will create an account
    //
    //
    //
    let request = tonic::Request::new(GetAccountAtLatestBlockRequest {
        address: "".as_bytes().to_vec(),
    });

    //
    //
    // implement the above, rebuild protoc generated code
    // edit get_account_at_latest_block to create_account or w/e it is
    //
    //
    let response = client.get_account_at_latest_block(request).await?;

    Ok(response.into_inner())
}

// execute_script will attempt to run the script and return the result as T or Error
pub async fn execute_script(
    network_address: String,
    script: Vec<u8>,
) -> Result<ExecuteScriptResponse, Box<dyn error::Error>> {
    let mut client = ApiClient::connect(network_address).await?;

    let request = tonic::Request::new(ExecuteScriptAtLatestBlockRequest { script });

    let response = client.execute_script_at_latest_block(request).await?;

    Ok(response.into_inner())
}

// build, sign, and execute transaction
pub async fn execute_transaction(
    network_address: String,
    proposer: AccountKey,
    authorizer: AccountKey,
    transaction: Vec<u8>,
    payer: AccountKey,
    payload_signatures: Vec<AccountKey>,
    envelope_signatures: Vec<AccountKey>,
) -> Result<SendTransactionResponse, Box<dyn error::Error>> {
    // build transaction
    let build: Option<Transaction> = None;
    //
    //
    //
    //

    // sign transaction
    //
    //
    //
    //

    // send to blockchain
    let mut client = ApiClient::connect(network_address).await?;

    let request = tonic::Request::new(SendTransactionRequest { transaction: build });

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
        let mut client = ApiClient::connect(network_address).await?;
        let request = tonic::Request::new(GetBlockByIdRequest {
            id: block_id.unwrap().as_bytes().to_vec(),
        });
        let response = client.get_block_by_id(request).await?;
        Ok(response.into_inner())
    } else if block_height.is_some() {
        // else IF block_height, use that
        let mut client = ApiClient::connect(network_address).await?;
        let request = tonic::Request::new(GetBlockByHeightRequest {
            height: block_height.unwrap(),
        });
        let response = client.get_block_by_height(request).await?;
        Ok(response.into_inner())
    } else {
        // else, just get latest block
        if is_sealed.is_some() {
            let mut client = ApiClient::connect(network_address).await?;
            let request = tonic::Request::new(GetLatestBlockRequest {
                is_sealed: is_sealed.unwrap(),
            });
            let response = client.get_latest_block(request).await?;
            Ok(response.into_inner())
        } else {
            let mut client = ApiClient::connect(network_address).await?;
            let request = tonic::Request::new(GetLatestBlockRequest { is_sealed: true });
            let response = client.get_latest_block(request).await?;
            Ok(response.into_inner())
        }
    }
}

//
pub async fn get_events_for_height_range(
    network_address: String,
    event_type: String,
    start_height: u64,
    end_height: u64,
) -> Result<EventsResponse, Box<dyn error::Error>> {
    let mut client = ApiClient::connect(network_address).await?;
    let request = tonic::Request::new(GetEventsForHeightRangeRequest {
        r#type: event_type,
        start_height,
        end_height,
    });
    let response = client.get_events_for_height_range(request).await?;
    Ok(response.into_inner())
}

//
pub async fn get_events_for_block_ids(
    network_address: String,
    event_type: String,
    ids: Vec<Vec<u8>>,
) -> Result<EventsResponse, Box<dyn error::Error>> {
    let mut client = ApiClient::connect(network_address).await?;
    let request = tonic::Request::new(GetEventsForBlockIdsRequest {
        r#type: event_type,
        block_ids: ids,
    });
    let response = client.get_events_for_block_i_ds(request).await?;
    Ok(response.into_inner())
}

//
pub async fn get_collection(
    network_address: String,
    collection_id: Vec<u8>,
) -> Result<CollectionResponse, Box<dyn error::Error>> {
    let mut client = ApiClient::connect(network_address).await?;
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
