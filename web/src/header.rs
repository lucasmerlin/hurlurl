use yew::{function_component, html};

use yew_router::prelude::*;

use crate::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div class="navbar bg-base-100">
            <Link<Route> to={Route::Home} classes="btn btn-ghost normal-case text-xl">{"hurlurl"}</Link<Route>>
        </div>
    }
}
