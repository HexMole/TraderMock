use lazy_static::lazy_static; // 1.4.0
use cfg_if::cfg_if;



cfg_if! { if #[cfg(feature = "ssr")] {
    use std::sync::Mutex;
    use super::anvil_service::AnvilInst;

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
    
    pub fn  create_new_wallet() -> Result<Address>{
        let wallet: LocalWallet = AnvilInst.lock().unwrap().keys()[0].clone().into();
        let wallet_address:Address= wallet.address();
        return Ok(wallet_address);
    }

}}