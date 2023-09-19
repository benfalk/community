use gloo_net::http::Request;
use yew::prelude::*;
use yew_hooks::prelude::*;

mod create_household;
use create_household::CreateHousehold;

#[function_component(Households)]
pub fn households() -> Html {
    let state: UseAsyncHandle<PagedResults<Household>, ()> =
        use_async(async move { Ok(get("api/households").await) });

    let run_once = state.clone();
    use_effect_once(move || {
        run_once.run();
        || {}
    });

    let afterclbk = {
        let update_list = state.clone();
        Callback::from(move |_|{
            update_list.run();
        })
    };

    html! {
        <div class={"container"}>
            <div class={"row"}>
                <div class={"col"}>
                    <CreateHousehold {afterclbk} />
                </div>
            </div>
            <div class={"row"}>
            {
                match &state.data {
                    Some(data) => html! {
                        <div class={"col"}>
                            <ul class={"list-group"}>
                            {
                                data.results.iter().map(|r| {
                                    html! { <li class={"list-group-item list-group-item-light"}>{ r.address.clone() }</li> }
                                }).collect::<Html>()
                            }
                            </ul>
                        </div>
                    },
                    None => html!{ "Loading..." }
                }
            }
            </div>
        </div>
    }
}

#[derive(Debug, serde::Deserialize, Clone, PartialEq)]
pub struct Household {
    id: i64,
    address: String,
}

#[derive(Debug, serde::Deserialize, Clone)]
struct PagedResults<T> {
    results: Vec<T>,
    has_next_page: bool,
}

async fn get<T>(uri: &str) -> T
where
    T: for<'de> serde::Deserialize<'de>,
{
    Request::get(uri)
        .send()
        .await
        .unwrap()
        .json::<T>()
        .await
        .unwrap()
}
