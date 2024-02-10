
use std::vec;

use ethers::solc::resolver::print;
use leptonic::prelude::*;
use leptos::*;
use leptos::error::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use lazy_static::lazy_static;

type CatCount = usize;

#[cfg(feature = "ssr")]
use ethers::{
    contract::{abigen, ContractFactory},
    core::utils::Anvil,
    core::types::Address,
    middleware::SignerMiddleware,
    providers::{Http, Provider, StreamExt, Ws},
    signers::{LocalWallet, Signer},
    solc::{Artifact, Project, ProjectPathsConfig},
    core::utils::parse_ether,
};


#[cfg(feature = "ssr")]
use crate::be_services::anvil_service::*;

use crate::be_services::anvil_service::ContractModel;
#[server]
pub async fn get_deployed_contracts() -> Result<Vec<ContractModel>, ServerFnError> {
    
    let cloned_contractList= ContractList.lock().unwrap().clone();

    println!("uniswap_contracts_deployed {}", cloned_contractList.len());
    Ok(cloned_contractList)
}


#[component]
pub fn Deployments() -> impl IntoView {

    let contracts_deployed_rsc = create_resource(|| (), |_| async move { get_deployed_contracts().await });
    view! {
        <h1>"Deployed Contracts"</h1>
        <Suspense fallback=move || view! { <p>"Loading..."</p> } >
        <TableContainer>
                <Table bordered=true hoverable=true>
                    <TableHeader>
                        <TableRow>
                            <TableHeaderCell min_width=true>"#"</TableHeaderCell>
                            <TableHeaderCell>"Name"</TableHeaderCell>
                            <TableHeaderCell>"Type"</TableHeaderCell>
                            <TableHeaderCell>"Name"</TableHeaderCell>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                                {move || {
                                    contracts_deployed_rsc.and_then(|contracts| {
                                        contracts.iter()
                                                .map(|contract| {
                                                            let contract_name = contract.Name.clone();
                                                            let contract_ContractAddress = contract.ContractAddress.clone();
                                                            let contract_type = contract.Type.clone();
                                                            view! {
                                                                <TableRow>
                                                                    <TableCell>"#"</TableCell>
                                                                    <TableCell>{contract_name}</TableCell>
                                                                    <TableCell>{contract_ContractAddress}</TableCell>
                                                                    <TableCell>{contract_type}</TableCell>
                                                                </TableRow>
                                                    }
                                                }).collect_view()
                                        })
                                    }
                                }
                    </TableBody>
                </Table>
            </TableContainer>
        </Suspense>
    }
}



