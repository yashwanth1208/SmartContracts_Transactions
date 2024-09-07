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

    //Creating a local wallet
    let wallet: Localwallet = ganche.keys()[0].clone.into();
    let first_address = wallet.address();
    println!(
        "Wallet First Address: {}",first_address.encode_hex::<String>()
    );

    //Provider
    let provider = Provider::try_from(ganache.endpoint())?.interval(Duration::from_millis(10));

    //Query the balance 
    let first_balance = provider.get_balance(first_address,None).await?;
    println!("Wallet First Address Balance: {}", first_balance);

    //Random Acc
    let other_address_hex = "0xaf206dCE72A0ef76643dfeDa34DB764E2126E646";
    let other_address = "0xaf206dCE72A0ef76643dfeDa34DB764E2126E646".parse::<Address>()?;

    let other_balance = provider.get_balance(other_address,None).await?;
    println!(
        "Balance For the Address {}: {}",other_address_hex,other_balance
    );

    
}

