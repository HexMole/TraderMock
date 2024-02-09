#[macro_use]
extern crate lazy_static;
use leptos::*;


mod app;
mod pages;
mod be_services;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use dotenv::dotenv;
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

    let LEPTOS_OUTPUT_NAME = env::var("LEPTOS_OUTPUT_NAME").expect("$LEPTOS_OUTPUT_NAME is not set");
    let LEPTOS_SITE_ROOT = env::var("LEPTOS_SITE_ROOT").expect("$LEPTOS_SITE_ROOT is not set");
    let LEPTOS_SITE_PKG_DIR = env::var("LEPTOS_SITE_PKG_DIR").expect("$LEPTOS_SITE_PKG_DIR is not set");


    println!("LEPTOS_OUTPUT_NAME {}",LEPTOS_OUTPUT_NAME);
    println!("LEPTOS_SITE_ROOT {}",LEPTOS_SITE_ROOT);
    println!("LEPTOS_SITE_PKG_DIR {}",LEPTOS_SITE_PKG_DIR );

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


    dotenv().ok();
    let cargo_toml_file = std::env::var("CARGO_TOML_FILE").expect("CARGO_TOML_FILE must be set.");

    println!("cargo_toml_file: {}", cargo_toml_file);

    let conf = get_configuration(Some(&cargo_toml_file.to_string())).await.unwrap();
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
pub fn main() {}
