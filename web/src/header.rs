use yew::{function_component, html, Html, Properties};

use yew_router::prelude::*;

use crate::Route;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub link: Option<String>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    html! {
        <div class="navbar bg-base-100">
            <Link<Route> to={Route::Home} classes="btn btn-ghost normal-case text-xl">
                {"hurlurl"}
                if let Some(link) = &props.link {
                    <span class="opacity-50">
                        <span class="mx-1">
                            {"/"}
                        </span>
                        {link}
                    </span>
                }
            </Link<Route>>
        </div>
    }
}
