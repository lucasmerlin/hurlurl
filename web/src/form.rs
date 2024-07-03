use crate::permanent_redirect_checkbox::PermanentRedirectCheckbox;
use crate::Route;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use shared::{CreateLinkDto, CreateResult, CreateTargetDto};
use validator::Validate;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Event, HtmlInputElement};
use yew::{function_component, html, use_state, Callback, Html};
use yew_router::hooks::use_navigator;

#[derive(Serialize, Deserialize)]
struct PlausibleProps {
    permanent_redirect: bool,
    targets: usize,
}

#[wasm_bindgen]
extern "C" {
    fn plausible(s: &str, props: JsValue);
}

#[function_component(Form)]
pub fn form() -> Html {
    let targets = use_state::<Vec<CreateTargetDto>, _>(|| vec![]);

    let plausible_event = |props: PlausibleProps| {
        plausible(
            "Create hurlurl",
            serde_wasm_bindgen::to_value(&props).unwrap(),
        );
    };

    let permanent_redirect = use_state(|| false);

    let errors = targets
        .iter()
        .map(|target| {
            if let Ok(_) = target.validate() {
                None
            } else {
                Some("Invalid URL")
            }
        })
        .collect::<Vec<_>>();

    let has_error = errors.iter().find(|v| v.is_some()).is_some();

    let navigator = use_navigator().unwrap();

    let add_target = {
        let targets = targets.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();
            let input = target.dyn_ref::<HtmlInputElement>().unwrap();
            let value = input.value();

            input.set_value("");

            let mut targets_clone = (*targets).clone();
            targets_clone.push(CreateTargetDto { target_url: value });
            targets.set(targets_clone);
        })
    };

    let on_target_change = {
        let targets = targets.clone();
        Callback::from(move |(e, i): (Event, usize)| {
            let value = e
                .target()
                .unwrap()
                .dyn_ref::<HtmlInputElement>()
                .unwrap()
                .value();

            let mut targets_clone = (*targets).clone();
            targets_clone[i].target_url = value;
            targets.set(targets_clone);
        })
    };

    let create_link = {
        let targets = targets.clone();
        let navigator = navigator.clone();
        let permanent_redirect = permanent_redirect.clone();
        Callback::from(move |_| {
            let targets = targets.clone();
            let navigator = navigator.clone();
            let permanent_redirect = permanent_redirect.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::post("/api/links")
                    .header("Content-Type", "application/json")
                    .json(&CreateLinkDto {
                        url: None,
                        permanent_redirect: *permanent_redirect,
                        targets: (*targets).clone(),
                    })
                    .unwrap()
                    .send()
                    .await
                    .unwrap()
                    .json::<CreateResult>()
                    .await
                    .unwrap();

                match response {
                    CreateResult::Link(link) => {
                        navigator.push(&Route::Link {
                            link: link.link.url,
                        });
                    }
                    CreateResult::StripeRedirect(url) => {
                        web_sys::window()
                            .unwrap()
                            .location()
                            .set_href(&url)
                            .unwrap();
                    }
                }

                plausible_event(PlausibleProps {
                    permanent_redirect: *permanent_redirect,
                    targets: targets.len(),
                })
            });
        })
    };

    let redirect_click = {
        let permanent_redirect = permanent_redirect.clone();
        Callback::from(move |_| {
            permanent_redirect.set(!*permanent_redirect);
        })
    };

    html! {
        <>
            { targets.iter().enumerate().map(|(i, target)| {
                let on_target_change = on_target_change.clone();
                html! {
                    <div class="form-control">
                        <input
                            autofocus={true}
                            type="text"
                            placeholder="Enter URLs"
                            class={format!("input input-bordered {}", errors[i].map(|_e| "input-error").unwrap_or(""))}
                            value={Some(target.target_url.clone())}
                            onchange={move |e| {on_target_change.emit((e, i));}}
                        />
                        if let Some(error) = errors[i] {
                            <label class="label">
                                <span class="label-text-alt text-error">{error}</span>
                            </label>
                        }
                    </div>
                }
            }).collect::<Html>() }


            <div class="form-control">
                <input type="text" placeholder="Enter URLs" class="input input-bordered" onchange={add_target.clone()} />
            </div>

            <PermanentRedirectCheckbox on_click={redirect_click} checked={*permanent_redirect} disabled={false} />

            <div>
                {"You will need to pay 1€ via Stripe. (No signup required!) "}
                <label for="paid-dialog" class="cursor-pointer link">
                    {"Why?"}
                </label>
            </div>

            <input type="checkbox" id="paid-dialog" class="modal-toggle" />
            <div class="modal">
                <div class="modal-box">
                    <h3 class="font-bold text-lg">{"Why is hurlurl paid?"}</h3>
                    <p class="py-4">{r#"
                        hurlurl was used by a lot of spammers and scammers.
                        To prevent abuse, we charge a small fee to create a hurlurl.
                        This fee is used to pay for the infrastructure and to prevent abuse.

                        YouTube urls are whitelisted, so you can freely create YouTube hurlurls.
                    "#}</p>
                    <p class="py-4">
                        {r#"
                            If you need to create a lot of different hurlurls for a certain domain,
                            reach out to me at
                        "#}
                        <a href="mailto:lucas@merlins.media" class="link">{"lucas@merlins.media"}</a>
                       {r#"
                            and I can whitelist the domain.
                        "#}
                    </p>
                    <p class="py-4">
                        {r#"
                            You can also fork the project on github
                            to host your own version:
                        "#}
                        <a href="https://github.com/lucasmerlin/hurlurl" class="link">{
                            "https://github.com/lucasmerlin/hurlurl"
                        }</a>
                    </p>
                    <div class="modal-action">
                        <label for="paid-dialog" class="btn">{"Ok!"}</label>
                    </div>
                </div>
            </div>

            <div class="form-control">
                <button class="btn btn-primary" onclick={create_link} disabled={has_error || targets.len() == 0}>{ "Create hurlurl" }</button>
            </div>
        </>
    }
}
