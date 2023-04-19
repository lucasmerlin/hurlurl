use web_sys::window;
use yew::{function_component, html, Html};

#[function_component(Imprint)]
pub fn imprint() -> Html {
    // if you deploy hurlurl to your own domain you can update the imprint / legal notice here
    let is_hurlurl = window().unwrap().location().hostname().unwrap() == "hurlurl.com";

    html! {
        <div>
            <div class="card max-w-lg bg-base-200 shadow-xl mx-auto mt-32" style="white-space: pre-wrap">
                <div class="card-body">
                    <h2 class="card-title">{"hurlurl - Imprint"}</h2>
                    if is_hurlurl {
                        <p>
                        {"
    Lucas Meurer

    Meldaustraße 17

    30419 Hannover

    Germany

    Email: lucas@merlins.media

    Umsatzsteuer ID: DE 325629806
                        "}
                        </p>
                    }
                </div>
            </div>
        </div>
    }
}
