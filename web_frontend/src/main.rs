#![allow(dead_code)]

mod routes;
mod components;

use yew::prelude::*;
use yew_router::prelude::*;
use routes::Route;
use components::Households;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Households /> },
        Route::NotFound => html! {
            <h1>{ "Not Found!!!" }</h1>
        },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
