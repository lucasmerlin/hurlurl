use std::ops::{Deref, DerefMut};

use gloo_net::http::Request;
use regex::Regex;
use serde::Serialize;
use validator::Validate;
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::function_component;
use yew::prelude::*;
use yew_router::hooks::use_history;
use yew_router::Routable;
use yew_router::history::History;
use shared::{CreateLinkDto, CreateTargetDto, LinkDto};
use crate::Route;



#[function_component(Home)]
pub fn home() -> Html {
    let input_ref = use_node_ref();

    let targets = use_state::<Vec<CreateTargetDto>, _>(|| vec![]);

    let error = use_state::<Option<String>, _>(|| None);

    log::info!("{:?}", targets);

    let history = use_history().unwrap();

    let confirm = {
        let input_ref = input_ref.clone();
        let targets = targets.clone();
        let error = error.clone();
        move || {
            let targets = targets.clone();

            if let Some(input) = input_ref.cast::<HtmlInputElement>() {

                let val = input.value();
                let create = CreateTargetDto {
                    target_url: val.clone(),
                };

                if let Err(e) = create.validate() {
                    error.set(Some("Invalid URL".to_string()));
                    return;
                } else {
                    error.set(None);
                }

                let mut clone = (*targets).clone();
                clone.push(CreateTargetDto {
                    target_url: input.value().clone(),
                });
                targets.set(clone);
                input.set_value("");
            }
        }
    };

    let add_target = {
        let confirm = confirm.clone();
        Callback::from(move |_| {
            confirm();
        })
    };

    let onkeyup = {
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                confirm();
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
                        targets: (*targets).clone(),
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
        <div class="container">
            <h1>{ "hurlurl" }</h1>
            <p>{ "hurlurl is a load balancing link shortening service." }</p>

            <p>{ "A hurlurl takes a list of links and randomly forwards to one of them."}</p>

            <p>{ "Enter the URLs to shorten here:" }</p>

            { targets.iter().map(|target| html! {
                <a class="target-item" href={target.target_url.clone()} target="_blank">{ &target.target_url }</a>
            }).collect::<Html>() }

            <div class="add_target">

                <div>
                    <input ref={input_ref} {onkeyup} label="Target URL" placeholder="enter link" />
                    { error.iter().map(|error| html! {
                        <div class="error">{ error }</div>
                    }).collect::<Html>() }
                </div>
                <button onclick={add_target}>{ "+" }</button>

            </div>
            <button class="primary" onclick={create_link}>{ "create hurlurl" }</button>

            <hr/>

            <div class="info">
                <p>{ "hurlurl is a open source website written in rust" }</p>
                <iframe class="gh-button" src="https://ghbtns.com/github-btn.html?user=lucasmerlin&repo=urllb&type=star&count=true&size=large" frameborder="0" scrolling="0" width="170" height="32" title="GitHub"></iframe>
            </div>
        </div>
    }
}
