import hre from "hardhat";

async function main() {
  console.log(`Call Viewer contract...`)
  const { ethers } = hre;
  const rpcUrl = "http://localhost:8545";

  const contract = await ethers.getContractAt("Viewer", "0x5FbDB2315678afecb367f032d93F642f64180aa3");
  console.log(`contract address: ${await contract.getAddress()}`)
  console.log(`name: ${await contract.name()}`)
  console.log(`version: ${await contract.version()}`)
  console.log(`i256_max: ${await contract.get_i256_max()}`)
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
