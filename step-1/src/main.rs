use solana_sdk::{
    instruction::Instruction, message::Message, pubkey::Pubkey, signature::Signer,
    transaction::Transaction,
};
use std::str::FromStr;

mod utils;

const MEMO_PROGRAM_ID: &str = "Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo";
const ORDER_ID: u64 = 1606954633;

fn main() {
    let fee_payer = utils::load_config_keypair();
    let rpc_client = utils::new_rpc_client();
    let memo = format!("GIMME STICKER: {}", ORDER_ID);

    // Step 1: Create an SPL Memo instruction
    let memo_instruction = Instruction::new(
        Pubkey::from_str(MEMO_PROGRAM_ID).unwrap(),
        &memo,
        Vec::new(),
    );

    // Step 2: Fetch a recent blockhash
    let (recent_blockhash, _fee_calculator) = rpc_client
        .get_recent_blockhash()
        .expect("failed to get recent blockhash");

    // Step 3: Create a transaction
    let tx = Transaction::new(
        &[&fee_payer],
        Message::new(&[memo_instruction], Some(&fee_payer.pubkey())),
        recent_blockhash,
    );

    // Step 4: Send transaction
    rpc_client
        .send_and_confirm_transaction_with_spinner(&tx)
        .expect("tx failed");

    println!("Created Memo: {}", memo);
    println!("Transaction Signature: {}", utils::tx_signature(&tx));
}
