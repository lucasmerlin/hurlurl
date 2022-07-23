use futures_util::TryFutureExt;
use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use wasm_bindgen_futures::spawn_local;
use yew::{use_effect_with_deps, use_state};
use shared::LinkDto;

pub fn use_fetch(url: &str) -> Option<LinkDto> {
    let data = use_state::<Option<LinkDto>, _>(|| None);

    let cloned_data = data.clone();
    use_effect_with_deps(|link| {
        let link = link[0].clone();
        spawn_local(
            async move {
                let result = fetch::<LinkDto>(&link).await;
                cloned_data.set(result.ok());
            }
        );
        || {}
    }, [url.to_string()]);

    (*data).clone()
}


/// You can use reqwest or other crates to fetch your api.
async fn fetch<T>(url: &str) -> Result<T, String>
    where
        T: DeserializeOwned,
{
    Request::get(url).send()
        .and_then(|response| async move { response.json().await }).await
        .map_err(|error| error.to_string())
}
