use reqwest;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    result: TxResult,
}

#[derive(Debug, Serialize, Deserialize)]
struct TxResult {
    hash: String,
    height: String,
    proof: Proof,
}

#[derive(Debug, Serialize, Deserialize)]
struct Proof {
    data: Vec<String>,
    share_proofs: Vec<ShareProof>,
    namespace_id: String,
    row_proof: RowProof,
    namespace_version: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ShareProof {
    end: i32,
    nodes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RowProof {
    row_roots: Vec<String>,
    proofs: Vec<ProofDetail>,
    start_row: i32,
    end_row: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProofDetail {
    total: String,
    index: String,
    leaf_hash: String,
    aunts: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let hash = &args[1];
    let url = format!("https://rpc.celestia-mocha.com/tx?hash=0x{}&prove=true", hash);
    println!("URL: {}", url);

    let response = reqwest::get(url).await?;
    let data: Response = response.json().await?;

    println!("Hash: {}", data.result.hash);
    println!("Height: {}", data.result.height);
    println!("Proof: {:?}", data.result.proof);

    Ok(())
}

