
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
    core::types::{Address},
    middleware::SignerMiddleware,
    providers::{Http, Provider, StreamExt, Ws},
    signers::{LocalWallet, Signer},
    solc::{Artifact, Project, ProjectPathsConfig},
    core::utils::{parse_ether},
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
    tracing::info!("IntoView component");

    let posts =
        create_resource(|| (), |_| async { list_post_metadata().await });


    let postsByNumber =
        create_resource(|| (), |_| async { list_post_metadata().await });

    let posts_view = move || {
        posts.and_then(|posts| {
                        posts.iter()
                            .map(|post| view! {
                                <li>
                                    <a href=format!("/post/{}", post.id)>{&post.title}</a> "|"
                                    <a href=format!("/post_in_order/{}", post.id)>{&post.title}"(in order)"</a>
                                </li>
                            })
                            .collect_view()
                    })
    };

    let (cat_count, set_cat_count) = create_signal::<CatCount>(0);

    let cats = create_local_resource(move || cat_count.get(), fetch_cats);

    let fallback = move |errors: RwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                    .collect_view()
            })
        };

        view! {
            <div class="error">
                <h2>"Error"</h2>
                <ul>{error_list}</ul>
            </div>
        }
    };

    let cats_view = move || {
        cats.and_then(|data| {
            data.iter()
                // .map(|s| view! { <p><img src={s}/></p> })
                .map(|s| view! {<pre><label>{s}</label></pre>})
                .collect_view()
        })
    };
    //set_cat_count.update(|n| *n=val);
    view! {
        <h1>"My Great Blog"</h1>
        <Suspense fallback=move || view! { <p>"Loading posts..."</p> }>
            <ul>{posts_view}</ul>
        </Suspense>
        <div>
            <label>
                "How many cats would you like?"
                <input
                    type="number"
                    prop:value=move || cat_count.get().to_string()
                    on:input=move |ev| {
                        tracing::info!("How many cats called");
                        let val = event_target_value(&ev).parse::<CatCount>().unwrap_or(0);
                        set_cat_count.update(|n| *n=val);
                    }
                />
            </label>
            <Button on_click=move |_| {tracing::info!("How many cats called");} variant=ButtonVariant::Flat>"Flat"</Button>
            <Transition fallback=move || {
                view! { <div>"Loading (Suspense Fallback)..."</div> }
            }>
                <ErrorBoundary fallback>
                <div>
                    {cats_view}
                </div>
                </ErrorBoundary>
            </Transition>
        </div>
        <H1>"Contract deployed"</H1>

        <P>"Uniswap v3 Contracts:"</P>

        <TableContainer>
        <Table bordered=true hoverable=true>
            <TableHeader>
                <TableRow>
                    <TableHeaderCell min_width=true>"#"</TableHeaderCell>
                    <TableHeaderCell>"Name"</TableHeaderCell>
                    <TableHeaderCell>"Appearance"</TableHeaderCell>
                    <TableHeaderCell>"Num. eyes"</TableHeaderCell>
                </TableRow>
            </TableHeader>
            <TableBody>
                <TableRow>
                    <TableCell>"1"</TableCell>
                    <TableCell>"Kevin"</TableCell>
                    <TableCell>"Tall"</TableCell>
                    <TableCell>"2"</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>"2"</TableCell>
                    <TableCell>"Bob"</TableCell>
                    <TableCell>"Short"</TableCell>
                    <TableCell>"2"</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>"3"</TableCell>
                    <TableCell>"Stuart"</TableCell>
                    <TableCell>"Medium"</TableCell>
                    <TableCell>"1"</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>"4"</TableCell>
                    <TableCell>"Otto"</TableCell>
                    <TableCell>"Round"</TableCell>
                    <TableCell>"2"</TableCell>
                </TableRow>
            </TableBody>
        </Table>
    </TableContainer>
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



// #[derive(Params, Copy, Clone, Debug, PartialEq, Eq)]
// pub struct PostParams {
//     id: Option<usize>,
// }

// #[component]
// pub fn Post() -> impl IntoView {
//     let query = use_params::<PostParams>();
//     let id = move || {
//         query.with(|q| {
//             q.as_ref().map(|q| q.id).map_err(|_| PostError::InvalidId)
//         })
//     };
//     let post = create_resource(id, |id| async move {
//         match id {
//             Err(e) => Err(e),
//             Ok(id) => get_post(id.unwrap())
//                 .await
//                 .map(|data| data.ok_or(PostError::PostNotFound))
//                 .map_err(|_| PostError::ServerError),
//         }
//     });

//     let post_view = move || {
//         post.and_then(|post| {
//             view! {
//                 // render content
//                 <h1>{&post.clone().unwrap().title}</h1>
//                 <p>{&post.clone().unwrap().content}</p>

//                 // since we're using async rendering for this page,
//                 // this metadata should be included in the actual HTML <head>
//                 // when it's first served
//                 // <Title text=post.title.clone()/>
//                 // <Meta name="description" content=post.content.clone()/>
//             }
//         })
//     };

//     view! {
//         <Suspense fallback=move || view! { <p>"Loading post..."</p> }>
//             <ErrorBoundary fallback=|errors| {
//                 view! {
//                     <div class="error">
//                         <h1>"Something went wrong."</h1>
//                         <ul>
//                         {move || errors.get()
//                             .into_iter()
//                             .map(|(_, error)| view! { <li>{error.to_string()} </li> })
//                             .collect_view()
//                         }
//                         </ul>
//                     </div>
//                 }
//             }>
//                 {post_view}
//             </ErrorBoundary>
//         </Suspense>
//     }
// }

#[server]
pub async fn get_post(id: usize) -> Result<Option<Post>, ServerFnError> {
    println!("get_post");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    Ok(POSTS.iter().find(|post| post.id == id).cloned())
}