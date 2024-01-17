import hre, { viem } from "hardhat";

async function main() {
  console.log(`Call Viewer contract...`)
  // const rpcUrl = "http://localhost:8545";
  // const publicClient = await hre.viem.getTestClient();

  const contract = await hre.viem.getContractAt("Viewer", "0x5FbDB2315678afecb367f032d93F642f64180aa3");
  console.log(`contract address: ${contract.address}`)
  console.log(`i256_max: ${await contract.read.get_i256_max()}`)
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
