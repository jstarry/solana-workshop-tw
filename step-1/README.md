# Send a Transaction

The goal of this step is to use Rust to send a Solana transaction
to a Solana cluster.

A Solana transaction is a combination of a "message" and a list of
signatures. A message is an ordered list of instructions which will
each call an on-chain program. Each signature will be used to
verify who authorized the message in this transaction.

#### Terminology
- `transaction`: includes a "message" and list of signatures
- `message`: list of instructions that will be processed sequentially
- `instruction`: describes how to call an on-chain program
- `signature`: a cryptographic proof that the holder of a private key has signed a message

For this workshop, we will be creating a simple transaction with a single
instruction. The instruction will call the SPL Memo on-chain program. The
memo program doesn't actually do anything. Its purpose is to provide a
standard way to attach a text message to a transaction.

## Task 1

On Solana, every transaction must pay fees to be processed. To pay fees,
each transaction specifies an account called the "fee payer" which will
be used to pay transaction fees. Transaction fees are paid using SOL, the
native crypto currency used on Solana. The smallest unit of SOL is called
a "lamport" and there are 10^9 lamports in 1 SOL.

The Solana CLI tools allow developers to manage their accounts. Each account
has a corresponding keypair which can be used to sign messages to create
signatures. Anyone create a new keypair, there are roughly 2^255 valid
keypairs (~10^76) which means it's extremely unlikely for someone to randomly
generate the same keypair. To give perspective, there are roughly 10^80
atoms in the universe.

#### Terminology
- `fee payer`: the account which pays the fees for a transaction
- `keypair`: a combination of a public key and private key
- `lamport`: the smallest unit of SOL, equivalent to 0.000000001 SOL

### Create a keypair

Run the following command to create a new random and unique keypair.
Note that this workshop is only interacting with the testnet so you
do not need to add a passphrase to secure your keypair.

```sh
$ solana-keygen new
```

### Check the balance

Your new keypair is a combination of a public key and private key. The
public key is used as an identifier on the Solana blockchain called
an address. An address is a base-58 encoded string representation of
a 32-byte public key. You can find out the address of your keypair by
running the following command:

```sh
$ solana address
```

You may query the balance of your account by running the following command:

```sh
$ solana balance
```

To do anything interesting, we will need some SOL. The Solana testnet cluster
provides a public service called a faucet which can be used to airdrop
new SOL for free. Run the airdrop command to get some free SOL on testnet:

```sh
$ solana airdrop 1
```

Note: the max size is 10 SOL

#### Terminology
- `airdrop`: a mechanism to receive free tokens
- `faucet`: a service which provides free tokens

Check your balance to ensure you have at least 1 SOL in your account now:

```sh
$ solana balance
```

## Task 2

Time to start coding! Open the `/step-1/src/main.rs` file which contains
instructions for creating your first Solana transaction.

When you're finished, run the following command:

```sh
$ cargo run
```

The `cargo` tool is used for building and running Rust programs and libraries.
It allows Rust packages to specify dependencies. In Step 1 we are depending on
a few Solana packages:

- `solana-cli-config` - Used to read your keypair for signing transactions
- `solana-client` - Used for connecting to the Solana cluster through an RPC HTTP API
- `solana-sdk` - Includes basic types used for creating transactions
