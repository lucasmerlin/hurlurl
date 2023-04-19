use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::Routable;

use imprint::Imprint;

use crate::home::Home;
use crate::info::Info;

mod form;
mod header;
mod home;
mod imprint;
mod info;
mod permanent_redirect_checkbox;
mod total_stats;
mod use_fetch;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/info/:link")]
    Link { link: String },
    #[at("/imprint")]
    Imprint,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home/> },
        Route::Link { link } => html! {<Info link={link.clone()}/>},
        Route::Imprint {} => html! {<Imprint/>},
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
