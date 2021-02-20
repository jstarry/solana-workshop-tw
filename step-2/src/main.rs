use solana_sdk::{
    message::Message,
    program_pack::Pack,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction::create_account,
    transaction::Transaction,
};
use spl_token::instruction::initialize_account;
use std::str::FromStr;

mod utils;

const TOKEN_ACCOUNT_SIZE: usize = spl_token::state::Account::LEN;
const WORKSHOP_STICKER_MINT: &str = "8UPdrRe1FajsbHgpB6tgpghv8C3JmRvmjZMhrKDAK6aL";

fn main() {
    let fee_payer = utils::load_config_keypair();
    let rpc_client = utils::new_rpc_client();
    let token_keypair = Keypair::new();
    let workshop_sticker_mint = Pubkey::from_str(WORKSHOP_STICKER_MINT).unwrap();

    // Step 1: Fetch minimum required balance for SPL Token accounts
    let token_balance = rpc_client
        .get_minimum_balance_for_rent_exemption(TOKEN_ACCOUNT_SIZE)
        .expect("failed to get min balance");

    // Step 2: Create a System - Create Account instruction
    let create_account_instruction = create_account(
        &fee_payer.pubkey(),
        &token_keypair.pubkey(),
        token_balance,
        TOKEN_ACCOUNT_SIZE as u64,
        &spl_token::ID,
    );

    // Step 3: Create a SPL Token - Initialize Account instruction
    let initialize_account_instruction = initialize_account(
        &spl_token::ID,
        &token_keypair.pubkey(),
        &workshop_sticker_mint,
        &fee_payer.pubkey(),
    )
    .unwrap();

    // Step 4: Fetch a recent blockhash
    let (recent_blockhash, _fee_calculator) = rpc_client
        .get_recent_blockhash()
        .expect("failed to get recent blockhash");

    // Step 5: Create a transaction
    let tx = Transaction::new(
        &[&fee_payer, &token_keypair],
        Message::new(
            &[create_account_instruction, initialize_account_instruction],
            Some(&fee_payer.pubkey()),
        ),
        recent_blockhash,
    );

    // Step 6: Send transaction
    rpc_client
        .send_and_confirm_transaction_with_spinner(&tx)
        .expect("tx failed");

    println!("Created sticker token account: {}", token_keypair.pubkey());
    println!("Transaction Signature: {}", utils::tx_signature(&tx));
}
