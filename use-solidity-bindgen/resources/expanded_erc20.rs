#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::str::FromStr;
use erc20::ERC20;
use secp256k1::SecretKey;
use solidity_bindgen::Web3Context;
use web3::ethabi::Address;
mod erc20 {
    use solidity_bindgen::contract_abi;
    pub struct ERC20<SolidityBindgenProvider> {
        provider: ::std::sync::Arc<SolidityBindgenProvider>,
        pub address: ::web3::types::Address,
    }
    impl<SolidityBindgenProvider> ::std::clone::Clone
    for ERC20<SolidityBindgenProvider> {
        fn clone(&self) -> Self {
            Self {
                provider: ::std::clone::Clone::clone(&self.provider),
                address: self.address,
            }
        }
    }
    impl<SolidityBindgenProvider> ERC20<SolidityBindgenProvider> {
        pub fn new<Context>(address: ::web3::types::Address, context: &Context) -> Self
        where
            Context: ::solidity_bindgen::Context<Provider = SolidityBindgenProvider>,
        {
            let abi = "[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"name_\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"string\",\n        \"name\": \"symbol_\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"decimals_\",\n        \"type\": \"uint8\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"initialBalance_\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"feeReceiver_\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Approval\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Transfer\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"allowance\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"approve\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"balanceOf\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"decimals\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"\",\n        \"type\": \"uint8\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"subtractedValue\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"decreaseAllowance\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"spender\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"addedValue\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"increaseAllowance\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"name\",\n    \"outputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"\",\n        \"type\": \"string\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"symbol\",\n    \"outputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"\",\n        \"type\": \"string\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"totalSupply\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"transfer\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"transferFrom\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n";
            let provider = ::solidity_bindgen::Context::provider(
                context,
                address,
                abi.as_bytes(),
            );
            let provider = ::std::sync::Arc::new(provider);
            Self { address, provider }
        }
    }
    impl<SolidityBindgenProvider> ERC20<SolidityBindgenProvider>
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
        pub async fn transfer_from(
            &self,
            sender: ::web3::types::Address,
            recipient: ::web3::types::Address,
            amount: ::web3::types::U256,
        ) -> ::std::result::Result<SolidityBindgenProvider::Out, ::web3::Error> {
            self.provider
                .send("transferFrom", (sender, recipient, amount), None, None)
                .await
        }
        pub async fn decrease_allowance(
            &self,
            spender: ::web3::types::Address,
            subtracted_value: ::web3::types::U256,
        ) -> ::std::result::Result<SolidityBindgenProvider::Out, ::web3::Error> {
            self.provider
                .send("decreaseAllowance", (spender, subtracted_value), None, None)
                .await
        }
        pub async fn transfer(
            &self,
            recipient: ::web3::types::Address,
            amount: ::web3::types::U256,
        ) -> ::std::result::Result<SolidityBindgenProvider::Out, ::web3::Error> {
            self.provider.send("transfer", (recipient, amount), None, None).await
        }
        pub async fn increase_allowance(
            &self,
            spender: ::web3::types::Address,
            added_value: ::web3::types::U256,
        ) -> ::std::result::Result<SolidityBindgenProvider::Out, ::web3::Error> {
            self.provider
                .send("increaseAllowance", (spender, added_value), None, None)
                .await
        }
        pub async fn approve(
            &self,
            spender: ::web3::types::Address,
            amount: ::web3::types::U256,
        ) -> ::std::result::Result<SolidityBindgenProvider::Out, ::web3::Error> {
            self.provider.send("approve", (spender, amount), None, None).await
        }
    }
    impl<SolidityBindgenProvider> ERC20<SolidityBindgenProvider>
    where
        SolidityBindgenProvider: ::solidity_bindgen::CallProvider,
    {
        pub async fn name(
            &self,
        ) -> ::std::result::Result<::std::string::String, ::web3::Error> {
            self.provider.call("name", ()).await
        }
        pub async fn decimals(&self) -> ::std::result::Result<u8, ::web3::Error> {
            self.provider.call("decimals", ()).await
        }
        pub async fn symbol(
            &self,
        ) -> ::std::result::Result<::std::string::String, ::web3::Error> {
            self.provider.call("symbol", ()).await
        }
        pub async fn total_supply(
            &self,
        ) -> ::std::result::Result<::web3::types::U256, ::web3::Error> {
            self.provider.call("totalSupply", ()).await
        }
        pub async fn allowance(
            &self,
            owner: ::web3::types::Address,
            spender: ::web3::types::Address,
        ) -> ::std::result::Result<::web3::types::U256, ::web3::Error> {
            self.provider.call("allowance", (owner, spender)).await
        }
        pub async fn balance_of(
            &self,
            account: ::web3::types::Address,
        ) -> ::std::result::Result<::web3::types::U256, ::web3::Error> {
            self.provider.call("balanceOf", account).await
        }
    }
}
fn main() {}
