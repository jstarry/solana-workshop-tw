use solana_sdk::{
    instruction::Instruction, pubkey::Pubkey, signature::Signer,
    transaction::Transaction,
};
use std::str::FromStr;

mod utils;

const MEMO_PROGRAM_ID: &str = "Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo";
// Enter your own order id to receive a prize!
const EVENTBRITE_ORDER_ID: u64 = 0;

fn main() {
    let keypair = utils::load_config_keypair();
    let pubkey = keypair.pubkey();

    // Step 1: Create an SPL Memo instruction
    //
    //   Each Solana instruction includes a program id, list of account inputs,
    //   and a data vector. Memo instructions don't require any account inputs!
    //
    // Doc hints:
    //  - https://docs.rs/solana-sdk/1.5.8/solana_sdk/instruction/struct.Instruction.html#method.new
    //  - https://docs.rs/solana-sdk/1.5.8/solana_sdk/pubkey/struct.Pubkey.html#impl-FromStr
    let memo = format!("GIMME STICKER: {}", EVENTBRITE_ORDER_ID);
    let memo_program_id = Pubkey::from_str(MEMO_PROGRAM_ID).unwrap(); 
    // let memo_instruction = Instruction::new(...);

    // Step 2: Create a transaction with payer
    // 
    //   All transactions must pay fees!
    //
    // Doc hints:
    //  - https://docs.rs/solana-sdk/1.5.8/solana_sdk/transaction/struct.Transaction.html#method.new_with_payer
    let fee_payer = Some(&pubkey);

    // Step 3: Fetch a recent blockhash
    //  
    //   In order to make each transaction unique and prevent duplicate transactions (imagine you perform
    //   the same transaction of 50 NT to the same coffee shop every day) each transaction includes a
    //   recent blockhash. This blockhash also gives transactions a lifetime because a blockhash is only
    //   "recent" for about 2 minutes.
    //
    // Doc hints:
    //  - https://docs.rs/solana-client/1.5.8/solana_client/rpc_client/struct.RpcClient.html#method.get_recent_blockhash
    let rpc_client = utils::new_rpc_client();

    // Step 4: Sign transaction
    //
    //   Prove that you approve the transaction by signing with your keypair!
    //
    // Doc hints:
    //  - https://docs.rs/solana-sdk/1.5.8/solana_sdk/transaction/struct.Transaction.html#method.sign
    let keypairs = &[&keypair];

    // Step 5: Send transaction
    //
    //   Once signed, send your transaction to the Solana cluster for processing
    //
    // Doc hints:
    //  - https://docs.rs/solana-client/1.5.8/solana_client/rpc_client/struct.RpcClient.html#method.send_and_confirm_transaction_with_spinner

    // Step 6: Uncomment the println's below and run `cargo run`
    //
    //   This will print the signature of your transaction. Copy and paste the signature into
    //   the workshop block explorer to see if it was successful: https://defi-workshop.netlify.app/
    //
    //  - Note: be sure your transaction variable is named `tx`
    //
    // println!("Created Memo: {}", memo);
    // println!("Transaction Signature: {}", utils::tx_signature(&tx));
}
