import hre from "hardhat";

async function main() {
  console.log(`Deploy Viewer contract...`)

  const contract = await hre.viem.deployContract("Viewer", []);
  console.log(`contract address: ${contract.address}`)
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
