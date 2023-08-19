// source: https://github.com/sogolmalek/EIP-x/blob/main/Client-Rust

use std::{path::PathBuf, str::FromStr};

use env_logger::Env;
use ethers::{types::Address, Block, BlockTag, utils};
use dirs::home_dir;
use eyre::Result;
use helios::{
    client::TemporaryDB,
    config::networks::Network,
    prelude::*
};

// use helios::db::{Database, InMemoryDatabase};
use std::collections::HashMap;
// use bls12_381::{PublicKey, Signature, G1Affine, Fq12, Fq6, G2Affine, G2Projective};
use std::convert::TryFrom;

// // Define a function to verify BLS signature
// fn verify_signature(
//   public_key: &PublicKey<G2Affine>,
//   message: &[u8],
//   signature: &Signature<G1Affine>,
// ) -> bool {
//   // Verify the BLS signature
//   public_key.verify::<Fq6, Fq12>(&message, signature)
// }

// Define a cache struct to hold the cached data
struct Cache {
    cache: HashMap<Address, f64>,
}

impl Cache {
    fn new() -> Self {
        Cache {
            cache: HashMap::new(),
        }
    }

    fn get(&self, address: &Address) -> Option<f64> {
        self.cache.get(address).cloned()
    }

    fn insert(&mut self, address: Address, balance: f64) {
        self.cache.insert(address, balance);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let untrusted_execution_rpc_url = std::env::var("MAINNET_EXECUTION_RPC")?;
    log::info!("Using untrusted RPC URL [REDACTED]");

    let consensus_rpc_url = std::env::var("MAINNET_CONSENSUS_RPC")?;
    log::info!("Using consensus RPC URL: {}", consensus_rpc_url);

    let mainnet_data_dir_ext = std::env::var("MAINNET_DATA_DIR_EXT")?;
    let data_path = home_dir().unwrap().join(mainnet_data_dir_ext);

    let mut client: Client = ClientBuilder::new()
        .network(Network::MAINNET)
        .consensus_rpc(&consensus_rpc_url)
        .execution_rpc(&untrusted_execution_rpc_url)
        .load_external_fallback()
        // .data_dir(PathBuf::from(data_path))
        .database(TemporaryDB::new())
        .build()?;

    log::info!(
        "Built client on network \"{}\" with external checkpoint fallbacks",
        Network::MAINNET
    );

    client.start().await?;

    let head_block_num = client.get_block_number().await?;
    let addr = Address::from_str("0x00000000219ab540356cBB839Cbe05303d7705Fa")?;
    let block = BlockTag::Latest;
    let balance = client.get_balance(&addr, block).await?;

    log::info!("synced up to block: {}", head_block_num);
    log::info!(
        "balance of deposit contract: {}",
        utils::format_ether(balance)
    );

    // Query the latest block
    let latest_block: Option<Block> = client.get_block(BlockTag::Latest).await?;
    if let Some(block) = latest_block {
        log::info!("Latest block details: {:?}", block);
    } else {
        log::info!("Latest block not found");
    }

    // Initialize the cache
    let mut cache = Cache::new();

    // let database = TemporaryDB::new();

    // TODO - the below is yet to be implemented.
    // it would require Helios to have an in-memory database

    // // Initialize the in-memory database
    // let database = InMemoryDatabase::new();

    // // Example Ethereum address to query
    // let address = Address::from_str("0x0000000000000000000000000000000000000000")
    //     .expect("Failed to parse address");

    // // Query the balance and store it in the partial view data
    // query_balance(&client, &database, &mut cache, address).await?;

    // // Get the balance from the partial view data
    // let balance = get_balance(&database, address);
    // println!("Balance of {}: {} ETH", address, balance);

    Ok(())
}


// async fn query_balance<D: Database>(
//   client: &helios::Client<D>,
//   database: &D,
//   cache: &mut Cache,
//   address: Address,
// ) -> Result<()> {
//   // Check if the balance is already in the cache
//   if let Some(balance) = cache.get(&address) {
//       // If in cache, no need to query, just store it in the database
//       database.save_checkpoint(address.to_string(), balance.to_string())?;
//       return Ok(());
//   }

//   // Fetch the balance from the Helios client
//   let balance = client
//       .eth_balance(address, None)
//       .await?
//       .as_u64() as f64 / 1_000_000_000_000_000_000.0;

//   // Insert the balance into the cache
//   cache.insert(address, balance);

//   // Store the balance in the partial view data (in-memory database)
//   database.save_checkpoint(address.to_string(), balance.to_string())?;

//   Ok(())
// }

// fn get_balance<D: Database>(database: &D, address: Address) -> f64 {
//   // Try to get the balance from the cache first
//   if let Some(balance) = database.load_checkpoint(address.to_string()) {
//       return balance.parse().unwrap_or(0.0);
//   }

//   // If not in cache, return 0.0 as default
//   0.0
// }
