import { ethers } from "hardhat";

async function main() {
  console.log(`Deploying Viewer contract...`)
  const contract = await ethers.deployContract("Viewer", []);
  await contract.waitForDeployment();

  console.log(`contract address: ${await contract.getAddress()}`)
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
