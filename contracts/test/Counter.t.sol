// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/Counter.sol";

contract CounterTest is Test {
    Counter public counter;

    function setUp() public {
        bytes memory pubk = vm.readFileBinary("test/data/public_key.pub");
        bytes memory zero_enc = vm.readFileBinary("test/data/zero.bin");
        counter = new Counter(pubk);
        counter.setNumber(zero_enc);
    }

    function testIncrement() public {
        bytes memory one_enc = vm.readFileBinary("test/data/one.bin");
        counter.increment();
        assertEq(counter.number(), one_enc);
    }

    function testSetNumber() public {
        bytes memory one_enc = vm.readFileBinary("test/data/one.bin");
        counter.setNumber(one_enc);
        assertEq(counter.number(), one_enc);
    }
}
