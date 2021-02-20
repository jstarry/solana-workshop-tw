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
const MY_STICKER_REDEEMER_PROGRAM_ID: &str = "25MoM9fShhmWXZy8EbQwsTpYFqMpUD9tarfJD7HoTLzN";

const WS1_MINT: &str = "Akp5NZGTP24LWFUnMZywdi7G1ig9KZoKUhCTmomh9Swm";
const WS2_MINT: &str = "8UPdrRe1FajsbHgpB6tgpghv8C3JmRvmjZMhrKDAK6aL";
const WS3_MINT: &str = "5k6WYcBAtWZJJQF4RXXz1yA1M9AXUZS9gLETTqUxZGom";
const WS4_MINT: &str = "3jx7SitnQVjz8tp6WH9YS1ZoY69ASpLBhKKArq9mNHQc";

// Pick a sticker mint
const WORKSHOP_STICKER_MINT: &str = WS2_MINT;

// Search your address on the workshop explorer and then click "Tokens" to find
// all the sticker tokens you own. Copy/paste your token account address here:
const MY_STICKER_TOKEN: &str = "2UsyPHVgUWMrdGDfZBeJu7E3f3CerfwxgjRqDTRJyALk";

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
    let redeem_sticker_ix = Instruction::new(
        my_program_id,
        &(),
        vec![
            AccountMeta::new(my_sticker_token, false),
            AccountMeta::new(workshop_sticker_mint, false),
            AccountMeta::new(my_pubkey, true),
            AccountMeta::new_readonly(spl_token::ID, false),
        ],
    );

    // Step 2: Fetch a recent blockhash
    let rpc_client = utils::new_rpc_client();
    let (recent_blockhash, _fee_calculator) = rpc_client
        .get_recent_blockhash()
        .expect("failed to get recent blockhash");

    // Step 3: Create a transaction
    let tx = Transaction::new(
        &[&my_keypair],
        Message::new(&[redeem_sticker_ix], Some(&my_pubkey)),
        recent_blockhash,
    );

    // Step 4: Send transaction
    rpc_client
        .send_and_confirm_transaction_with_spinner(&tx)
        .expect("tx failed");

    // Step 5: Uncomment the println's below and run `cargo run`
    //
    //   This will print the address of your burned token account. Copy and paste the address into
    //   the workshop block explorer to see if it exists: https://defi-workshop.netlify.app/
    //
    //  - Note: be sure your transaction variable is named `tx`
    //
    println!("Burned sticker token account: {}", my_sticker_token);
    println!("Transaction Signature: {}", utils::tx_signature(&tx));
}
