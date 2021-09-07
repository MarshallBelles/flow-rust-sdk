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


Here's what doesn't work:

* sign_transaction : I'm still working on the ECDSA signature process



List of To-Do (incomplete):

* Unit Testing - the project has 0 unit testing coverage at the moment.
* E2E Testing - yea, this also hasn't been done yet.
* get_collection has not been tested
* get_events_for_block_ids has not been tested
* get_events_for_height_range has not been tested