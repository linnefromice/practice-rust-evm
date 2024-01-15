#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::str::FromStr;
use secp256k1::SecretKey;
use solidity_bindgen::Web3Context;
use web3::ethabi::Address;
mod uniswap_v3_pool {
    use solidity_bindgen::contract_abi;
    pub struct UniswapV3Pool<SolidityBindgenProvider> {
        provider: ::std::sync::Arc<SolidityBindgenProvider>,
        pub address: ::web3::types::Address,
    }
    impl<SolidityBindgenProvider> ::std::clone::Clone
    for UniswapV3Pool<SolidityBindgenProvider> {
        fn clone(&self) -> Self {
            Self {
                provider: ::std::clone::Clone::clone(&self.provider),
                address: self.address,
            }
        }
    }
    impl<SolidityBindgenProvider> UniswapV3Pool<SolidityBindgenProvider> {
        pub fn new<Context>(address: ::web3::types::Address, context: &Context) -> Self
        where
            Context: ::solidity_bindgen::Context<Provider = SolidityBindgenProvider>,
        {
            let abi = "[\n  {\n    \"inputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"int24\",\n        \"name\": \"tickLower\",\n        \"type\": \"int24\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"int24\",\n        \"name\": \"tickUpper\",\n        \"type\": \"int24\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"amount\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount0\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount1\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Burn\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"int24\",\n        \"name\": \"tickLower\",\n        \"type\": \"int24\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"int24\",\n        \"name\": \"tickUpper\",\n        \"type\": \"int24\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"amount0\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"amount1\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"Collect\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"amount0\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"amount1\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"CollectProtocol\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount0\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount1\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"paid0\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"paid1\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Flash\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint16\",\n        \"name\": \"observationCardinalityNextOld\",\n        \"type\": \"uint16\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint16\",\n        \"name\": \"observationCardinalityNextNew\",\n        \"type\": \"uint16\"\n      }\n    ],\n    \"name\": \"IncreaseObservationCardinalityNext\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint160\",\n        \"name\": \"sqrtPriceX96\",\n        \"type\": \"uint160\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"int24\",\n        \"name\": \"tick\",\n        \"type\": \"int24\"\n      }\n    ],\n    \"name\": \"Initialize\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"int24\",\n        \"name\": \"tickLower\",\n        \"type\": \"int24\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"int24\",\n        \"name\": \"tickUpper\",\n        \"type\": \"int24\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"amount\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount0\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"amount1\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Mint\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint8\",\n        \"name\": \"feeProtocol0Old\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint8\",\n        \"name\": \"feeProtocol1Old\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint8\",\n        \"name\": \"feeProtocol0New\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint8\",\n        \"name\": \"feeProtocol1New\",\n        \"type\": \"uint8\"\n      }\n    ],\n    \"name\": \"SetFeeProtocol\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"int256\",\n        \"name\": \"amount0\",\n        \"type\": \"int256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"int256\",\n        \"name\": \"amount1\",\n        \"type\": \"int256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint160\",\n        \"name\": \"sqrtPriceX96\",\n        \"type\": \"uint160\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint128\",\n        \"name\": \"liquidity\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"int24\",\n        \"name\": \"tick\",\n        \"type\": \"int24\"\n      }\n    ],\n    \"name\": \"Swap\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"int24\",\n        \"name\": \"tickLower\",\n        \"type\": \"int24\"\n      },\n      {\n        \"internalType\": \"int24\",\n        \"name\": \"tickUpper\",\n        \"type\": \"int24\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"amount\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"burn\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount0\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount1\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"int24\",\n        \"name\": \"tickLower\",\n        \"type\": \"int24\"\n      },\n      {\n        \"internalType\": \"int24\",\n        \"name\": \"tickUpper\",\n        \"type\": \"int24\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"amount0Requested\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"amount1Requested\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"collect\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"amount0\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"amount1\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"amount0Requested\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"amount1Requested\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"name\": \"collectProtocol\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"amount0\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"amount1\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"factory\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"fee\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint24\",\n        \"name\": \"\",\n        \"type\": \"uint24\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"feeGrowthGlobal0X128\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"feeGrowthGlobal1X128\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount0\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount1\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"flash\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint16\",\n        \"name\": \"observationCardinalityNext\",\n        \"type\": \"uint16\"\n      }\n    ],\n    \"name\": \"increaseObservationCardinalityNext\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint160\",\n        \"name\": \"sqrtPriceX96\",\n        \"type\": \"uint160\"\n      }\n    ],\n    \"name\": \"initialize\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"liquidity\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"maxLiquidityPerTick\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"int24\",\n        \"name\": \"tickLower\",\n        \"type\": \"int24\"\n      },\n      {\n        \"internalType\": \"int24\",\n        \"name\": \"tickUpper\",\n        \"type\": \"int24\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"amount\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"mint\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount0\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount1\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"observations\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint32\",\n        \"name\": \"blockTimestamp\",\n        \"type\": \"uint32\"\n      },\n      {\n        \"internalType\": \"int56\",\n        \"name\": \"tickCumulative\",\n        \"type\": \"int56\"\n      },\n      {\n        \"internalType\": \"uint160\",\n        \"name\": \"secondsPerLiquidityCumulativeX128\",\n        \"type\": \"uint160\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"initialized\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint32[]\",\n        \"name\": \"secondsAgos\",\n        \"type\": \"uint32[]\"\n      }\n    ],\n    \"name\": \"observe\",\n    \"outputs\": [\n      {\n        \"internalType\": \"int56[]\",\n        \"name\": \"tickCumulatives\",\n        \"type\": \"int56[]\"\n      },\n      {\n        \"internalType\": \"uint160[]\",\n        \"name\": \"secondsPerLiquidityCumulativeX128s\",\n        \"type\": \"uint160[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"positions\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"liquidity\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"feeGrowthInside0LastX128\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"feeGrowthInside1LastX128\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"tokensOwed0\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"tokensOwed1\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"protocolFees\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"token0\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"token1\",\n        \"type\": \"uint128\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"feeProtocol0\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"feeProtocol1\",\n        \"type\": \"uint8\"\n      }\n    ],\n    \"name\": \"setFeeProtocol\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"slot0\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint160\",\n        \"name\": \"sqrtPriceX96\",\n        \"type\": \"uint160\"\n      },\n      {\n        \"internalType\": \"int24\",\n        \"name\": \"tick\",\n        \"type\": \"int24\"\n      },\n      {\n        \"internalType\": \"uint16\",\n        \"name\": \"observationIndex\",\n        \"type\": \"uint16\"\n      },\n      {\n        \"internalType\": \"uint16\",\n        \"name\": \"observationCardinality\",\n        \"type\": \"uint16\"\n      },\n      {\n        \"internalType\": \"uint16\",\n        \"name\": \"observationCardinalityNext\",\n        \"type\": \"uint16\"\n      },\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"feeProtocol\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"unlocked\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"int24\",\n        \"name\": \"tickLower\",\n        \"type\": \"int24\"\n      },\n      {\n        \"internalType\": \"int24\",\n        \"name\": \"tickUpper\",\n        \"type\": \"int24\"\n      }\n    ],\n    \"name\": \"snapshotCumulativesInside\",\n    \"outputs\": [\n      {\n        \"internalType\": \"int56\",\n        \"name\": \"tickCumulativeInside\",\n        \"type\": \"int56\"\n      },\n      {\n        \"internalType\": \"uint160\",\n        \"name\": \"secondsPerLiquidityInsideX128\",\n        \"type\": \"uint160\"\n      },\n      {\n        \"internalType\": \"uint32\",\n        \"name\": \"secondsInside\",\n        \"type\": \"uint32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"zeroForOne\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"int256\",\n        \"name\": \"amountSpecified\",\n        \"type\": \"int256\"\n      },\n      {\n        \"internalType\": \"uint160\",\n        \"name\": \"sqrtPriceLimitX96\",\n        \"type\": \"uint160\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"swap\",\n    \"outputs\": [\n      {\n        \"internalType\": \"int256\",\n        \"name\": \"amount0\",\n        \"type\": \"int256\"\n      },\n      {\n        \"internalType\": \"int256\",\n        \"name\": \"amount1\",\n        \"type\": \"int256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"int16\",\n        \"name\": \"\",\n        \"type\": \"int16\"\n      }\n    ],\n    \"name\": \"tickBitmap\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"tickSpacing\",\n    \"outputs\": [\n      {\n        \"internalType\": \"int24\",\n        \"name\": \"\",\n        \"type\": \"int24\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"int24\",\n        \"name\": \"\",\n        \"type\": \"int24\"\n      }\n    ],\n    \"name\": \"ticks\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint128\",\n        \"name\": \"liquidityGross\",\n        \"type\": \"uint128\"\n      },\n      {\n        \"internalType\": \"int128\",\n        \"name\": \"liquidityNet\",\n        \"type\": \"int128\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"feeGrowthOutside0X128\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"feeGrowthOutside1X128\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"int56\",\n        \"name\": \"tickCumulativeOutside\",\n        \"type\": \"int56\"\n      },\n      {\n        \"internalType\": \"uint160\",\n        \"name\": \"secondsPerLiquidityOutsideX128\",\n        \"type\": \"uint160\"\n      },\n      {\n        \"internalType\": \"uint32\",\n        \"name\": \"secondsOutside\",\n        \"type\": \"uint32\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"initialized\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"token0\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"token1\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  }\n]";
            let provider = ::solidity_bindgen::Context::provider(
                context,
                address,
                abi.as_bytes(),
            );
            let provider = ::std::sync::Arc::new(provider);
            Self { address, provider }
        }
    }
    impl<SolidityBindgenProvider> UniswapV3Pool<SolidityBindgenProvider>
    where
        SolidityBindgenProvider: ::solidity_bindgen::SendProvider,
    {
        pub async fn send(
            &self,
            func: &'static str,
            params: impl web3::contract::tokens::Tokenize + Send,
            options: Option<::web3::contract::Options>,
            confirmations: Option<usize>,
        ) -> Result<SolidityBindgenProvider::Out, ::web3::Error> {
            self.provider.send(func, params, options, confirmations).await
        }
        pub async fn initialize(
            &self,
            sqrt_price_x96: u160,
        ) -> ::std::result::Result<SolidityBindgenProvider::Out, ::web3::Error> {
            self.provider.send("initialize", sqrt_price_x96, None, None).await
        }
        pub async fn collect(
            &self,
            recipient: ::web3::types::Address,
            tick_lower: i24,
            tick_upper: i24,
            amount_0_requested: u128,
            amount_1_requested: u128,
        ) -> ::std::result::Result<SolidityBindgenProvider::Out, ::web3::Error> {
            self.provider
                .send(
                    "collect",
                    (
                        recipient,
                        tick_lower,
                        tick_upper,
                        amount_0_requested,
                        amount_1_requested,
                    ),
                    None,
                    None,
                )
                .await
        }
        pub async fn collect_protocol(
            &self,
            recipient: ::web3::types::Address,
            amount_0_requested: u128,
            amount_1_requested: u128,
        ) -> ::std::result::Result<SolidityBindgenProvider::Out, ::web3::Error> {
            self.provider
                .send(
                    "collectProtocol",
                    (recipient, amount_0_requested, amount_1_requested),
                    None,
                    None,
                )
                .await
        }
        pub async fn flash(
            &self,
            recipient: ::web3::types::Address,
            amount_0: ::web3::types::U256,
            amount_1: ::web3::types::U256,
            data: ::std::vec::Vec<u8>,
        ) -> ::std::result::Result<SolidityBindgenProvider::Out, ::web3::Error> {
            self.provider
                .send("flash", (recipient, amount_0, amount_1, data), None, None)
                .await
        }
        pub async fn mint(
            &self,
            recipient: ::web3::types::Address,
            tick_lower: i24,
            tick_upper: i24,
            amount: u128,
            data: ::std::vec::Vec<u8>,
        ) -> ::std::result::Result<SolidityBindgenProvider::Out, ::web3::Error> {
            self.provider
                .send(
                    "mint",
                    (recipient, tick_lower, tick_upper, amount, data),
                    None,
                    None,
                )
                .await
        }
        pub async fn set_fee_protocol(
            &self,
            fee_protocol_0: u8,
            fee_protocol_1: u8,
        ) -> ::std::result::Result<SolidityBindgenProvider::Out, ::web3::Error> {
            self.provider
                .send("setFeeProtocol", (fee_protocol_0, fee_protocol_1), None, None)
                .await
        }
        pub async fn burn(
            &self,
            tick_lower: i24,
            tick_upper: i24,
            amount: u128,
        ) -> ::std::result::Result<SolidityBindgenProvider::Out, ::web3::Error> {
            self.provider
                .send("burn", (tick_lower, tick_upper, amount), None, None)
                .await
        }
        pub async fn increase_observation_cardinality_next(
            &self,
            observation_cardinality_next: u16,
        ) -> ::std::result::Result<SolidityBindgenProvider::Out, ::web3::Error> {
            self.provider
                .send(
                    "increaseObservationCardinalityNext",
                    observation_cardinality_next,
                    None,
                    None,
                )
                .await
        }
        pub async fn swap(
            &self,
            recipient: ::web3::types::Address,
            zero_for_one: bool,
            amount_specified: ::solidity_bindgen::internal::Unimplemented,
            sqrt_price_limit_x96: u160,
            data: ::std::vec::Vec<u8>,
        ) -> ::std::result::Result<SolidityBindgenProvider::Out, ::web3::Error> {
            self.provider
                .send(
                    "swap",
                    (
                        recipient,
                        zero_for_one,
                        amount_specified,
                        sqrt_price_limit_x96,
                        data,
                    ),
                    None,
                    None,
                )
                .await
        }
    }
    impl<SolidityBindgenProvider> UniswapV3Pool<SolidityBindgenProvider>
    where
        SolidityBindgenProvider: ::solidity_bindgen::CallProvider,
    {
        pub async fn ticks(
            &self,
            input_0: i24,
        ) -> ::std::result::Result<
            (u128, i128, ::web3::types::U256, ::web3::types::U256, i56, u160, u32, bool),
            ::web3::Error,
        > {
            self.provider.call("ticks", input_0).await
        }
        pub async fn fee_growth_global_0x128(
            &self,
        ) -> ::std::result::Result<::web3::types::U256, ::web3::Error> {
            self.provider.call("feeGrowthGlobal0X128", ()).await
        }
        pub async fn factory(
            &self,
        ) -> ::std::result::Result<::web3::types::Address, ::web3::Error> {
            self.provider.call("factory", ()).await
        }
        pub async fn token_1(
            &self,
        ) -> ::std::result::Result<::web3::types::Address, ::web3::Error> {
            self.provider.call("token1", ()).await
        }
        pub async fn max_liquidity_per_tick(
            &self,
        ) -> ::std::result::Result<u128, ::web3::Error> {
            self.provider.call("maxLiquidityPerTick", ()).await
        }
        pub async fn slot_0(
            &self,
        ) -> ::std::result::Result<(u160, i24, u16, u16, u16, u8, bool), ::web3::Error> {
            self.provider.call("slot0", ()).await
        }
        pub async fn token_0(
            &self,
        ) -> ::std::result::Result<::web3::types::Address, ::web3::Error> {
            self.provider.call("token0", ()).await
        }
        pub async fn fee(&self) -> ::std::result::Result<u24, ::web3::Error> {
            self.provider.call("fee", ()).await
        }
        pub async fn snapshot_cumulatives_inside(
            &self,
            tick_lower: i24,
            tick_upper: i24,
        ) -> ::std::result::Result<(i56, u160, u32), ::web3::Error> {
            self.provider
                .call("snapshotCumulativesInside", (tick_lower, tick_upper))
                .await
        }
        pub async fn tick_bitmap(
            &self,
            input_0: i16,
        ) -> ::std::result::Result<::web3::types::U256, ::web3::Error> {
            self.provider.call("tickBitmap", input_0).await
        }
        pub async fn fee_growth_global_1x128(
            &self,
        ) -> ::std::result::Result<::web3::types::U256, ::web3::Error> {
            self.provider.call("feeGrowthGlobal1X128", ()).await
        }
        pub async fn protocol_fees(
            &self,
        ) -> ::std::result::Result<(u128, u128), ::web3::Error> {
            self.provider.call("protocolFees", ()).await
        }
        pub async fn observe(
            &self,
            seconds_agos: ::std::vec::Vec<u32>,
        ) -> ::std::result::Result<
            (::std::vec::Vec<i56>, ::std::vec::Vec<u160>),
            ::web3::Error,
        > {
            self.provider.call("observe", seconds_agos).await
        }
        pub async fn tick_spacing(&self) -> ::std::result::Result<i24, ::web3::Error> {
            self.provider.call("tickSpacing", ()).await
        }
        pub async fn liquidity(&self) -> ::std::result::Result<u128, ::web3::Error> {
            self.provider.call("liquidity", ()).await
        }
        pub async fn positions(
            &self,
            input_0: [u8; 32usize],
        ) -> ::std::result::Result<
            (u128, ::web3::types::U256, ::web3::types::U256, u128, u128),
            ::web3::Error,
        > {
            self.provider.call("positions", input_0).await
        }
        pub async fn observations(
            &self,
            input_0: ::web3::types::U256,
        ) -> ::std::result::Result<(u32, i56, u160, bool), ::web3::Error> {
            self.provider.call("observations", input_0).await
        }
    }
}
fn main() {}
