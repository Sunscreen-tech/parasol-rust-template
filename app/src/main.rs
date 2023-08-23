use std::sync::Arc;

use bindings::counter::Counter;
use ethers::{
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
};
use sunscreen_web3::{testnet::parasol::generate_keys, AsBytes, SignedMiddleware};

use eyre::Result;
use sunscreen_web3::testnet::parasol::PARASOL;

#[tokio::main]
async fn main() -> Result<()> {
    let wallet: LocalWallet = sunscreen_web3::testing::ALICE.clone();
    let provider = Arc::new(Provider::<Http>::try_from("http://localhost:8545")?);
    let client = Arc::new(SignedMiddleware::new(provider, wallet.with_chain_id(PARASOL.chain_id)));
    let contract_addr =
        Counter::deploy(Arc::clone(&client), ())?.gas(300_000_000_000u64).send().await?.address();
    println!("Contract deployed at address {:#?}", contract_addr);
    Ok(())
}

// 10. Deploying contract
// 11. Gen admin parasol keys
// 12. Go to faucet with pub key
// 13. Do deploy (script? or cli bindings?) maybe in this case clients are also contract owners, in which case cli bindings.
// 14. Instruct user to save the deployed contract address and save it as a constant in the client code.

#[cfg(test)]
mod tests {

    use super::*;
    use sunscreen_web3::{
        testing::Node, testnet::parasol::RUNTIME, Ciphertext, PrivateKey, PublicKey,
        SignedMiddleware, Unsigned256,
    };

    struct Test {
        counter: Counter<SignedMiddleware>,
        private_key: PrivateKey,
        public_key: PublicKey,
        // When this is dropped, the node is killed
        _node: Node,
    }

    impl Test {
        async fn new() -> Result<Self> {
            let node = Node::spawn();
            let (public_key, private_key) = generate_keys()?;
            let wallet: LocalWallet = node.anvil.keys()[0].clone().into();
            let client = Arc::new(node.client(wallet));
            let contract_addr = Counter::deploy(Arc::clone(&client), ())?.send().await?.address();
            let counter = Counter::new(contract_addr, client);
            counter
                .set_public_key(public_key.as_bytes()?)
                .send()
                .await?
                .log_msg("Pending transfer hash")
                .await?;
            Ok(Self { counter, private_key, public_key, _node: node })
        }
    }

    #[tokio::test]
    async fn counter_works() -> Result<()> {
        let Test { counter, private_key, public_key, _node } = Test::new().await?;

        let zero_enc = RUNTIME.encrypt(Unsigned256::from(0), &public_key)?;

        counter.set_number(zero_enc.as_bytes()?).send().await?.log().await?;
        counter.increment().send().await?.log().await?;
        counter.increment().send().await?.log().await?;

        let two_enc = counter.number().call().await?;
        let two: Unsigned256 = RUNTIME.decrypt(&Ciphertext::from_bytes(&two_enc)?, &private_key)?;

        assert_eq!(two, Unsigned256::from(2));
        Ok(())
    }
}
