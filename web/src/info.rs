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

use shared::{CreateLinkDto, LinkDto};

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
        <>
            <h1>{ "UrlLB" }</h1>
            { if let Some(data) = data.as_ref() {
                html!{<>
                    <h2> { &data.link.url } </h2>
                    <a href={ format!("/{}", &data.link.url) }>{ "Try me" }</a>
                    <ul>
                        { for data.targets.iter().map(|target| html! {
                            <li>
                                <b>{ &target.target_url }</b>
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
        </>
    }
}

