# Step 2 - Create an SPL Token

The goal of this step is to use Rust to send a Solana transaction
which creates an SPL Token account.

"SPL" stands for the "Solana Program Library" which contains a collection
of useful programs that can be interacted with on-chain and composed
together to create more interesting and complex programs.

SPL Links: [[Github]](https://github.com/solana-labs/solana-program-library) [[Docs]](https://spl.solana.com/)

For this workshop, we will be creating SPL Token accounts to receive
new tokens from the "Workshop Token" mint on the Solana Testnet cluster.
SPL Token accounts can only hold tokens from one mint. On Solana, users
can own many SPL Token accounts. Each SPL Token account specifies the
address of the associated mint and the address of the account that owns it.

## Task 2A - Learn about accounts

In order to create and initialize an SPL Token account, we need to create
a new account on the blockchain. Accounts are used for storing data on the blockchain
and are similar to files in an operating system. They have the following attributes:

```rust
pub struct Account {
    /// lamports in the account, there are 10^9 lamports in one SOL
    pub lamports: u64,
    /// data held in this account, may be empty.
    pub data: Vec<u8>,
    /// the program that owns this account.
    pub owner: Pubkey,
    /// this account can be run as a program
    pub executable: bool,
    /// ...
}
```

#### Terminology
- `lamport`: The smallest fractional unit of a SOL. 1 lamport = 0.000000001 SOL 
- `epoch`: Unit of time roughly equivalent to 3 days
- `rent`: Fees required to store data on the blockchain, collected once per epoch
- `program`: Account that is executable and stores executable bytecode in its `data` vector
- `token`: A unit of currency (BTC, SOL, Sticker, etc)

#### Ownership

All Solana accounts specify an `owner` public key. This `owner` is typically an on-chain
program. The Solana runtime allows programs to modify the metadata and subtract lamports
from the accounts it owns. If a program does not own an account, it is still allowed to
add lamports to the account's balance and read its metadata.

Solana accounts start with the System Program as their owner. The System Program is allowed
to *assign* a new `owner` and *allocate* new `data` for accounts owned by the System Program.

In order to turn a System account into an SPL Token account, we must first *assign* 
it to the SPL Token program which will set its owner to the SPL Token program address.
Once assigned, we can call SPL Token program with this account to initialize it.

**SPL Token account requirement #1**: Must be owned by the SPL Token program

#### Data / State

Solana accounts can store up to 10MB of data and the owner program has full control over
modifications to this data. Programs typically require account data to have a minimum size
to store the necessary state.

The SPL Token program requires token accounts to have a
[precise number of bytes](https://github.com/solana-labs/solana-program-library/blob/25abfe4fbac35e780d34d7a6d955c8fc6b08960a/token/program/src/state.rs#L124).
This means we must *allocate* data for an account before the SPL Token program will
allow it to be initialized as an SPL Token account.

**SPL Token account requirement #2**: Must have data size of 165 bytes

#### Rent

On Solana, rent fees increase with the total size of the account metadata. Users
must pay for any data stored on-chain. Rent is collected every few days and once
an account runs out of lamports, it will be deleted.

Rent fees can be avoided by storing enough lamports in an account to cover 2 years
of rent. Accounts with sufficient rent for 2 years are called rent-exempt accounts.
Rent can be calculated using the [getMinimumBalanceForRentExemption](https://docs.rs/solana-client/1.5.8/solana_client/rpc_client/struct.RpcClient.html#method.get_minimum_balance_for_rent_exemption)
RPC method.

The SPL Token program **requires** token accounts to be rent-exempt to prevent users
from having their tokens accidentally deleted from rent. This means that we must
*transfer* enough lamports to an account before the SPL Token program will
allow it to be initialized.

**SPL Token account requirement #3**: Must be "rent-exempt"

## Task 2B - Code!

Time to start coding! Open the `/step-2/src/main.rs` file which contains
instructions for creating your first SPL Token account

When you're finished, run the following command:

```sh
$ cargo run
```

## Task 2C - Receive sticker token!

Open up the [Workshop Explorer](https://defi-workshop.netlify.app/) and search for your
account's address to check if your token account has been created.

If you're at the in-person workshop, send Justin a link to your account to receive
a Workshop Sticker #2 token :)
