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

const WORKSHOP_STICKER_1_MINT: &str = "Akp5NZGTP24LWFUnMZywdi7G1ig9KZoKUhCTmomh9Swm";
const WORKSHOP_STICKER_2_MINT: &str = "8UPdrRe1FajsbHgpB6tgpghv8C3JmRvmjZMhrKDAK6aL";
const WORKSHOP_STICKER_3_MINT: &str = "5k6WYcBAtWZJJQF4RXXz1yA1M9AXUZS9gLETTqUxZGom";
const WORKSHOP_STICKER_4_MINT: &str = "3jx7SitnQVjz8tp6WH9YS1ZoY69ASpLBhKKArq9mNHQc";

const MY_STICKER_REDEEMER_PROGRAM_ID: &str = "25MoM9fShhmWXZy8EbQwsTpYFqMpUD9tarfJD7HoTLzN";

fn main() {
    let my_keypair = utils::load_config_keypair();
    let my_pubkey = my_keypair.pubkey();
    let rpc_client = utils::new_rpc_client();

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
    let my_program_id = Pubkey::from_str(MY_STICKER_REDEEMER_PROGRAM_ID).unwrap();

    // Step 2: Fetch a recent blockhash

    // Step 3: Create a transaction

    // Step 4: Send transaction

    // Step 5: Uncomment the println's below and run `cargo run`
    //
    //   This will print the address of your burned token account. Copy and paste the address into
    //   the workshop block explorer to see if it exists: https://defi-workshop.netlify.app/
    //
    //  - Note: be sure your transaction variable is named `tx`
    //
    // println!("Burned sticker token account: {}", new_token_pubkey);
    // println!("Transaction Signature: {}", utils::tx_signature(&tx));
}
