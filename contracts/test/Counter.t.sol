// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/Counter.sol";

contract CounterTest is Test {
    Counter public counter;

    function setUp() public {
        counter = new Counter();
    }

    function testIncrement() public {
        // Inc to 2
        counter.increment();
        counter.increment();

        // Decrypt
        bytes memory current_number_enc = counter.number();
        uint256 current_number = FHE.decryptUint256(current_number_enc);

        assertEq(current_number, 2);
    }

    function testSetNumber() public {
        // Set to 100
        counter.setNumber(100);

        // Inc to 101
        counter.increment();

        // Decrypt
        bytes memory current_number_enc = counter.number();
        uint256 current_number = FHE.decryptUint256(current_number_enc);

        assertEq(current_number, 101);
    }
}
