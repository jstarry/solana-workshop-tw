use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
};

// Enter your own order id to receive a prize!
const EVENTBRITE_ORDER_ID: u64 = 0;

entrypoint!(process_instruction);
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Sticker Redeemer!");
    msg!("Eventbrite order id: {}", EVENTBRITE_ORDER_ID);

    // Step 1: Extract account inputs
    //
    //   Expected account inputs:
    //    1. My sticker token address
    //    2. Sticker mint address
    //    3. My pubkey
    //
    // Doc hints:
    //  - https://docs.rs/solana-program/1.5.8/solana_program/account_info/fn.next_account_info.html
    let account_info_iter = &mut accounts.iter();

    // Step 2: Create "Burn" instruction for SPL Token program
    //
    //  Tips:
    //   - This instruction should burn exactly one token.
    //   - Pass `&[]` as `signer_keys`, `signer_keys` are not important here
    //   - `authority_pubkey` is the owner of the token account
    //
    // Doc hints:
    //  - https://docs.rs/spl-token/3.1.0/spl_token/instruction/fn.burn.html
    //  - https://docs.rs/solana-program/1.5.8/solana_program/account_info/struct.AccountInfo.html

    msg!("Redeeming sticker...");

    // Step 3: Invoke the "Burn" instruction for SPL Token program
    //
    //  Tips:
    //   - Each account info must be cloned
    //
    // Doc hints:
    //  - https://docs.rs/solana-program/1.5.8/solana_program/program/fn.invoke.html

    // Step 4: Create "Close Account" instruction for SPL Token program
    //
    //  Now that the token has been burned, we don't need the token account anymore.
    //  The remaining lamports can be reclaimed by closing the account.
    //
    //  Tips:
    //   - `account_pubkey` is the token account to be closed
    //   - `destination_pubkey` is the account that will receive the token account's lamports
    //   - `owner_pubkey` is the owner of the token account
    //
    // Doc hints:
    //  - https://docs.rs/spl-token/3.1.0/spl_token/instruction/fn.close_account.html

    msg!("Closing sticker account...");

    // Step 5: Invoke the "Close Account" instruction for SPL Token program
    //
    //  Tips:
    //   - Each account info must be cloned
    //
    // Doc hints:
    //  - https://docs.rs/solana-program/1.5.8/solana_program/program/fn.invoke.html

    msg!("Redeem succeeded!");
    Ok(())
}
