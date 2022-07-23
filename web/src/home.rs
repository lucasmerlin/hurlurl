use std::ops::{Deref, DerefMut};

use gloo_net::http::Request;
use serde::Serialize;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::function_component;
use yew::prelude::*;
use yew_router::hooks::use_history;
use yew_router::Routable;
use yew_router::history::History;
use shared::{CreateLinkDto, LinkDto};
use crate::Route;


#[derive(Clone, Eq, PartialEq, Debug)]
struct Target {
    target_url: String,
}


#[function_component(Home)]
pub fn home() -> Html {
    let targets = use_state::<Vec<Target>, _>(|| vec![]);

    log::info!("{:?}", targets);

    let history = use_history().unwrap();

    let onkeyup = {
        let mut copy = targets.clone();
        Callback::from(move |e: KeyboardEvent| {
            let target = e.target_unchecked_into::<HtmlInputElement>();

            let len = copy.len();
            let mut clone = (*copy).clone();

            if e.key() == "Enter" {
                clone.push(Target {
                    target_url: target.value(),
                });
                target.set_value("");
                copy.set(clone);
            }
        })
    };

    let create_link = {
        let targets = targets.clone();
        let history = history.clone();
        Callback::from(move |_| {
            let targets = targets.clone();
            let history = history.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::post("/api/links")
                    .header("Content-Type", "application/json")
                    .json(&CreateLinkDto {
                        url: None,
                        permanent_redirect: false,
                        targets: targets.iter().map(|target| target.target_url.clone()).collect(),
                    }).unwrap()
                    .send()
                    .await
                    .unwrap()
                    .json::<LinkDto>()
                    .await
                    .unwrap();

                history.push(Route::Link {link: response.link.url});
            });
        })
    };

    html! {
        <>
            <h1>{ "UrlLB" }</h1>
            <p>{ "UrlLB is a load balancing link shortening service." }</p>


            { targets.iter().map(|target| html! {
                <p>{ &target.target_url }</p>
            }).collect::<Html>() }


            <input {onkeyup} label="Target URL" />

            <button onclick={create_link}>{ "Create Link" }</button>

        </>
    }
}
