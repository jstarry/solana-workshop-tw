use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};
use std::str::FromStr;

mod utils;

// Copy/paste your deployed program id here:
const MY_STICKER_REDEEMER_PROGRAM_ID: &str = "";

const WS1_MINT: &str = "Akp5NZGTP24LWFUnMZywdi7G1ig9KZoKUhCTmomh9Swm";
const WS2_MINT: &str = "8UPdrRe1FajsbHgpB6tgpghv8C3JmRvmjZMhrKDAK6aL";
const WS3_MINT: &str = "5k6WYcBAtWZJJQF4RXXz1yA1M9AXUZS9gLETTqUxZGom";
const WS4_MINT: &str = "3jx7SitnQVjz8tp6WH9YS1ZoY69ASpLBhKKArq9mNHQc";

// Pick a sticker mint
const WORKSHOP_STICKER_MINT: &str = WS2_MINT;

// Search your address on the workshop explorer and then click "Tokens" to find
// all the sticker tokens you own. Copy/paste your token account address here:
const MY_STICKER_TOKEN: &str = "";

fn main() {
    let my_keypair = utils::load_config_keypair();
    let my_pubkey = my_keypair.pubkey();

    // Step 1: Create an instruction for your Sticker Redeemer program
    //
    //   Account inputs:
    //    1. My sticker token address
    //    2. Sticker mint address
    //    3. My pubkey
    //    4. SPL Token program id
    //
    // Doc hints:
    //  - https://docs.rs/solana-sdk/1.5.8/solana_sdk/instruction/struct.Instruction.html#method.new
    //  - https://docs.rs/solana-program/1.5.8/solana_program/instruction/struct.AccountMeta.html
    let my_program_id = Pubkey::from_str(MY_STICKER_REDEEMER_PROGRAM_ID).unwrap();
    let my_sticker_token = Pubkey::from_str(MY_STICKER_TOKEN).unwrap();
    let workshop_sticker_mint = Pubkey::from_str(WORKSHOP_STICKER_MINT).unwrap();

    // Step 2: Fetch a recent blockhash
    let rpc_client = utils::new_rpc_client();

    // Step 3: Create a transaction

    // Step 4: Send transaction

    // Step 5: Uncomment the println's below and run `cargo run`
    //
    //   This will print the address of your burned token account. Copy and paste the address into
    //   the workshop block explorer to see if it exists: https://defi-workshop.netlify.app/
    //
    //  - Note: be sure your transaction variable is named `tx`
    //
    // println!("Burned sticker token account: {}", my_sticker_token);
    // println!("Transaction Signature: {}", utils::tx_signature(&tx));
}
