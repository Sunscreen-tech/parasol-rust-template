# counter cli

This CLI allows you to interact with either a local Anvil node for testing, or
directly on the Parasol network.

# usage

### Generate keys to start
Generate a parasol account and an FHE keypair. By default this will store keys
under a `.keys` directory:

```sh
$ cargo run -- gen
Saved new keys under directory .keys
Head to https://faucet.sunscreen.tech/?address=0x3ca28c3a100651a38789ddaec115e0a255078551 for some free SPETH!
```

### Fund your account!
To start, your new wallet has no money. Follow the link in the output to get some SPETH
currency so that you can deploy the contract.

### Deploy the counter contract
To start, your new wallet has no money. Follow the link to get some SPETH
currency so that you can deploy the contract.

```sh
$ cargo run -- deploy
Contract deployed at address 0x19dbbd8998f2bcf4b1fe13d2e53b2b258a7ada65
Assigned public key to contract
Initialized counter to zero
```

Copy the contract address to call the methods on it below.

### Verify the counter is zero
The contract counter state is _encrypted_. Use your local private key to decrypt
the value:

```sh
$ cargo run -- decrypt --contract-address 0x19dbbd8998f2bcf4b1fe13d2e53b2b258a7ada65
Current counter value: 0
```

### Increment the counter
Use homomorphic addition to increment the counter a couple times:

```sh
$ cargo run -- increment --contract-address 0x19dbbd8998f2bcf4b1fe13d2e53b2b258a7ada65
$ cargo run -- increment --contract-address 0x19dbbd8998f2bcf4b1fe13d2e53b2b258a7ada65
```

### Decrypt the counter
Decrypt the counter again to see the current value:

```sh
$ cargo run -- decrypt --contract-address 0x19dbbd8998f2bcf4b1fe13d2e53b2b258a7ada65
Current counter value: 2
```
