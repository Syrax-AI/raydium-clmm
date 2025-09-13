use anyhow::Result;
use solana_sdk::signature::Keypair;
use solana_sdk::pubkey::Pubkey;
use std::fs::File;
use std::io::Read;

pub mod instructions;
pub use instructions::utils;

#[derive(Clone, Debug, PartialEq)]
pub struct ClientConfig {
    pub http_url: String,
    pub ws_url: String,
    pub payer_path: String,
    pub admin_path: String,
    pub raydium_v3_program: Pubkey,
    pub slippage: f64,
    pub amm_config_key: Pubkey,
    pub mint0: Option<Pubkey>,
    pub mint1: Option<Pubkey>,
    pub pool_id_account: Option<Pubkey>,
    pub tickarray_bitmap_extension: Option<Pubkey>,
    pub amm_config_index: u16,
}

pub fn read_keypair_file(s: &str) -> Result<Keypair> {
    let mut file = File::open(s)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let keypair_bytes: Vec<u8> = serde_json::from_str(&buf)?;
    let keypair = Keypair::from_bytes(&keypair_bytes)?;
    Ok(keypair)
}
