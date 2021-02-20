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

const WORKSHOP_STICKER_2_MINT: &str = "8UPdrRe1FajsbHgpB6tgpghv8C3JmRvmjZMhrKDAK6aL";

fn main() {
    let my_keypair = utils::load_config_keypair();
    let my_pubkey = my_keypair.pubkey();

    // Step 1: Fetch minimum required balance for SPL Token accounts
    //
    //   Each SPL Token account must be rent-exempt. Otherwise, tokens accounts
    //   which fail to pay rent could be unintentionally destroyed.
    //
    // Doc hints:
    //  - https://docs.rs/solana-client/1.5.8/solana_client/rpc_client/struct.RpcClient.html#method.get_minimum_balance_for_rent_exemption
    let token_account_size = spl_token::state::Account::LEN;
    let rpc_client = utils::new_rpc_client();
    let token_balance = rpc_client
        .get_minimum_balance_for_rent_exemption(token_account_size)
        .expect("failed to get min balance");

    // Step 2: Create a "Create Account" instruction for the System program
    //
    //   SPL Token account requirements:
    //    1. Must be owned by the SPL Token program
    //    2. Must have data size equivalent to an SPL Token account
    //    3. Must be rent-exempt
    //
    //   The System "Create Account" instruction does three things:
    //    1. Transfers `lamports` from an account to the new account
    //    2. Allocates `space` bytes of data for the new account
    //    3. Assigns an account to a program
    //
    // Doc hints:
    //  - https://docs.rs/solana-sdk/1.5.8/solana_sdk/system_instruction/fn.create_account.html
    //  - https://docs.rs/spl-token/3.1.0/spl_token/static.ID.html
    let new_token_keypair = Keypair::new(); // New random keypair
    let new_token_pubkey = new_token_keypair.pubkey();
    let create_account_instruction = create_account(
        &my_pubkey,
        &new_token_pubkey,
        token_balance,
        token_account_size as u64,
        &spl_token::ID,
    );

    // Step 3: Create a "Initialize Account" instruction for the SPL Token program
    //
    //   Initialize a new token account for the Workshop Sticker #2 token!
    //
    // Doc hints:
    //  - https://docs.rs/spl-token/3.1.0/spl_token/instruction/fn.initialize_account.html
    let sticker_token_mint = Pubkey::from_str(WORKSHOP_STICKER_2_MINT).unwrap();
    let initialize_account_instruction = initialize_account(
        &spl_token::ID,
        &new_token_pubkey,
        &sticker_token_mint,
        &my_pubkey,
    )
    .unwrap();

    // Step 4: Fetch a recent blockhash
    let (recent_blockhash, _fee_calculator) = rpc_client
        .get_recent_blockhash()
        .expect("failed to get recent blockhash");

    // Step 5: Create a transaction
    //
    //   Try creating this with `Transaction::new` this time! It will create a signed
    //   transaction, so no need to call `tx.sign(..)`. Remember that a transaction is the
    //   combination of a `Message` and a list of signatures. A `Message` contains a list
    //   of instructions that will be executed in order. If any fail, all changes will be
    //   rolled back.
    //
    //   Hint: Make sure to add `new_token_keypair` as a signer! A signature is always required
    //   when creating new accounts. Otherwise, anyone could create an account for keypairs
    //   they didn't own.
    //
    // Doc hints:
    //  - https://docs.rs/solana-sdk/1.5.8/solana_sdk/transaction/struct.Transaction.html#method.new
    //  - https://docs.rs/solana-sdk/1.5.8/solana_sdk/message/struct.Message.html#method.new
    let tx = Transaction::new(
        &[&my_keypair, &new_token_keypair],
        Message::new(
            &[create_account_instruction, initialize_account_instruction],
            Some(&my_pubkey),
        ),
        recent_blockhash,
    );

    // Step 6: Send transaction
    rpc_client
        .send_and_confirm_transaction_with_spinner(&tx)
        .expect("tx failed");

    // Step 6: Uncomment the println's below and run `cargo run`
    //
    //   This will print the address of your token account. Copy and paste the address into
    //   the workshop block explorer to see if it exists: https://defi-workshop.netlify.app/
    //
    //  - Note: be sure your transaction variable is named `tx`
    //
    println!("Created sticker token account: {}", new_token_pubkey);
    println!("Transaction Signature: {}", utils::tx_signature(&tx));
}
