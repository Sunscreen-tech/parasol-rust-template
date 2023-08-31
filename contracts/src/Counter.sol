// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import "sunscreen/src/FHE.sol";

contract Counter {
    bytes public number;
    FHE fhe;

    constructor() {
        fhe = new FHE();
        number = fhe.encryptUint256(0);
    }

    function setNumber(uint256 _number) public {
        number = fhe.encryptUint256(_number);
    }

    function increment() public {
        number = fhe.addUint256EncPlain(fhe.networkPublicKey(), number, 1);
    }

    function reencryptNumber(bytes memory publicKey) public view returns (bytes memory) {
        return fhe.reencryptUint256(publicKey, number);
    }
}
