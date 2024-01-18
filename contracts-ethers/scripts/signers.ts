import hre from "hardhat";

async function main() {
  console.log(`Check signers...`)
  const { ethers } = hre;
  const signers = await ethers.getSigners();
  console.log(signers.length);
  console.log(signers[0].address);
  console.log(ethers.formatEther(await ethers.provider.getBalance(signers[0].address)));

  // const result = [];
  // for await (const signer of signers) {
  //   const address = signer.address;
  //   const balance = await ethers.provider.getBalance(address);
  //   result.push({ address, balance: ethers.formatEther(balance) });
  // }
  // console.log(result);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
