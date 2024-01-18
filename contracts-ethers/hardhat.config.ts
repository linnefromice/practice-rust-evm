import * as dotenv from "dotenv";
import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import { HttpNetworkAccountsUserConfig, NetworksUserConfig } from "hardhat/types";

dotenv.config();

const MNEMONIC = process.env.MNEMONIC || "";
const POLYGON_MUMBAI_RPC =
  process.env.POLYGON_MUMBAI_RPC || "https://polygon-mumbai.blockpi.network/v1/rpc/public";
  const POLYGONSCAN_API_KEY = process.env.POLYGONSCAN_API_KEY || "";

const GWEI = 1000 * 1000 * 1000;

const ACCOUNTS: HttpNetworkAccountsUserConfig = {
  mnemonic: MNEMONIC,
  path: "m/44'/60'/0'/0",
  initialIndex: 0,
  count: 20,
};
const NETWORK_CONFIGS: NetworksUserConfig = {
  mumbai: {
    chainId: 80001,
    url: POLYGON_MUMBAI_RPC,
    gasPrice: 3 * GWEI,
    accounts: ACCOUNTS,
  },
};


const config: HardhatUserConfig = {
  solidity: "0.8.19",
  networks: {
    localhost: {
      url: 'http://127.0.0.1:8545',
    },
    ...NETWORK_CONFIGS,
  },
  etherscan: {
    apiKey: {
      polygonMumbai: POLYGONSCAN_API_KEY,
    },
  },
};

export default config;
