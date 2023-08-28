// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import "sunscreen/src/FHE.sol";

contract Counter {
    bytes public number;
    bytes public publicKey;
    FHE fhe;

    constructor() {
        fhe = new FHE();
    }

    function setPublicKey(bytes memory _publicKey) public {
        publicKey = _publicKey;
    }

    function setNumber(bytes memory _number) public {
        number = _number;
    }

    function increment() public {
        number = fhe.addUint256EncPlain(publicKey, number, 1);
    }
}
