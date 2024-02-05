#[macro_use]
extern crate lazy_static;
use leptos::*;


mod app;
mod pages;
mod be_services;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    
    use axum::{routing::post, Router};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use ltrader_mock::app::*;
    use ltrader_mock::fileserv::file_and_error_handler;

    // use leptonict_template_ssr::app::*;
    // use leptonic_template_ssr::fileserv::file_and_error_handler;
    use std::env;
    use tracing_subscriber::{
        prelude::__tracing_subscriber_SubscriberExt,
        util::SubscriberInitExt,
        Layer,
    };
    // use ethers::{
    //     contract::{abigen, ContractFactory},
    //     core::utils::Anvil,
    //     core::types::{Address},
    //     middleware::SignerMiddleware,
    //     providers::{Http, Provider, StreamExt, Ws},
    //     signers::{LocalWallet, Signer},
    //     solc::{Artifact, Project, ProjectPathsConfig},
    //     core::utils::{parse_ether},
    // };
    // use eyre::Result;
    // use std::{path::PathBuf, sync::Arc, time::Duration, ptr::addr_of};
    // use crate::services::anvil_service;

    // 2. instantiate our wallet
    // let wallet: LocalWallet = anvil.keys()[0].clone().into();
    // let wallet_address:Address= wallet.address();
    // println!("wallet_address: {}", anvil_service::create_new_wallet().unwrap());

    // let LEPTOS_OUTPUT_NAME = env::var("LEPTOS_OUTPUT_NAME").expect("$LEPTOS_OUTPUT_NAME is not set");
    // let LEPTOS_SITE_ROOT = env::var("LEPTOS_SITE_ROOT").expect("$LEPTOS_SITE_ROOT is not set");
    // let LEPTOS_SITE_PKG_DIR = env::var("LEPTOS_SITE_PKG_DIR").expect("$LEPTOS_SITE_PKG_DIR is not set");


    // println!("LEPTOS_OUTPUT_NAME {}",LEPTOS_OUTPUT_NAME);
    // println!("LEPTOS_SITE_ROOT {}",LEPTOS_SITE_ROOT);
    // println!("LEPTOS_SITE_PKG_DIR {}",LEPTOS_SITE_PKG_DIR );

    let log_filter = tracing_subscriber::filter::Targets::new()
        .with_default(tracing::Level::INFO)
        .with_target("tokio", tracing::Level::WARN)
        .with_target("runtime", tracing::Level::WARN);

    let fmt_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_file(true)
        .with_line_number(true)
        .with_ansi(true)
        .with_thread_names(false)
        .with_thread_ids(false);
    println!("tracing_subscriber server ok...");

    let fmt_layer_filtered = fmt_layer.with_filter(log_filter);

    tracing_subscriber::Registry::default()
        .with(fmt_layer_filtered)
        .init();

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(Some("/home/imc/BitMole/TraderMock/Cargo.toml")).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    println!("leptos_options server ok...{}", addr);

    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    tracing::info!("listening on http://{}", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
