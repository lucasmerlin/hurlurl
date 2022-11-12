use crate::permanent_redirect_checkbox::PermanentRedirectCheckbox;
use crate::Route;
use gloo_net::http::Request;
use shared::{CreateLinkDto, CreateTargetDto, LinkDto};
use validator::Validate;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};
use yew::{function_component, html, use_state, Callback, Html};
use yew_router::history::History;
use yew_router::hooks::use_history;

#[function_component(Form)]
pub fn form() -> Html {
    let targets = use_state::<Vec<CreateTargetDto>, _>(|| vec![]);

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

    log::info!("{:?}", targets);

    let history = use_history().unwrap();

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
        let history = history.clone();
        let permanent_redirect = permanent_redirect.clone();
        Callback::from(move |_| {
            let targets = targets.clone();
            let history = history.clone();
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
                    .json::<LinkDto>()
                    .await
                    .unwrap();

                history.push(Route::Link {
                    link: response.link.url,
                });
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

            <div class="form-control">
                <button class="btn btn-primary" onclick={create_link} disabled={has_error || targets.len() == 0}>{ "Create hurlurl" }</button>
            </div>
        </>
    }
}
