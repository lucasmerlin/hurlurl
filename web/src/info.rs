use std::ops::{Deref, DerefMut};

use futures_util::TryFutureExt;
use gloo_net::Error;
use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, window};
use yew::function_component;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::Routable;
use yew_router::prelude::*;

use shared::{CreateLinkDto, LinkDto};
use crate::Route;
use crate::header::Header;

use crate::use_fetch::use_fetch;
use crate::permanent_redirect_checkbox::PermanentRedirectCheckbox;


#[derive(Properties, PartialEq)]
pub struct InfoProps {
    pub link: String,
}

#[function_component(Info)]
pub fn info(props: &InfoProps) -> Html {
    let link = props.link.clone();

    let data = use_fetch(&format!("/api/links/{}", link));

    let copy_link = {
        let link = link.clone();
        Callback::from(move |_| {
            if let Some(clipboard) = window().unwrap().navigator().clipboard() {
                clipboard.write_text(&format!("https://hurlurl.com/{}", link));
            }
        })
    };

    html! {
        <>
            <Header/>
            <div class="flex items-center justify-center">

                { if let Some(data) = data.as_ref() {
                    let url = format!("/{}", &data.link.url);
                    html!{<div>

                        <h1 class="text-5xl my-5">{"Stats"}</h1>

                        <PermanentRedirectCheckbox checked={data.link.permanent_redirect} disabled={true} />

                        <button onclick={copy_link} class="btn m-4">{"Copy link"}</button><br/>

                        <div class="stats shadow">

                            <div class="stat">
                                <a href={format!("https://hurlurl.com/{}", data.link.url)} target="_blank" class="stat-title">{"hurlurl.com/"}{&data.link.url}</a>
                                <div class="stat-value text-primary">{&data.link.redirects}</div>
                                <div class="stat-desc">{"clicks"}</div>
                            </div>

                        </div>

                        <h1 class="text-2xl my-5">{"Target link stats"}</h1>

                        <div class="stats stats-vertical shadow">

                            { for data.targets.iter().map(|target| html! {
                                <div class="stat">
                                    <a href={target.target_url.clone()} target="_blank" class="stat-title">{&target.target_url}</a>
                                    <div class="stat-value text-primary">{&target.redirects}</div>
                                    <div class="stat-desc">{"redirects"}</div>
                                </div>
                            }) }

                        </div>

                    </div>}
                } else {
                    html!{"Loading..."}
                }}
            </div>
        </>
    }
}

