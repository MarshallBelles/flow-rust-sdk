// ****************************************************
// Welcome to the MONOLITHIC flow-rust-sdk library!
// This project is under heavy construction
// Expect breaking changes
// Please read the license file
// ****************************************************

// ****************************************************
// External Dependencies
// ****************************************************
use std::error;

// ****************************************************
// Data Models
// ****************************************************

// These are the keys to our accounts
pub struct FlowKeys {
    public: String,
    private: String,
}

// This is the data representing our account on the blockchain
pub struct FlowAccount {}

// This is the result of our transaction execution
pub struct FlowTransactionResult {}

pub struct FlowBlock {}

pub struct FlowEvent {}

// ****************************************************
// Public Methods
// ****************************************************

// get_account expects the address and will return the FlowAccount or an Err
pub fn get_account(address: String) -> Result<FlowAccount, Box<dyn error::Error>> {}

// create_account expects an optional FlowKeys and will return FlowAccount or Err
// if FlowKeys is not provided, the service account keys from flow.config will
// be used with a weight of 1000
pub fn create_account(keys: Option<FlowKeys>) -> Result<FlowAccount, Box<dyn error::Error>> {}

pub fn add_contract(
    contract_name: String,
    contract: Vec<u8>,
    keys: Option<FlowKeys>,
) -> Result<FlowAccount, Box<dyn error::Error>> {
}

pub fn update_contract(
    contract_name: String,
    contract: Vec<u8>,
    keys: Option<FlowKeys>,
) -> Result<FlowAccount, Box<dyn error::Error>> {
}

pub fn remove_contract(
    contract_name: String,
    keys: Option<FlowKeys>,
) -> Result<FlowAccount, Box<dyn error::Error>> {
}

// deploy_project reads from the flow.config and deploys the project
// update_if_exists will allow updating the contract
pub fn deploy_project(
    contract_name: String,
    update_if_exists: Option<bool>,
) -> Result<FlowAccount, Box<dyn error::Error>> {
}

// execute_script will attempt to run the script and return the result as T or Error
pub fn execute_script<T>(script: Vec<u8>) -> Result<T, Box<dyn error::Error>> {}

// build, sign, and execute transaction
pub fn execute_transaction(
    transaction: Vec<u8>,
    proposer: FlowKeys,
    authorizer: FlowKeys,
    payer: FlowKeys,
    signers_inner: Vec<FlowKeys>,
    signers_outter: Vec<FlowKeys>,
) -> Result<FlowTransactionResult, Box<dyn error::Error>> {
}

// get_block accepts either the block_id or block_height. If neither are defined it returns the latest block.
pub fn get_block(
    block_id: Option<String>,
    block_height: Option<u32>,
) -> Result<FlowBlock, Box<dyn error::Error>> {
}

// TODO get_event description
pub fn get_event(event_name: String) -> Result<FlowEvent, Box<dyn error::Error>> {}

// TODO get collection description
pub fn get_collection(collection_id: String) -> Result<FlowEvent, Box<dyn error::Error>> {}


// ****************************************************
// Private Methods
// ****************************************************



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
