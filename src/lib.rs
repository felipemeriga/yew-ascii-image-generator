mod components;
mod pages;
mod router;
mod utils;

use crate::router::{switch, Route};
use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Switch};

#[function_component]
pub fn App() -> Html {
    wasm_logger::init(wasm_logger::Config::default());

    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
