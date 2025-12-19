## jito-rs

This repository contains code to communicate with Jito's Block-Engine via gRPC, allowing for bundle submissions and status subscriptions.

Jito's block engine allows gRPC connections with whitelisted keypairs authentication (for increased rate limits) or without authentication (at the default rate limit of 1 request per second).

## Usage

```rust
use jito_rs::searcher_client::{get_searcher_client_auth, get_searcher_client_no_auth};
use solana_sdk::signature::Keypair;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

// Setup
let exit = Arc::new(AtomicBool::new(false));
let block_engine_url = "https://ny.mainnet.block-engine.jito.wtf";
let json_rpc_url = "https://api.mainnet-beta.solana.com";

// With authentication
let auth_keypair = Keypair::from_base58_string("");
let auth_keypair = Arc::new(auth_keypair);

let (searcher_client, cluster_data_impl) = get_searcher_client_auth(
    &auth_keypair,
    &exit,
    block_engine_url,
    json_rpc_url,
).await?;

// Without authentication (default rate limit: 1 request per second)
let (searcher_client, cluster_data_impl) = get_searcher_client_no_auth(
    &exit,
    block_engine_url,
    json_rpc_url,
).await?;

// Send a bundle and receive its bundle id. transaction is a VersionedTransaction.
use solana_sdk::transaction::VersionedTransaction;
let bundle_id = searcher_client.send_bundle(vec![transaction]).await.unwrap();
println!("{:?}", response);

// Subscribe to bundle results
let receiver = searcher_client.subscribe_bundle_results(100).await.unwrap();
while let Some(result) = receiver.recv().await {
    println!("{:?}", result);
}
```
