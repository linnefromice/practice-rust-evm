#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::str::FromStr;
use secp256k1::SecretKey;
use solidity_bindgen::Web3Context;
use web3::ethabi::Address;
mod chainlink {
    use solidity_bindgen::contract_abi;
    pub struct EACAggregatorProxy<SolidityBindgenProvider> {
        provider: ::std::sync::Arc<SolidityBindgenProvider>,
        pub address: ::web3::types::Address,
    }
    impl<SolidityBindgenProvider> ::std::clone::Clone
    for EACAggregatorProxy<SolidityBindgenProvider> {
        fn clone(&self) -> Self {
            Self {
                provider: ::std::clone::Clone::clone(&self.provider),
                address: self.address,
            }
        }
    }
    impl<SolidityBindgenProvider> EACAggregatorProxy<SolidityBindgenProvider> {
        pub fn new<Context>(address: ::web3::types::Address, context: &Context) -> Self
        where
            Context: ::solidity_bindgen::Context<Provider = SolidityBindgenProvider>,
        {
            let abi = "[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_aggregator\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_accessController\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"int256\",\n        \"name\": \"current\",\n        \"type\": \"int256\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint256\",\n        \"name\": \"roundId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"updatedAt\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"AnswerUpdated\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint256\",\n        \"name\": \"roundId\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"startedBy\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"startedAt\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"NewRound\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"OwnershipTransferRequested\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"OwnershipTransferred\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"acceptOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"accessController\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract AccessControllerInterface\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"aggregator\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_aggregator\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"confirmAggregator\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"decimals\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"\",\n        \"type\": \"uint8\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"description\",\n    \"outputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"\",\n        \"type\": \"string\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_roundId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getAnswer\",\n    \"outputs\": [\n      {\n        \"internalType\": \"int256\",\n        \"name\": \"\",\n        \"type\": \"int256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint80\",\n        \"name\": \"_roundId\",\n        \"type\": \"uint80\"\n      }\n    ],\n    \"name\": \"getRoundData\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint80\",\n        \"name\": \"roundId\",\n        \"type\": \"uint80\"\n      },\n      {\n        \"internalType\": \"int256\",\n        \"name\": \"answer\",\n        \"type\": \"int256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"startedAt\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"updatedAt\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint80\",\n        \"name\": \"answeredInRound\",\n        \"type\": \"uint80\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_roundId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getTimestamp\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"latestAnswer\",\n    \"outputs\": [\n      {\n        \"internalType\": \"int256\",\n        \"name\": \"\",\n        \"type\": \"int256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"latestRound\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"latestRoundData\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint80\",\n        \"name\": \"roundId\",\n        \"type\": \"uint80\"\n      },\n      {\n        \"internalType\": \"int256\",\n        \"name\": \"answer\",\n        \"type\": \"int256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"startedAt\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"updatedAt\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint80\",\n        \"name\": \"answeredInRound\",\n        \"type\": \"uint80\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"latestTimestamp\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"owner\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint16\",\n        \"name\": \"\",\n        \"type\": \"uint16\"\n      }\n    ],\n    \"name\": \"phaseAggregators\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract AggregatorV2V3Interface\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"phaseId\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint16\",\n        \"name\": \"\",\n        \"type\": \"uint16\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_aggregator\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"proposeAggregator\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"proposedAggregator\",\n    \"outputs\": [\n      {\n        \"internalType\": \"contract AggregatorV2V3Interface\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint80\",\n        \"name\": \"_roundId\",\n        \"type\": \"uint80\"\n      }\n    ],\n    \"name\": \"proposedGetRoundData\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint80\",\n        \"name\": \"roundId\",\n        \"type\": \"uint80\"\n      },\n      {\n        \"internalType\": \"int256\",\n        \"name\": \"answer\",\n        \"type\": \"int256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"startedAt\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"updatedAt\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint80\",\n        \"name\": \"answeredInRound\",\n        \"type\": \"uint80\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"proposedLatestRoundData\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint80\",\n        \"name\": \"roundId\",\n        \"type\": \"uint80\"\n      },\n      {\n        \"internalType\": \"int256\",\n        \"name\": \"answer\",\n        \"type\": \"int256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"startedAt\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"updatedAt\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint80\",\n        \"name\": \"answeredInRound\",\n        \"type\": \"uint80\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_accessController\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"setController\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"_to\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"transferOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"version\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  }\n]";
            let provider = ::solidity_bindgen::Context::provider(
                context,
                address,
                abi.as_bytes(),
            );
            let provider = ::std::sync::Arc::new(provider);
            Self { address, provider }
        }
    }
    impl<SolidityBindgenProvider> EACAggregatorProxy<SolidityBindgenProvider>
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
        pub async fn propose_aggregator(
            &self,
            aggregator: ::web3::types::Address,
        ) -> ::std::result::Result<SolidityBindgenProvider::Out, ::web3::Error> {
            self.provider.send("proposeAggregator", aggregator, None, None).await
        }
        pub async fn confirm_aggregator(
            &self,
            aggregator: ::web3::types::Address,
        ) -> ::std::result::Result<SolidityBindgenProvider::Out, ::web3::Error> {
            self.provider.send("confirmAggregator", aggregator, None, None).await
        }
        pub async fn accept_ownership(
            &self,
        ) -> ::std::result::Result<SolidityBindgenProvider::Out, ::web3::Error> {
            self.provider.send("acceptOwnership", (), None, None).await
        }
        pub async fn set_controller(
            &self,
            access_controller: ::web3::types::Address,
        ) -> ::std::result::Result<SolidityBindgenProvider::Out, ::web3::Error> {
            self.provider.send("setController", access_controller, None, None).await
        }
        pub async fn transfer_ownership(
            &self,
            to: ::web3::types::Address,
        ) -> ::std::result::Result<SolidityBindgenProvider::Out, ::web3::Error> {
            self.provider.send("transferOwnership", to, None, None).await
        }
    }
    impl<SolidityBindgenProvider> EACAggregatorProxy<SolidityBindgenProvider>
    where
        SolidityBindgenProvider: ::solidity_bindgen::CallProvider,
    {
        pub async fn access_controller(
            &self,
        ) -> ::std::result::Result<::web3::types::Address, ::web3::Error> {
            self.provider.call("accessController", ()).await
        }
        pub async fn latest_round_data(
            &self,
        ) -> ::std::result::Result<
            (
                u80,
                ::solidity_bindgen::internal::Unimplemented,
                ::web3::types::U256,
                ::web3::types::U256,
                u80,
            ),
            ::web3::Error,
        > {
            self.provider.call("latestRoundData", ()).await
        }
        pub async fn latest_timestamp(
            &self,
        ) -> ::std::result::Result<::web3::types::U256, ::web3::Error> {
            self.provider.call("latestTimestamp", ()).await
        }
        pub async fn phase_aggregators(
            &self,
            input_0: u16,
        ) -> ::std::result::Result<::web3::types::Address, ::web3::Error> {
            self.provider.call("phaseAggregators", input_0).await
        }
        pub async fn phase_id(&self) -> ::std::result::Result<u16, ::web3::Error> {
            self.provider.call("phaseId", ()).await
        }
        pub async fn proposed_latest_round_data(
            &self,
        ) -> ::std::result::Result<
            (
                u80,
                ::solidity_bindgen::internal::Unimplemented,
                ::web3::types::U256,
                ::web3::types::U256,
                u80,
            ),
            ::web3::Error,
        > {
            self.provider.call("proposedLatestRoundData", ()).await
        }
        pub async fn latest_answer(
            &self,
        ) -> ::std::result::Result<
            ::solidity_bindgen::internal::Unimplemented,
            ::web3::Error,
        > {
            self.provider.call("latestAnswer", ()).await
        }
        pub async fn get_answer(
            &self,
            round_id: ::web3::types::U256,
        ) -> ::std::result::Result<
            ::solidity_bindgen::internal::Unimplemented,
            ::web3::Error,
        > {
            self.provider.call("getAnswer", round_id).await
        }
        pub async fn get_timestamp(
            &self,
            round_id: ::web3::types::U256,
        ) -> ::std::result::Result<::web3::types::U256, ::web3::Error> {
            self.provider.call("getTimestamp", round_id).await
        }
        pub async fn version(
            &self,
        ) -> ::std::result::Result<::web3::types::U256, ::web3::Error> {
            self.provider.call("version", ()).await
        }
        pub async fn owner(
            &self,
        ) -> ::std::result::Result<::web3::types::Address, ::web3::Error> {
            self.provider.call("owner", ()).await
        }
        pub async fn proposed_aggregator(
            &self,
        ) -> ::std::result::Result<::web3::types::Address, ::web3::Error> {
            self.provider.call("proposedAggregator", ()).await
        }
        pub async fn proposed_get_round_data(
            &self,
            round_id: u80,
        ) -> ::std::result::Result<
            (
                u80,
                ::solidity_bindgen::internal::Unimplemented,
                ::web3::types::U256,
                ::web3::types::U256,
                u80,
            ),
            ::web3::Error,
        > {
            self.provider.call("proposedGetRoundData", round_id).await
        }
        pub async fn decimals(&self) -> ::std::result::Result<u8, ::web3::Error> {
            self.provider.call("decimals", ()).await
        }
        pub async fn latest_round(
            &self,
        ) -> ::std::result::Result<::web3::types::U256, ::web3::Error> {
            self.provider.call("latestRound", ()).await
        }
        pub async fn description(
            &self,
        ) -> ::std::result::Result<::std::string::String, ::web3::Error> {
            self.provider.call("description", ()).await
        }
        pub async fn aggregator(
            &self,
        ) -> ::std::result::Result<::web3::types::Address, ::web3::Error> {
            self.provider.call("aggregator", ()).await
        }
        pub async fn get_round_data(
            &self,
            round_id: u80,
        ) -> ::std::result::Result<
            (
                u80,
                ::solidity_bindgen::internal::Unimplemented,
                ::web3::types::U256,
                ::web3::types::U256,
                u80,
            ),
            ::web3::Error,
        > {
            self.provider.call("getRoundData", round_id).await
        }
    }
}
fn main() {}
