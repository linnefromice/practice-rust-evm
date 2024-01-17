import {
  time,
  loadFixture,
} from "@nomicfoundation/hardhat-toolbox/network-helpers";
import { expect } from "chai";
import hre from "hardhat";

const MAX_U256 = (2n << 255n) - 1n;
const MAX_I256 = ((2n << 255n) / 2n) - 1n;
const MIN_I256 = -((2n << 255n) / 2n);

describe("Viewer", function () {
  // We define a fixture to reuse the same setup in every test.
  // We use loadFixture to run this setup once, snapshot that state,
  // and reset Hardhat Network to that snapshot in every test.
  async function deploy() {
    // Contracts are deployed using the first signer/account by default
    const { ethers } = hre;
    const [owner, otherAccount] = await ethers.getSigners();

    const Viewer = await ethers.getContractFactory("Viewer");
    const contract = await Viewer.deploy();

    return {
      contract,
      owner,
      otherAccount
    };
  }

  describe("Deployment", function () {
    it(".name", async () => {
      const { contract } = await loadFixture(deploy);

      expect(await contract.name()).to.equal("Viewer");
    });
    it(".get_[u|i]256", async () => {
      const { contract } = await loadFixture(deploy);

      expect(await contract.get_u256_max()).to.equal(MAX_U256);
      expect(await contract.get_u256_min()).to.equal(0n);

      expect(await contract.get_i256_max()).to.equal(MAX_I256);
      expect(await contract.get_i256_min()).to.equal(MIN_I256);
    });
    it(".get_i256s", async () => {
      const { contract } = await loadFixture(deploy);

      expect(await contract.get_multiple_i256()).to.deep.equal([
        MIN_I256,
        0n,
        MAX_I256,
      ]);
      expect(await contract.get_multiple_i256_plus()).to.deep.equal([
        1n,
        MAX_I256 / 2n,
        MAX_I256 - 1n,
      ]);
      expect(await contract.get_multiple_i256_minus()).to.deep.equal([
        -1n,
        MIN_I256 / 2n,
        MIN_I256 + 1n,
      ]);
    })
  });
});
