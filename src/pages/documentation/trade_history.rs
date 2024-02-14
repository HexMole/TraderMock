use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[cfg(feature = "ssr")]
use crate::be_services::anvil_service::get_pool_tokens_transaction_history;

use crate::be_services::anvil_service::{ContractsEventsModel,  PoolModel};

#[server]
pub async fn get_deployed_contracts() -> Result<Vec<PoolModel>, ServerFnError> {
    
    let pools_events = get_pool_tokens_transaction_history().await.unwrap();
    Ok(pools_events)
}

#[component]
pub fn TradeHistory() -> impl IntoView {
    let contracts_deployed_rsc = create_resource(|| (), |_| async move { get_deployed_contracts().await });
    view! {
        <H1>"TradeHistory"</H1>
        <Suspense fallback=move || view! { <p>"Loading..."</p> } >
        <Collapsibles default_on_open=OnOpen::CloseOthers>
        <Stack spacing=Size::Em(0.6)>
                    {move || {
                        contracts_deployed_rsc.and_then(|pools| {
                            pools.iter()
                                    .map(|pool| {

                                        let pool_i=pool.clone();
                                        let pool_name=pool.PoolName.clone();
                                        view!{
                                            
                                            <Collapsible>
                                                <CollapsibleHeader slot>{pool_name}</CollapsibleHeader>
                                                <CollapsibleBody class="my-body" slot><RenderPoolModel poolmodel=pool_i /> </CollapsibleBody>
                                            </Collapsible> 
                                        }
                                    }).collect_view()
                            })
                        }
                    }
        </Stack>
        </Collapsibles>

        </Suspense>
    }
}

#[component]
pub fn RenderPoolModel(#[prop(into, optional)] poolmodel:PoolModel) -> impl IntoView {
    view! {
            <Collapsibles default_on_open=OnOpen::DoNothing>
                <Stack spacing=Size::Em(0.6)>
                    <Collapsible>
                        <CollapsibleHeader slot>{poolmodel.Token0Name.clone()}</CollapsibleHeader>
                        <CollapsibleBody class="my-body" slot><Rendertokens tokenEvents=poolmodel.Token0.clone() /></CollapsibleBody>
                    </Collapsible>
                    <Collapsible>
                        <CollapsibleHeader slot>{poolmodel.Token1Name.clone()}</CollapsibleHeader>
                        <CollapsibleBody slot><Rendertokens tokenEvents=poolmodel.Token1.clone() /></CollapsibleBody>
                    </Collapsible>
                </Stack>
            </Collapsibles>

    }
}


#[component]
pub fn Rendertokens(#[prop(into, optional)] tokenEvents:Vec<ContractsEventsModel>) -> impl IntoView {
    view! {
        <TableContainer>
        <Table bordered=true hoverable=true>
            <TableHeader>
                <TableRow>
                    <TableHeaderCell min_width=true>"#"</TableHeaderCell>
                    <TableHeaderCell>"EventType"</TableHeaderCell>
                    <TableHeaderCell>"From"</TableHeaderCell>
                    <TableHeaderCell>"To"</TableHeaderCell>
                </TableRow>
            </TableHeader>
            <TableBody>
                    {move || tokenEvents.iter().map(|toekn_event| 
                    {
                        let eventType = toekn_event.EventType.clone();
                        let from = toekn_event.From.clone();
                        let to = toekn_event.To.clone();
                        view! {
                                <TableRow>
                                    <TableCell>"#"</TableCell>
                                    <TableCell>{eventType}</TableCell>
                                    <TableCell>{from}</TableCell>
                                    <TableCell>{to}</TableCell>
                                </TableRow>
                        }
                    }).collect_view()
                }
            </TableBody>
            </Table>
        </TableContainer>
    }
}