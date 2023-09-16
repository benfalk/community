use yew::prelude::*;
use yew_router::prelude::*;
mod router;

use router::Route;

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
        Route::Home => html! {
            <h1>{ "Hello World!!" }</h1>
        },
        Route::NotFound => html! {
            <h1>{ "Not Found Bro" }</h1>
        },
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
