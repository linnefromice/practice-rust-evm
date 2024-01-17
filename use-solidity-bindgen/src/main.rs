use std::str::FromStr;

use secp256k1::SecretKey;
use solidity_bindgen::Web3Context;
use web3::ethabi::Address;

// mod erc20;
// mod uniswap;
// mod lyra;
mod chainlink;
mod local;

const ETHEREUM_URL: &str = "https://eth.llamarpc.com";
const OPTIMISM_URL: &str = "https://optimism-mainnet.public.blastapi.io";
const HARDHAT_URL: &str = "http://localhost:8545";

const DAI: &str = "0x6b175474e89094c44da98b954eedeac495271d0f";
const AGGREGATOR_ETHUSD: &str = "0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419";
const OPEN_MARKET_VIEWER_IN_OP: &str = "0x136d92f1d103BA5267c85555b28787AE53Ee3CEF";

#[tokio::main]
async fn main() {
    let ctx = Web3Context::new(
        HARDHAT_URL,
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
    // let address = Address::from_str(OPEN_MARKET_VIEWER_IN_OP).unwrap();
    // let contract = lyra::OpenMarketViewer_remove_error::new(address, &ctx);
    // println!("{:#x}", contract.owner().await.unwrap());
    // let open_market = Address::from_str("0x59c671B1a1F261FB2192974B43ce1608aeFd328E").unwrap();
    // let _ = contract.get_market(open_market).await.unwrap();

    // For Aggregator
    // let address = Address::from_str(AGGREGATOR_ETHUSD).unwrap();
    // let contract = chainlink::EACAggregatorProxy::new(address, &ctx);
    // println!("{:?}", contract.latest_answer().await.unwrap());

    // For Viewer
    let address = Address::from_str("0x5fbdb2315678afecb367f032d93f642f64180aa3").unwrap();
    let contract = local::Viewer::new(address, &ctx);
    println!("{:?}", contract.name().await.unwrap());
    println!("{:?}", contract.version().await.unwrap());
    println!("{:?}", contract.get_i_256_min().await.unwrap());
    println!("{:?}", contract.get_i_256_max().await.unwrap());
    println!("{:?}", contract.get_multiple_i_256().await.unwrap());
    println!("{:?}", contract.get_multiple_i_256_plus().await.unwrap());
    println!("{:?}", contract.get_multiple_i_256_minus().await.unwrap());
}

// fn main() {
//     // from rust-web3
//     let web3 = web3::Web3::new(web3::transports::Http::new(ETHEREUM_URL).unwrap());
//     let contract = web3::contract::Contract::from_json(
//         web3.eth(),
//         Address::from_str(DAI).unwrap(),
//         include_bytes!("../resources/ERC20.json"),
//     ).unwrap();

//     // from ethabi
//     let contract: web3::ethabi::Contract = serde_json::from_str(include_str!("../resources/EACAggregatorProxy.json")).unwrap();
//     let func = contract.function("getRoundData").unwrap();
//     println!("{:?}", func.name);
//     println!("{:?}", func.inputs);
//     println!("{:?}", func.outputs);
// }