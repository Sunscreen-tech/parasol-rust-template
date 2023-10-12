# counter cli

This CLI allows you to interact with either a local Anvil node for testing, or
directly on the Parasol network. The following instructions assume your current
directory is here in `app`.

## anvil

First, let's go over how to deploy this on a local node for testing. To start,
spin up Anvil:

```sh
$ anvil \
    --mnemonic "gas monster ski craft below illegal discover limit dog bundle bus artefact" \
    --gas-limit 3000000000000000000
```

### Generate keys

We'll use the unlocked funded accounts on the Anvil instance, but we still need
an FHE keypair:

```sh
$ cargo run -- --network local gen
Saved new keys under directory .keys
```

For the rest of counter CLI commands, we'll pass in a `--wallet-key` to
override the one generated in `.keys`, since that one won't have any funds in
Anvil.

### Deploy the contract

We'll need to first deploy the `FHE` library, and then link to it when deploying
the `Counter` contract.

```sh
# first deploy the FHE library
$ forge create \
    --root=../contracts/lib/sunscreen-contracts \
    --private-key 0x1c0eb5244c165957525ef389fc14fac4424feaaefabf87c7e4e15bcc7b425e15 \
    ./src/FHE.sol:FHE
Deployer: 0xB5f27C716E44ffe48FD6622983c651355AD8C75A
Deployed to: 0xb70a3B53d02B7882bEe532e5387343AA3263F0e8
Transaction hash: 0xb26ffe392b77207b7d6ef42de02d4ee95cb190cfd4f43fad9958226fb8f561a9

# then use the above "deployed to" hash when deploying the counter contract
$ forge create \
    --root=../contracts \
    --private-key 0x1c0eb5244c165957525ef389fc14fac4424feaaefabf87c7e4e15bcc7b425e15 \
    --libraries=lib/sunscreen-contracts/src/FHE.sol:FHE:0xb70a3B53d02B7882bEe532e5387343AA3263F0e8 \
    ./src/Counter.sol:Counter
Deployer: 0xB5f27C716E44ffe48FD6622983c651355AD8C75A
Deployed to: 0xbb9AbFB450CBc64e550b4e41a1b9B7aD8313450C
Transaction hash: 0x3779c4421c64e15a4b48046c0f30a93a935228f9d9f9f93f3e8ba39f524dc093
```

In the following commands, you'll use the "deployed to" address of the `Counter`
contract.

### Verify the counter is zero

The contract counter state is _encrypted_. Use your local private key to decrypt
the value:

```sh
$ cargo run -- \
    --wallet-key 0x1c0eb5244c165957525ef389fc14fac4424feaaefabf87c7e4e15bcc7b425e15 \
    --network local \
    decrypt \
    --contract-address 0xbb9AbFB450CBc64e550b4e41a1b9B7aD8313450C
Current counter value: 0
```

### Increment the counter

Use homomorphic addition to increment the counter a couple times:

```sh
$ cargo run -- \
    --wallet-key 0x1c0eb5244c165957525ef389fc14fac4424feaaefabf87c7e4e15bcc7b425e15 \
    --network local \
    increment \
    --contract-address 0xbb9AbFB450CBc64e550b4e41a1b9B7aD8313450C
$ cargo run -- \
    --wallet-key 0x1c0eb5244c165957525ef389fc14fac4424feaaefabf87c7e4e15bcc7b425e15 \
    --network local \
    increment \
    --contract-address 0xbb9AbFB450CBc64e550b4e41a1b9B7aD8313450C
```

### Decrypt the counter

Decrypt the counter again to see the current value:

```sh
$ cargo run -- \
    --wallet-key 0x1c0eb5244c165957525ef389fc14fac4424feaaefabf87c7e4e15bcc7b425e15 \
    --network local \
    decrypt \
    --contract-address 0xbb9AbFB450CBc64e550b4e41a1b9B7aD8313450C
Current counter value: 2
```

## parasol

### Generate keys

Generate a Parasol account and an FHE keypair. By default this will store keys
under a `.keys` directory:

```sh
$ cargo run -- gen
Saved new keys under directory .keys
Head to https://faucet.sunscreen.tech/?address=0x3ca28c3a100651a38789ddaec115e0a255078551 for some free SPETH!
```

### Fund your account!

To start, your new wallet has no currency. Follow the link in the output above to get
some SPETH currency so that you can deploy the contract.

### Deploy the counter contract

Note that our FHE library contract is already deployed to Parasol at address `0xb70a3B53d02B7882bEe532e5387343AA3263F0e8`, so you could skip directly to deploying the `Counter` contract. But if you want to deploy a copy of the library yourself, feel free to do so!

```sh
# optional: deploy the FHE library
$ forge create \
    --chain 574 \
    --rpc-url 'https://rpc.sunscreen.tech/parasol' \
    --root=../contracts/lib/sunscreen-contracts \
    --private-key $(hexdump -ve '1/1 "%.2x"' .keys/wallet.sk) \
    src/FHE.sol:FHE
Deployer: 0xB5f27C716E44ffe48FD6622983c651355AD8C75A
Deployed to: 0xb70a3B53d02B7882bEe532e5387343AA3263F0e8
Transaction hash: 0xb26ffe392b77207b7d6ef42de02d4ee95cb190cfd4f43fad9958226fb8f561a9

# use the above contract address to link to FHE when deploying Counter
$ forge create \
    --chain 574 \
    --rpc-url 'https://rpc.sunscreen.tech/parasol' \
    --root=../contracts \
    --private-key $(hexdump -ve '1/1 "%.2x"' .keys/wallet.sk) \
    --libraries=lib/sunscreen-contracts/src/FHE.sol:FHE:0xb70a3B53d02B7882bEe532e5387343AA3263F0e8 \
    ./src/Counter.sol:Counter
Deployer: 0xB5f27C716E44ffe48FD6622983c651355AD8C75A
Deployed to: 0xbb9AbFB450CBc64e550b4e41a1b9B7aD8313450C
Transaction hash: 0x3779c4421c64e15a4b48046c0f30a93a935228f9d9f9f93f3e8ba39f524dc093
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
