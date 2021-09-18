# flow-rust-sdk
This project is a WIP.


Here's what works right now:

* check_availability : Ping (check if endpoint is accepting requests)
* get_account : Gets the account at the requested location (remove the `0x` prefix from your address or decoding will fail)
* execute_script : Runs the provided script on-chain
* build_transaction : Build a provided cadence script and details into a Transaction object
* execute_transaction : Sends a built and signed transcation to the blockchain
* get_transaction_result : Checks the status / result of a transaction
* get_block : Gets the latest block by default
* sign_transaction : Signs the provided transaction.


List of To-Do (incomplete):

* Unit Testing - the project has 0 unit testing coverage at the moment.
* E2E Testing - yea, this also hasn't been done yet.
* get_collection has not been tested
* get_events_for_block_ids has not been tested
* get_events_for_height_range has not been tested


### Current Usage Example:

Add to your Cargo.toml:
```toml
[dependencies]
flow-rust-sdk = { git = "https://github.com/MarshallBelles/flow-rust-sdk.git", branch = "release" }
tokio = { version = "1.11.0", features = ["full"] }
serde_json = "1.0.67"
hex = "0.4.3"
```
(this will be released on Crates.io once feature-complete)


Usage within your main.rs:

```rs
use flow_rust_sdk::flow::*;
use flow_rust_sdk::{
    build_transaction, check_availability, execute_script, execute_transaction, get_account,
    get_block, get_transaction_result, sign_transaction,
};
use hex;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // check if the node is available
    check_availability("http://localhost:3569".to_string()).await?;

    // get account at address
    let acct: Account = get_account(
        "http://localhost:3569".to_string(),
        "f8d6e0586b0a20c7".to_string(),
    )
    .await?
    .account
    .unwrap();

    // Print out the address and balance of the account
    println!("Address: {:?}", hex::encode(&acct.address));
    println!("Balance: {:?}", &acct.balance);

    // Define script
    let script = b"
        pub fun main(): String {
            return \"Hello World On Flow!\"
        }";

    // Send script to the blockchain
    let script_results: ExecuteScriptResponse =
        execute_script("http://localhost:3569".to_string(), script.to_vec()).await?;
    let v: Value = serde_json::from_str(&String::from_utf8(script_results.value).unwrap())?;
    println!("{}", v["value"]);

    // define transaction, such as to create a new account
        let transaction = b"
            transaction() {
                prepare(signer: AuthAccount) {
                    let acct = AuthAccount(payer: signer)
                }
            }";
        // get the latest block for our transaction request
        let latest_block: BlockResponse =
            get_block("http://localhost:3569".to_string(), None, None, Some(false))
                .await
                .expect("Could not get latest block");

        // get account
        let acct: Account = get_account(
            "http://localhost:3569".to_string(),
            "f8d6e0586b0a20c7".to_string(),
        )
        .await
        .expect("Could not get account")
        .account
        .unwrap();

        // setup proposer
        let proposal_key: TransactionProposalKey = TransactionProposalKey {
            address: hex::decode("f8d6e0586b0a20c7").unwrap(),
            key_id: 0,
            sequence_number: acct.keys[0].sequence_number as u64,
        };

        let latest_block_id = latest_block.block.unwrap().id;

        // build the transaction
        let build: Transaction = build_transaction(
            transaction.to_vec(),
            vec![],
            latest_block_id,
            1000,
            proposal_key,
            ["f8d6e0586b0a20c7".to_string()].to_vec(),
            "eb179c27144f783c".to_string(),
        )
        .await
        .expect("Could not build transaction");

        // sign the transaction
        let authorizer = Sign {
            address: "f8d6e0586b0a20c7".to_owned(),
            key_id: 0,
            private_key: "324db577a741a9b7a2eb6cef4e37e72ff01a554bdbe4bd77ef9afe1cb00d3cec"
                .to_owned(),
        };
        // payer always signs the envelope
        let payer = Sign {
            address: "eb179c27144f783c".to_owned(),
            key_id: 0,
            private_key: "7cc3cac310c24e0bbd6a471b172fd306cf1e12502026a6ec390178a56ca70267"
                .to_owned(),
        };
        let signed: Option<Transaction> =
            sign_transaction(build, vec![&authorizer], vec![&payer])
                .await
                .expect("Could not sign transaction");

        // send to the blockchain
        let transaction_execution: SendTransactionResponse =
            execute_transaction("http://localhost:3569".to_string(), signed)
                .await
                .expect("Could not execute transaction");

        // get the result of the transaction execution
        let get_transaction_result: TransactionResultResponse = get_transaction_result(
            "http://localhost:3569".to_string(),
            transaction_execution.id,
        )
        .await
        .expect("Could not get transaction result");
        assert_eq!(0, get_transaction_result.status_code);
    Ok(())
}
```
