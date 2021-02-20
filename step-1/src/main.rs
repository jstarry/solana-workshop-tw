use solana_sdk::{
    instruction::Instruction, message::Message, pubkey::Pubkey, signature::Signer,
    transaction::Transaction,
};
use std::str::FromStr;

mod utils;

// Used in Step 1
const MEMO_PROGRAM_ID: &str = "Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo";
const EVENTBRITE_ORDER_ID: u64 = 0;

fn main() {
    let fee_payer = utils::load_config_keypair();
    let rpc_client = utils::new_rpc_client();
    let memo = format!("GIMME STICKER: {}", EVENTBRITE_ORDER_ID);

    // Step 1: Create an SPL Memo instruction
    // Doc hints:
    //  - https://docs.rs/solana-sdk/1.5.8/solana_sdk/instruction/struct.Instruction.html#method.new
    //  - https://docs.rs/solana-sdk/1.5.8/solana_sdk/pubkey/struct.Pubkey.html#impl-FromStr

    // Step 2: Create a transaction
    // Doc hints:
    //  - https://docs.rs/solana-sdk/1.5.8/solana_sdk/transaction/struct.Transaction.html#method.new_with_payer

    // Step 3: Fetch a recent blockhash
    // Doc hints:
    //  - https://docs.rs/solana-client/1.5.8/solana_client/rpc_client/struct.RpcClient.html#method.get_recent_blockhash

    // Step 4: Sign transaction
    // Doc hints:
    //  - https://docs.rs/solana-sdk/1.5.8/solana_sdk/transaction/struct.Transaction.html#method.sign

    // Step 5: Send transaction
    // Doc hints:
    //  - https://docs.rs/solana-client/1.5.8/solana_client/rpc_client/struct.RpcClient.html#method.send_and_confirm_transaction_with_spinner

    // Step 6: Uncomment and run `cargo run`
    //  - Note: be sure your transaction variable is named `tx`
    // println!("Created Memo: {}", memo);
    // println!("Transaction Signature: {}", utils::tx_signature(&tx));
}
