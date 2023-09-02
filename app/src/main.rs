use std::{fs, path::PathBuf, str::FromStr, sync::Arc};

use bindings::counter::Counter;
use clap::{Parser, Subcommand, ValueEnum};
use ethers::{
    prelude::rand::thread_rng,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
};
use eyre::{bail, Result};
use sunscreen_web3::{
    testnet::parasol::{generate_keys, PARASOL, RUNTIME},
    AsBytes, AsFile, AsNum, Ciphertext, PrivateKey, PublicKey, SignedMiddleware, Unsigned256,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum NetworkOption {
    /// Locally runnying Anvil node at http://localhost:8545
    ///
    /// If you supply this option, you probably want to supply a --wallet-key
    /// as well, and pass in one of the Anvil account secret keys.
    Local,
    /// Parasol network
    Parasol,
}

#[derive(Parser, Debug)]
struct Args {
    /// Key store directory which holds Parasol wallet key and Sunscreen FHE keys
    #[arg(short, long, default_value = ".keys")]
    key_store: PathBuf,

    /// Network to connect to
    #[arg(short, long, value_enum, default_value_t = NetworkOption::Parasol)]
    network: NetworkOption,

    /// Wallet key (override whatever wallet is in the key_store)
    #[arg(short, long)]
    wallet_key: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generate keys
    Gen {
        /// Overwrite keys if they exist
        #[arg(short, long)]
        force: bool,
    },

    /// Increment the counter
    Increment {
        /// Address of deployed counter contract
        #[arg(short, long)]
        contract_address: Address,
    },

    /// Decrypt the counter
    Decrypt {
        /// Address of deployed counter contract
        #[arg(short, long)]
        contract_address: Address,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    fs::create_dir_all(args.key_store.clone())?;

    match args.command {
        Commands::Gen { force } => {
            let keys = KeyStore::generate(&args.key_store, force)?;

            // Log messages to the user
            eprintln!("Saved new keys under directory {}", args.key_store.display());
            match args.network {
                NetworkOption::Parasol => eprintln!(
                    "Head to {}?address={:?} for some free SPETH!",
                    PARASOL.faucet_url,
                    keys.wallet.address()
                ),
                _ => {}
            }
        }
        Commands::Increment { contract_address } => {
            let keys = KeyStore::init(args.key_store, args.wallet_key)?;
            let counter = keys.contract(args.network, contract_address)?;
            counter.increment().send().await?.await?;
        }
        Commands::Decrypt { contract_address } => {
            let keys = KeyStore::init(args.key_store, args.wallet_key)?;
            let counter = keys.contract(args.network, contract_address)?;
            let value_enc = counter.reencrypt_number(keys.public_key.as_bytes()?).call().await?;
            let value: Unsigned256 =
                RUNTIME.decrypt(&Ciphertext::from_bytes(&value_enc)?, &keys.private_key)?;
            eprintln!("Current counter value: {}", value.to())
        }
    }

    Ok(())
}

struct KeyStore {
    wallet: LocalWallet,
    public_key: PublicKey,
    private_key: PrivateKey,
}

impl KeyStore {
    const WALLET_PATH: &'static str = "wallet.sk";
    const PRIVATE_KEY_PATH: &'static str = "fhe.pri";
    const PUBLIC_KEY_PATH: &'static str = "fhe.pub";

    /// Generate new keys and save them to the specified directory.
    fn generate(parent_dir: &PathBuf, force: bool) -> Result<Self> {
        // Throw errors if necessary
        if !force {
            for file in [Self::WALLET_PATH, Self::PRIVATE_KEY_PATH, Self::PUBLIC_KEY_PATH] {
                let path = parent_dir.join(file);
                if path.exists() {
                    bail!("{} already exists; use --force to overwrite it", path.display());
                }
            }
        }

        // Generate new keys
        let (public_key, private_key) = generate_keys()?;
        let wallet = LocalWallet::new(&mut thread_rng());

        // Write keys to files
        public_key.write(parent_dir.join(Self::PUBLIC_KEY_PATH))?;
        private_key.write(parent_dir.join(Self::PRIVATE_KEY_PATH))?;
        wallet.write(parent_dir.join(Self::WALLET_PATH))?;
        Ok(Self { wallet, public_key, private_key })
    }

    fn init(parent_dir: PathBuf, wallet_key: Option<String>) -> Result<Self> {
        let public_key = PublicKey::read(parent_dir.join(Self::PUBLIC_KEY_PATH))?;
        let private_key = PrivateKey::read(parent_dir.join(Self::PRIVATE_KEY_PATH))?;
        let wallet = match wallet_key {
            Some(s) => LocalWallet::from_str(&s)?,
            None => LocalWallet::read(parent_dir.join(Self::WALLET_PATH))?,
        };
        Ok(Self { wallet, public_key, private_key })
    }

    fn client(&self, network: NetworkOption) -> Result<Arc<SignedMiddleware>> {
        Ok(match network {
            NetworkOption::Local => {
                let provider =
                    Arc::new(Provider::<Http>::try_from("http://localhost:8545").unwrap());
                Arc::new(SignedMiddleware::new(
                    provider,
                    self.wallet.clone().with_chain_id(31337_u64),
                ))
            }
            NetworkOption::Parasol => PARASOL.client(self.wallet.clone()),
        })
    }

    fn contract(
        &self,
        network: NetworkOption,
        contract_address: Address,
    ) -> Result<Counter<SignedMiddleware>> {
        let client = self.client(network)?;
        let contract = Counter::new(contract_address, client);
        Ok(contract)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use sunscreen_web3::testing::Node;

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
            Ok(Self { counter, private_key, public_key, _node: node })
        }
    }

    #[tokio::test]
    async fn counter_works() -> Result<()> {
        let Test { counter, private_key, public_key, _node } = Test::new().await?;

        counter.increment().send().await?.log().await?;
        counter.increment().send().await?.log().await?;

        let two_enc = counter.reencrypt_number(public_key.as_bytes()?).call().await?;
        let two: Unsigned256 = RUNTIME.decrypt(&Ciphertext::from_bytes(&two_enc)?, &private_key)?;

        assert_eq!(two, Unsigned256::from(2));
        Ok(())
    }
}
