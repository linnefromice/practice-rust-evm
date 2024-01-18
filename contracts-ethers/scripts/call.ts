import hre from "hardhat";

async function main() {
  console.log(`Call Viewer contract...`)
  const { ethers } = hre;
  
  // localhost: 0x5FbDB2315678afecb367f032d93F642f64180aa3
  // mumbai: 0x998b6bCBaB1C25D47c68990349670b2AfA24015E
  const contract = await ethers.getContractAt("Viewer", "0x998b6bCBaB1C25D47c68990349670b2AfA24015E");
  console.log(`contract address: ${await contract.getAddress()}`)
  console.log(`name: ${await contract.name()}`)
  console.log(`version: ${await contract.version()}`)
  console.log(`i256_min: ${await contract.get_i256_min()}`)
  console.log(`i256_max: ${await contract.get_i256_max()}`)
  console.log(`multi i256: ${await contract.get_multiple_i256()}`)
  console.log(`multi i256 plus: ${await contract.get_multiple_i256_plus()}`)
  console.log(`multi i256 minus: ${await contract.get_multiple_i256_minus()}`)
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
