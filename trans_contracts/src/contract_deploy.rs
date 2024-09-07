use std::time::Duration;

use ethers::{
    prelude::{Address, Localwallet, Middleware, Provider, Signer, TransaxtionRequest, U256},
    utils::Ganache,
};
use eyre::{ContextCompat, Result};
use hex::ToHex;

#[tokio::main]
async fn main() -> Result<()>{
    let mnemonic = "gas monster ski craft below illegal discover limit dog bundle bus artefact";
    //Spawning a ganache instance with menomic and printing the ganache endpoint
    let ganache = Ganache::new().menomic(mnemonic).spawn();
    println!("HTTP EndPoint: {}",ganache.endpoint());

    let wallet: Localwallet = ganche.keys()[0].clone.into();
    let first_address = wallet.
}

