use http::HeaderMap;

use super::*;

#[derive(Deserialize, Serialize)]
pub struct Output {
  pub address: Address,
}

pub(crate) fn run(options: Options) -> Result {
  let address = options
    .bitcoin_rpc_client_for_wallet_command(false)?
    .get_new_address(None, Some(bitcoincore_rpc::json::AddressType::Bech32m))?;

  print_json(Output { address })?;

  Ok(())
}

pub(crate) fn run_api(options: &Options, header: &HeaderMap) -> Result<Output> {
  let wallet = &String::from(header.get("wallet").unwrap().to_str().unwrap());  
  let address = options
    .bitcoin_rpc_client_for_wallet_command_and_rpc_name(false, wallet)?
    .get_new_address(None, Some(bitcoincore_rpc::json::AddressType::Bech32m))?;

  Ok(Output { address })
}