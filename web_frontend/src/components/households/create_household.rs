use super::Household;
use gloo_net::http::Request;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub afterclbk: Callback<Household>,
}

#[function_component(CreateHousehold)]
pub fn create_household(props: &Props) -> Html {
    let callbk = props.afterclbk.clone();

    let address = use_state(String::new);
    // Nimon

    let add_household: UseAsyncHandle<Household, ()> = {
        let address = address.clone();
        use_async(async move {
            let household = add(address.to_string()).await;
            callbk.emit(household.clone());
            Ok(household)
        })
    };

    let onkeypress = {
        let address = address.clone();
        let add_household = add_household.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() != "Enter" {
                return;
            }

            let input: HtmlInputElement = e.target_unchecked_into();
            address.set(input.value());
            input.set_value("");
            add_household.run();
        })
    };

    html! {
        <div>
            <input {onkeypress} />
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct CreateData {
    address: String,
}

async fn add(address: String) -> Household {
    let create = CreateData { address };
    Request::post("api/households")
        .json(&create)
        .unwrap()
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}
