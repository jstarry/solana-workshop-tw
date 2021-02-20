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
const WORKSHOP_STICKER_2_TOKEN: &str = "8UPdrRe1FajsbHgpB6tgpghv8C3JmRvmjZMhrKDAK6aL";

fn main() {
    let keypair = utils::load_config_keypair();

    // Step 1: Fetch minimum required balance for SPL Token accounts
    //
    //   Each SPL Token account must be rent-exempt. Otherwise, tokens accounts
    //   which fail to pay rent could be unintentionally destroyed.
    //
    // Doc hints:
    //  - https://docs.rs/solana-client/1.5.8/solana_client/rpc_client/struct.RpcClient.html#method.get_minimum_balance_for_rent_exemption
    let rpc_client = utils::new_rpc_client();

    // Step 2: Create a System - Create Account instruction
    //
    //   Account Requirements:
    //    1. Must be owned by the SPL Token program
    //    2. Must have data size equivalent to an SPL Token account
    //    3. Must be rent-exempt
    //
    // Doc hints:
    //  - https://docs.rs/solana-sdk/1.5.8/solana_sdk/system_instruction/fn.create_account.html
    //  - https://docs.rs/spl-token/3.1.0/spl_token/static.ID.html
    let token_keypair = Keypair::new(); // Random keypair

    // Step 3: Create a SPL Token - Initialize Account instruction
    //
    //   Initialize a new token account for the Workshop Sticker #2 token!
    //
    // Doc hints:
    //  - https://docs.rs/spl-token/3.1.0/spl_token/instruction/fn.initialize_account.html
    let sticker_token_pubkey = Pubkey::from_str(WORKSHOP_STICKER_2_TOKEN).unwrap();

    // Step 4: Fetch a recent blockhash

    // Step 5: Create a transaction
    //
    //   Try creating this with `Transaction::new` this time! It will create a signed
    //   transaction, so no need to call `tx.sign(..)`.
    //
    //   Hint: Make sure to add `token_keypair` as a signer! A signature is always required
    //   when creating new accounts. Otherwise, anyone could create an account for keypairs
    //   they didn't own.
    //
    // Doc hints:
    //  - https://docs.rs/solana-sdk/1.5.8/solana_sdk/transaction/struct.Transaction.html#method.new
    //  - https://docs.rs/solana-sdk/1.5.8/solana_sdk/message/struct.Message.html#method.new

    // Step 6: Send transaction

    // Step 6: Uncomment the println's below and run `cargo run`
    //
    //   This will print the address of your token account. Copy and paste the address into
    //   the workshop block explorer to see if it exists: https://defi-workshop.netlify.app/
    //
    //  - Note: be sure your transaction variable is named `tx`
    //
    // println!("Created sticker token account: {}", token_keypair.pubkey());
    // println!("Transaction Signature: {}", utils::tx_signature(&tx));
}
