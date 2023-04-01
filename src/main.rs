use std::fs;

use gm::runtime_types::{gm_chain_runtime::Call, orml_currencies::module::Call as CurrenciesCall};

use sp_core::Pair;
use structopt::StructOpt;
use subxt::{tx::PairSigner, OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "gm_metadata.scale")]
pub mod gm {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args = Cli::from_args();

    let mnemonic_phrase =
        fs::read_to_string(args.seed_file).expect("Should read from seed phrase file.");

    let pair = sp_core::sr25519::Pair::from_phrase(mnemonic_phrase.as_str(), None).unwrap();

    let signer = PairSigner::<PolkadotConfig, sp_core::sr25519::Pair>::new(pair.0);

    let api = OnlineClient::<PolkadotConfig>::from_url("wss://leemo.gmordie.com:443").await?;

    let batch_limit_address = gm::constants().utility().batched_calls_limit();

    let batch_limit = api.constants().at(&batch_limit_address)?;

    println!("Batch limit: {}", batch_limit);

    let burn_txn = Call::Currencies(CurrenciesCall::burn_fren {
        amount: 10_000_000_000_000,
    });

    let balance_txn = gm::tx().currencies().burn_fren(10_000_000_000_000);

    let batch_txn = gm::tx()
        .utility()
        .batch(vec![Call::Currencies(CurrenciesCall::burn_fren {
            amount: 10_000_000_000_000,
        })]);

    Ok(())
}

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "s")]
    seed_file: String,
}
