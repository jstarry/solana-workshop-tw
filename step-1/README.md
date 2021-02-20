# Send a Transaction

The goal of this step is to use Rust to send a Solana transaction
to a Solana cluster.

A Solana transaction is a list of instructions and signatures. Each
instruction will call an on-chain program and each signature will be
used to verify who authorized this transaction.

For this workshop, we will be creating a simple transaction with a single
instruction. The instruction will call the SPL Memo on-chain program. The
memo program doesn't actually do anything. Its purpose is to provide a
standard way to attach a text message to a transaction.

## Task 1

On Solana, every transaction must pay fees to be processed. Every
transaction specifies an account called the "fee payer" which will
be used to pay transaction fees.
