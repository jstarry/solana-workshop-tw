use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{read_keypair_file, Keypair},
    transaction::Transaction,
};

pub fn tx_signature(tx: &Transaction) -> String {
    tx.signatures
        .first()
        .expect("transaction not signed")
        .to_string()
}

pub fn load_config_keypair() -> Keypair {
    let config_path = solana_cli_config::CONFIG_FILE.as_ref().unwrap();
    let cli_config =
        solana_cli_config::Config::load(config_path).expect("failed to load config file");
    read_keypair_file(cli_config.keypair_path).expect("failed to load keypair")
}

pub fn new_rpc_client() -> RpcClient {
    let config_path = solana_cli_config::CONFIG_FILE.as_ref().unwrap();
    let cli_config =
        solana_cli_config::Config::load(config_path).expect("failed to load config file");
    RpcClient::new_with_commitment(cli_config.json_rpc_url, CommitmentConfig::confirmed())
}
