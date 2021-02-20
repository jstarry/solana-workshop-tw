# Create an SPL Token

The goal of this step is to use Rust to send a Solana transaction
which creates an SPL Token account.

"SPL" stands for the "Solana Program Library" which contains a collection
of useful programs that can be interacted with on-chain and composed
together to create more interesting and complex programs.

SPL Links: [[Github]](https://github.com/solana-labs/solana-program-library) [[Docs]](https://spl.solana.com/)

For this workshop, we will be creating SPL Token accounts to receive
new tokens from the "Workshop Token" mint on the Solana Testnet cluster.
SPL Token accounts can only hold tokens from one mint and so on Solana,
users can own many SPL Token accounts. Each of the SPL Token accounts
specifies the address which owns.

## Task 1

In order to create and initialize an SPL Token account, we first need a Solana
account. Solana accounts are used for storing data on-chain and are similar
to files in an operating system. They have the following attributes:

```rust
pub struct Account {
    /// lamports in the account, there are 10^9 lamports in one SOL
    pub lamports: u64,
    /// data held in this account, may be empty
    pub data: Vec<u8>,
    /// the program that owns this account. If executable, the program that loads this account.
    pub owner: Pubkey,
    /// this account's data contains a loaded program (and is now read-only)
    pub executable: bool,
    /// the epoch at which this account will next owe rent
    pub rent_epoch: Epoch,
}
```

#### Terminology
- `lamport`: The smallest fractional unit of a SOL. 1 lamport = 0.000000001 SOL 
- `epoch`: Unit of time roughly equivalent to 3 days
- `rent`: Fees required to store data on the blockchain, collected once per epoch
- `program`: On-chain program which stores executable bytecode in its `data` vector

### Account Requirements

1. Must be owned by the SPL Token program
2. Must have data size equivalent to an SPL Token account
3. Must be "rent-exempt" (explained below)

#### Ownership

All Solana accounts specify an `owner` public key. This `owner` is typically an on-chain
program. The Solana runtime only allows the owner of an account to modify the account
metadata and subtract lamports. Other programs are allowed add lamports and read the metadata.

Solana accounts start with the System Program as their owner. The System Program is allowed
to set a new `owner` and allocate new `data` for accounts owned by the System Program.

In order to initialize an SPL Token account, the SPL Token program needs to modify the account
data so we must "Assign" an account to have the SPL Token program public key as its `owner`.

#### Data / State

Solana accounts can store up to 10MB of data and the owner program has full control over
modifications to this data. Programs typically require account data to have a minimum size
to store the necessary state. The SPL Token program requires token accounts to have a
[precise number of bytes](https://github.com/solana-labs/solana-program-library/blob/25abfe4fbac35e780d34d7a6d955c8fc6b08960a/token/program/src/state.rs#L124).

Note that rent fees increase with the total size of the account metadata.

#### Rent Exemption

Rent fees can be avoided by storing enough lamports in an account to cover 2 years
of rent. Accounts with sufficient rent for 2 years are called rent-exempt accounts.
Rent can be calculated using the [getMinimumBalanceForRentExemption](https://docs.rs/solana-client/1.5.8/solana_client/rpc_client/struct.RpcClient.html#method.get_minimum_balance_for_rent_exemption)
RPC method.
