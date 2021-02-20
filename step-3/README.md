# Step 3 - Deploy an on-chain program

The goal of this step is to use Rust to write and deploy
a program on Solana that allows users to redeem stickers.

### Context

On Solana, an on-chain program is an executable that can be called
to change account state on the blockchain. Programs are always called
with a list of accounts and raw instruction data. It's up to each
program to decide which accounts are required, whether instruction data
is valid or not, etc.

Typically developers write programs using Rust and use the Solana tools
to compile their program and deploy it to the blockchain at an address
called the "program id". Once deployed, anyone can call your program by
creating an instruction that includes your program id. If you made a
mistake or wish to change behavior, you may redeploy your program to the
program id.

### Program entrypoint

Each program has a single entrypoint which can return success or failure.
The entrypoint includes `program_id` which is useful for checking if an
account is assigned to your program (meaning you can write to it!). It also
includes a list of `AccountInfo` which will provide each account input's
public key, account metadata, and whether or not the account signed the
instruction. Lastly, it includes raw instruction data which can be handled
however the program would like.

```rust
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult { .. }
```

### Sticker Redeemer

For this task, you will learn how to create a program which redeems
sticker tokens. To redeem a sticker token, it must be destroyed by sending
a "Burn" instruction to the SPL Token program.

## Task 3A - Code your program!

Time to start coding! Open the `/step-3/program/src/lib.rs` file which contains
instructions for writing your first Solana program.

When you're finished, run the following command:

```sh
$ cargo build-bpf
```

## Task 3B - Deploy your program!

After building your program, you can deploy it to the blockchain with
the following command:

```sh
$ solana program deploy ./target/deploy/solana_workshop_tw_step_3.so
```

## Task 3C - Call your program!

Keep on coding! Open the `/step-3/client/src/main.rs` file which contains
instructions for calling your Solana program.

When you're finished, run the following command:

```sh
$ cargo run
```
