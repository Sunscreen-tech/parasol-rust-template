// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import "sunscreen/src/FHE.sol";

contract Counter {
    bytes public number;

    constructor() {
        number = FHE.encryptUint256(0);
    }

    function setNumber(uint256 _number) public {
        number = FHE.encryptUint256(_number);
    }

    function increment() public {
        number = FHE.addUint256EncPlain(FHE.networkPublicKey(), number, 1);
    }

    function reencryptNumber(bytes memory publicKey) public view returns (bytes memory) {
        return FHE.reencryptUint256(publicKey, number);
    }
}
