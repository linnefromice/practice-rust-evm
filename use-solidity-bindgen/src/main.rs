use std::str::FromStr;

use secp256k1::SecretKey;
use solidity_bindgen::Web3Context;
use web3::ethabi::Address;

mod erc20;
// mod uniswap;
mod lyra;

const ETHEREUM_URL: &str = "https://eth.llamarpc.com";
const OPTIMISM_URL: &str = "https://optimism-mainnet.public.blastapi.io";

const DAI: &str = "0x6b175474e89094c44da98b954eedeac495271d0f";
const OPEN_MARKET_VIEWER_IN_OP: &str = "0x136d92f1d103BA5267c85555b28787AE53Ee3CEF";

#[tokio::main]
async fn main() {
    let ctx = Web3Context::new(
        OPTIMISM_URL,
        Address::zero(),
        &SecretKey::from_slice(&[1; 32]).unwrap()
    ).unwrap();
    
    // For ERC20
    // let address = Address::from_str(DAI).unwrap();
    // let contract = ERC20::new(address, &ctx);
    // println!("{}", contract.name().await.unwrap());
    // println!("{}", contract.symbol().await.unwrap());
    // println!("{}", contract.decimals().await.unwrap());
    // println!("{}", contract.total_supply().await.unwrap());

    // For lyra
    let address = Address::from_str(OPEN_MARKET_VIEWER_IN_OP).unwrap();
    let contract = lyra::OpenMarketViewer_remove_error::new(address, &ctx);
    println!("{:#x}", contract.owner().await.unwrap());
    let open_market = Address::from_str("0x59c671B1a1F261FB2192974B43ce1608aeFd328E").unwrap();
    let _ = contract.get_market(open_market).await.unwrap();
}
