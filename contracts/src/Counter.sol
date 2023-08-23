// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "sunscreen/src/FHE.sol";

contract Counter {
    bytes public number;
    bytes public publicKey;
    FHE fhe;

    constructor(bytes memory _publicKey) {
        fhe = new FHE();
        publicKey = _publicKey;
    }

    function setNumber(bytes memory _number) public {
        number = _number;
    }

    function increment() public {
        number = fhe.addUint256EncPlain(publicKey, number, 1);
    }
}
