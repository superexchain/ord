use http::HeaderMap;

use super::*;

#[derive(Serialize)]
pub(crate) struct Output {
  pub mnemonic: Mnemonic,
  pub passphrase: Option<String>,
}

#[derive(Debug, Parser, Deserialize)]
pub(crate) struct Create {
  #[clap(
    long,
    default_value = "",
    help = "Use <PASSPHRASE> to derive wallet seed."
  )]
  pub(crate) passphrase: String,
}

impl Create {
  pub(crate) fn run(self, options: Options) -> Result {
    let mut entropy = [0; 16];
    rand::thread_rng().fill_bytes(&mut entropy);

    let mnemonic = Mnemonic::from_entropy(&entropy)?;

    initialize_wallet(&options, mnemonic.to_seed(self.passphrase.clone()))?;

    print_json(Output {
      mnemonic,
      passphrase: Some(self.passphrase),
    })?;

    Ok(())
  }

  pub(crate) fn run_api(self, options: &Options, header: &HeaderMap) -> Result<Output> {
    let mut entropy = [0; 16];
    rand::thread_rng().fill_bytes(&mut entropy);

    let mnemonic = Mnemonic::from_entropy(&entropy)?;

    let wallet = &String::from(header.get("wallet").unwrap().to_str().unwrap());

    initialize_wallet_and_rpc_url(&options, mnemonic.to_seed(self.passphrase.clone()), wallet)?;

    Ok(Output {
      mnemonic,
      passphrase: Some(self.passphrase),
    })
  }
}
