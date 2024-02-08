
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

#[server]
pub async fn deploy_contract() -> Result<bool, ServerFnError> {
    let contractLength = ContractList.lock().unwrap().len();
    println!("deploy_contract {}",contractLength);
    Ok(contractLength==0)
}

#[server]
pub async fn list_post_metadata() -> Result<Vec<PostMetadata>, ServerFnError> {
    println!("list_post_metadata");
    // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    Ok(POSTS
        .iter()
        .map(|data| PostMetadata {
            id: data.id,
            title: data.title.clone(),
        })
        .collect())
}

// #[derive(Params, Copy, Clone, Debug, PartialEq, Eq)]
// pub struct PostParams {
//     id: usize,
// }

#[component]
pub fn Deployments() -> impl IntoView {

    let contracts_deployed_rsc = create_resource(|| (), |_| async move { get_deployed_contracts().await });
    view! {
        <h1>"Deployed Contracts"</h1>
        <Suspense fallback=move || view! { <p>"Loading..."</p> } >
        {move || {
            contracts_deployed_rsc.and_then(|contracts| {
                contracts.iter()
                        .map(|contract| view! {
                            <li>{contract.Name.clone()}</li>
                        })
                        .collect_view()
                })
            }
        }
        </Suspense>
    }
}


#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PostError {
    #[error("Invalid post ID.")]
    InvalidId,
    #[error("Post not found.")]
    PostNotFound,
    #[error("Server error.w")]
    ServerError,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    id: usize,
    title: String,
    content: String,
}

// Dummy API
lazy_static! {
    static ref POSTS: Vec<Post> = vec![
        Post {
            id: 0,
            title: "My first post".to_string(),
            content: "This is my first post".to_string(),
        },
        Post {
            id: 1,
            title: "My second post".to_string(),
            content: "This is my second post".to_string(),
        },
        Post {
            id: 2,
            title: "My third post".to_string(),
            content: "This is my third post".to_string(),
        },
    ];
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostMetadata {
    id: usize,
    title: String,
}



#[server]
pub async fn get_post(id: usize) -> Result<Option<Post>, ServerFnError> {
    println!("get_post");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    Ok(POSTS.iter().find(|post| post.id == id).cloned())
}