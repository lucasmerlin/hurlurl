use std::ops::{Deref, DerefMut};

use futures_util::TryFutureExt;
use gloo_net::Error;
use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::function_component;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::Routable;
use yew_router::prelude::*;

use shared::{CreateLinkDto, LinkDto};
use crate::Route;

use crate::use_fetch::use_fetch;

#[derive(Properties, PartialEq)]
pub struct InfoProps {
    pub link: String,
}

#[function_component(Info)]
pub fn info(props: &InfoProps) -> Html {
    let link = props.link.clone();

    let data = use_fetch(&format!("/api/links/{}", link));

    html! {
        <div class="container">

            <Link<Route> to={Route::Home}><h1>{ "hurlurl" }</h1></Link<Route>>

            { if let Some(data) = data.as_ref() {
                let url = format!("/{}", &data.link.url);
                html!{<>
                    <h2> { &data.link.url } {" - clicks: "} {&data.link.redirects} </h2>
                    <b><a href={ url.clone() }>{"https://hurlurl.hellopaint.io"}{ &url }</a></b>

                    <br/>

                    <div>{ "Targets: "}</div>
                    <ul>
                        { for data.targets.iter().map(|target| html! {
                            <li>
                                <b><a href={target.target_url.clone()}>{ &target.target_url }</a></b>
                                <ul>
                                    <li>{"clicks: "} { &target.redirects }</li>
                                </ul>
                            </li>
                        }) }
                    </ul>
                </>}
            } else {
                html!{"Loading..."}
            }}
        </div>
    }
}

