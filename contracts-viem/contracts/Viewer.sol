// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

// Uncomment this line to use console.log
// import "hardhat/console.sol";

contract Viewer {
    function name() public pure returns (string memory) {
        return "Viewer";
    }
    function version() public pure returns (string memory) {
        return "0.0.1";
    }
    
    function get_u256_min() public pure returns (uint256) {
        return type(uint256).min;
    }
    function get_u256_max() public pure returns (uint256) {
        return type(uint256).max;
    }
    function get_i256_min() public pure returns (int256) {
        return type(int256).min;
    }
    function get_i256_max() public pure returns (int256) {
        return type(int256).max;
    }

    function get_i256s() public pure returns (int256, int256, int256) {
        return (
            get_i256_min(),
            0,
            get_i256_max()
        );
    }
    function get_i256s_plus() public pure returns (int256, int256, int256) {
        return (
            1,
            get_i256_max() / 2,
            get_i256_max() - 1
        );
    }
    function get_i256s_minus() public pure returns (int256, int256, int256) {
        return (
            -1,
            get_i256_min() / 2,
            get_i256_min() + 1
        );
    }

    function get_i256_from_i64(int64 x) public pure returns (int256) {
        return int256(x);
    }
    function get_i256_from_i128(int128 x) public pure returns (int256) {
        return int256(x);
    }
    function get_i256_from_i256(int256 x) public pure returns (int256) {
        return int256(x);
    }
}
