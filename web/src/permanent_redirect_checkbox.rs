use yew::html::onclick::Event;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InfoProps {
    pub checked: bool,
    #[prop_or(Callback::noop())]
    pub on_click: Callback<Event>,
    pub disabled: bool,
}

#[function_component(PermanentRedirectCheckbox)]
pub fn permanent_redirect_checkbox(props: &InfoProps) -> Html {
    html! {
        <>
            <div class="form-control flex flex-row align-center">
                <label class="label cursor-pointer justify-start gap-4">
                    <input type="checkbox" checked={props.checked} disabled={props.disabled} onclick={&props.on_click} class="checkbox checkbox-primary" />
                    <span class="label-text">
                        {"Permanent Redirect"}
                    </span>
                </label>
                <label class="flex items-center" for="redirect-explain-dialog">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9 5.25h.008v.008H12v-.008z" />
                    </svg>
                </label>
            </div>
            <input type="checkbox" id="redirect-explain-dialog" class="modal-toggle" />
            <div class="modal">
                <div class="modal-box">
                    <h3 class="font-bold text-lg">{"Permanent Redirect"}</h3>
                    <p class="py-4">{"With a permanent redirect, the same person will always go to the same URL on multiple clicks, otherwise they always will get a new random URL."}</p>
                    <p class="py-4 text-sm opacity-75">{"Please note: With permanent redirect, only the first click of each user will be counted in the statistics."}</p>
                    <div class="modal-action">
                        <label for="redirect-explain-dialog" class="btn">{"Ok!"}</label>
                    </div>
                </div>
            </div>
        </>
    }
}
