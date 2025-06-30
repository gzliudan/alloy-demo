//! Demonstrates reading a contract by fetching the WETH balance of an address.
use alloy::{primitives::address, providers::ProviderBuilder, sol};
use std::error::Error;

// Generate the contract bindings for the ERC20 interface.
sol! {
   // The `rpc` attribute enables contract interaction via the provider.
   #[sol(rpc)]
   contract ERC20 {
        function balanceOf(address owner) external view returns (uint256);
        function totalSupply() external view returns (uint256);
   }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the provider.
    let provider = ProviderBuilder::new()
        .connect("https://erpc.apothem.network")
        .await?;

    // Instantiate the contract instance.
    let erc20_address = address!("0x99c1919280d80E1A23169fdbd3Cd573E41800cC6");
    let erc20 = ERC20::new(erc20_address, provider);

    let total_supply = erc20.totalSupply().call().await?;
    println!("totalSupply of {erc20_address} = {total_supply}");

    // Fetch the balance of WETH for a given address.
    let owner = address!("0xD4CE02705041F04135f1949Bc835c1Fe0885513c");
    let balance = erc20.balanceOf(owner).call().await?;
    println!("balanceOf   of {owner} = {balance}");

    Ok(())
}
