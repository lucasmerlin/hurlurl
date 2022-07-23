mod info;
mod home;
mod use_fetch;

use std::ops::{Deref, DerefMut};

use gloo_net::http::Request;
use serde::Serialize;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::Routable;
use shared::{CreateLinkDto, LinkDto};
use crate::home::Home;

use yew_router::prelude::*;
use crate::info::Info;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/info/:link")]
    Link { link: String },
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home/> },
        Route::Link { link } => html! {<Info link={link.clone()}/>},
    }
}


#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<App>();
}
