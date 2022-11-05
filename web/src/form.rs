use gloo_net::http::Request;
use web_sys::{Event, HtmlInputElement, KeyboardEvent};
use yew::{Callback, function_component, Html, html, use_node_ref, use_state};
use yew_router::hooks::use_history;
use shared::{CreateLinkDto, CreateTargetDto, LinkDto};
use crate::Route;
use validator::Validate;
use wasm_bindgen::JsCast;
use yew_router::history::History;

#[function_component(Form)]
pub fn form() -> Html {

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
        let mut targets = targets.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap();
            let input = target.dyn_ref::<HtmlInputElement>().unwrap();
            let value = input.value();

            input.set_value("");

            let mut targets_clone = (*targets).clone();
            targets_clone.push(CreateTargetDto {
                target_url: value,
            });
            targets.set(targets_clone);
        })
    };

    let onkeyup = {
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                confirm();
            }
        })
    };

    let on_target_change = {
        let mut targets = targets.clone();
        Callback::from(move |(e, i): (Event, usize)| {
            let value = e.target().unwrap().dyn_ref::<HtmlInputElement>().unwrap().value();

            let mut targets_clone = (*targets).clone();
            targets_clone[i].target_url = value;
            targets.set(targets_clone);

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

                history.push(Route::Link { link: response.link.url });
            });
        })
    };

    html! {
        <>
            { targets.iter().enumerate().map(|(i, target)| {

                let on_target_change = on_target_change.clone();

                html! {

                    <div class="form-control">
                        <input autofocus={true} type="text" placeholder="Enter URLs" class="input input-bordered" value={Some(target.target_url.clone())} onchange={move |e| {on_target_change.emit((e, i));}} />
                    </div>

                }
            }).collect::<Html>() }


            <div class="form-control">
                <input type="text" placeholder="Enter URLs" class="input input-bordered" onchange={add_target.clone()} />
            </div>

            <div class="form-control mt-6">
                <button class="btn btn-primary" onclick={create_link}>{ "Create hurlurl" }</button>
            </div>
        </>
    }

}
