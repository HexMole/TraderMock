use leptonic::prelude::*;
use leptos::*;

use crate::pages::documentation::doc_root::DocRoutes;

#[component]
pub fn PageOverview() -> impl IntoView {
    view! {
        <H1>"Overview"</H1>

        <P>
            "Uni Vizualizer is a tool to visualize Uniswap contract, transactions and history.  "<LinkExt href="https://leptos.dev/" target=LinkExtTarget::Blank>"Leptos"</LinkExt>" web framework."
        </P>

        <P>
            "It consist of two parts, client and server."
        </P>

        <P>
            "If you want to dive right in, follow our " <Link href=DocRoutes::Installation>"Installation"</Link> " instructions."
        </P>
    }
}
