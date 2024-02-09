
use std::vec;

use ethers::solc::resolver::print;
use leptonic::prelude::*;
use leptos::*;
use leptos::error::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use lazy_static::lazy_static;


// use gloo_net::http::Request;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cat {
    url: String,
}



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
use crate::be_services::anvil_service;

#[server]
async fn fetch_cats(count: CatCount) -> Result<Vec<String>, ServerFnError> {
    println!("Fetching cats...{}"   , count);
    if count > 0 {
        // // make the request
        let newAddress=anvil_service::create_new_wallet().unwrap();
        println!("wallet_address: {}",newAddress);
        println!("fetched?");
        let res = vec![newAddress.to_string()];
        Ok(res)
    } else {
        let res= vec![];
        Ok(res)
        // Err(CatError::NonZeroCats.into())
    }
}



#[cfg(feature = "ssr")]
use crate::be_services::anvil_service::*;

use crate::be_services::anvil_service::ContractModel;
#[server]
pub async fn get_deployed_contracts() -> Result<Vec<ContractModel>, ServerFnError> {
    println!("uniswap_contracts_deployed");
    // let contracts:Vec<ContractModel> =ContractList.lock().unwrap().clone();

    let contractModel1=ContractModel{
        Type:"contract1".to_string(),
        ContractAddress: "someaddress1".to_string(),
        Name:"contract Name1".to_string(),

    };

    let contractModel2=ContractModel{
        Type:"contract2".to_string(),
        ContractAddress: "someaddress2".to_string(),
        Name:"contract Name2".to_string(),

    };


    ContractList.lock().unwrap().push(contractModel1);
    ContractList.lock().unwrap().push(contractModel2);

    let cloned_contractList= ContractList.lock().unwrap().clone();


    //  contractLength.push(contractModel);
        
    // });
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



