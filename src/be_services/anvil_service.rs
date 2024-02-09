use lazy_static::lazy_static; // 1.4.0
use cfg_if::cfg_if;
use serde::Serialize;
use serde::Deserialize;


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractModel
{
    pub Type:String,
    pub ContractAddress: String,
    pub Name:String,
}

cfg_if! { if #[cfg(feature = "ssr")] {

    use std::{ops::Deref, sync::Mutex};


    use ethers::{
        contract::{abigen, ContractFactory},
        core::utils::Anvil,
        core::types::{Address},
        middleware::SignerMiddleware,
        providers::{Http, Provider, StreamExt, Ws},
        signers::{LocalWallet, Signer},
        solc::{Artifact, Project, ProjectPathsConfig},
        core::utils::{parse_ether},
    };
    use eyre::Result;
    use std::{path::PathBuf, sync::Arc, time::Duration, ptr::addr_of};


    lazy_static! {
        pub static ref AnvilInst: Mutex<ethers_core::utils::AnvilInstance> = Mutex::new(Anvil::new().args(["--code-size-limit", "100000"]).spawn());
        pub static ref ContractList: Mutex<Vec<ContractModel>> = Mutex::new(Vec::new());
    }

    pub fn  create_new_wallet() -> Result<Address>{
        let wallet: LocalWallet = AnvilInst.lock().unwrap().keys()[0].clone().into();
        let wallet_address:Address = wallet.address();

        return Ok(wallet_address);
    }


}}