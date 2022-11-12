use yew::function_component;
use yew::prelude::*;

use yew_router::prelude::*;

use crate::form::Form;
use crate::header::Header;
use crate::Route;
use crate::total_stats::TotalStats;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="min-h-screen flex flex-col">
            <Header/>
            <div class="hero bg-base-200 flex-grow">
                <div class="hero-content flex-col lg:flex-row-reverse gap-8 lg:gap-24">
                    <div class="text-center lg:text-left">
                        <div><h1 class="text-5xl font-bold">{ "🌪 hurlurl" }</h1></div>
                        <p class="py-6">{ "hurlurl is a load balancing link shortening service. A hurlurl takes a list of links and randomly forwards to one of them." }</p>
                        <TotalStats/>
                    </div>
                    <div class="card flex-shrink-0 w-full max-w-sm shadow-2xl bg-base-100">
                        <div class="card-body">
                            <Form/>
                        </div>
                    </div>
                </div>
            </div>

            <div>
                <footer class="footer items-center p-4 bg-neutral text-neutral-content">
                    <div class="items-center grid-flow-col">
                        <p>
                            {"hurlurl is a open source website written in rust by "}
                            <a class="link" href="https://lucasmerlin.me/">{"lucasmerlin"}</a>
                            {"."}
                        </p>
                    </div>
                    <div class="grid-flow-col gap-4 place-self-center justify-self-end">
                        <p>
                            <a href="https://audaxly.com/privacy-policy?code=la5zxvgcxh4hyf">{"Privacy"}</a>
                        </p>
                        <p>
                            <Link<Route> to={Route::Imprint}>{"Imprint / Legal Notice"}</Link<Route>>
                        </p>
                         <a class="github-button" href="https://github.com/lucasmerlin/hurlurl" data-size="large" aria-label="Star hurlurl on Github">{"Star on Github"}</a>
                    </div>
                </footer>
            </div>
        </div>
    }
}
