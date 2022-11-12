use web_sys::window;
use yew::function_component;
use yew::prelude::*;

use crate::header::Header;

use shared::LinkDto;

use crate::permanent_redirect_checkbox::PermanentRedirectCheckbox;
use crate::use_fetch::use_fetch;

#[derive(Properties, PartialEq)]
pub struct InfoProps {
    pub link: String,
}

#[function_component(Info)]
pub fn info(props: &InfoProps) -> Html {
    let link = props.link.clone();

    let data = use_fetch::<LinkDto>(&format!("/api/links/{}", link));

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
            <Header link={link.clone()}/>
            <div class="flex items-center justify-center md:bg-base-200 min-h-screen">

                { if let Some(data) = data.as_ref() {
                    let link = format!("https://hurlurl.com/{}", data.link.url);
                    html!{<div class="card bg-base-100 p-8 md:shadow">

                        <h1 class="text-xl md:text-3xl mb-2 flex items-center">
                            <a class="link text-primary" href={link.clone()}>{&link}</a>
                            <button class="ml-2 btn btn-circle btn-ghost opacity-60" onclick={copy_link}>
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 7.5V6.108c0-1.135.845-2.098 1.976-2.192.373-.03.748-.057 1.123-.08M15.75 18H18a2.25 2.25 0 002.25-2.25V6.108c0-1.135-.845-2.098-1.976-2.192a48.424 48.424 0 00-1.123-.08M15.75 18.75v-1.875a3.375 3.375 0 00-3.375-3.375h-1.5a1.125 1.125 0 01-1.125-1.125v-1.5A3.375 3.375 0 006.375 7.5H5.25m11.9-3.664A2.251 2.251 0 0015 2.25h-1.5a2.251 2.251 0 00-2.15 1.586m5.8 0c.065.21.1.433.1.664v.75h-6V4.5c0-.231.035-.454.1-.664M6.75 7.5H4.875c-.621 0-1.125.504-1.125 1.125v12c0 .621.504 1.125 1.125 1.125h9.75c.621 0 1.125-.504 1.125-1.125V16.5a9 9 0 00-9-9z" />
                                </svg>
                            </button>
                        </h1>

                        <PermanentRedirectCheckbox checked={data.link.permanent_redirect} disabled={true} />

                        <h1 class="text-2xl mt-5 mb-2">{"Link stats"}</h1>

                        <div class="stats shadow bg-white">

                            <div class="stat">
                                <a href={link} target="_blank" class="stat-title">{"hurlurl.com/"}{&data.link.url}</a>
                                <div class="stat-value text-primary">{&data.link.redirects}</div>
                                <div class="stat-desc">{"clicks"}</div>
                            </div>

                        </div>

                        <h1 class="text-2xl mt-5 mb-2">{"Target stats"}</h1>

                        <div class="stats stats-vertical shadow bg-white">

                            { for data.targets.iter().map(|target| html! {
                                <div class="stat">
                                    <a href={target.target_url.clone()} target="_blank" class="stat-title">{&target.target_url}</a>
                                    <div class="stat-value text-primary">{&target.redirects}</div>
                                    <div class="stat-desc">{"redirects"}</div>
                                </div>
                            }) }

                        </div>

                        <div class="alert alert-info shadow-lg mt-8">
                            <div>
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-current flex-shrink-0 w-6 h-6"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                                <span>{"This page is accessible by anyone with the link"}</span>
                            </div>
                        </div>

                    </div>}
                } else {
                    html!{"Loading..."}
                }}
            </div>
        </>
    }
}
