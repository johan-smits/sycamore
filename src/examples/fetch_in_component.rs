use sycamore::prelude::*;
use reqwasm::http::Request;
use serde::{Deserialize, Serialize};

const API_BASE_URL: &str = "https://official-joke-api.appspot.com/random_joke";


async fn fetch_joke() -> Result<Joke, reqwasm::Error> {
    let resp = Request::get(API_BASE_URL).send().await?;

    let body = resp.json::<Joke>().await?;
    Ok(body)
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub(crate) struct Joke {
    id: i32,
    setup: String,
    punchline: String,
}

#[component]
pub(crate) async fn UpdateFetchInComponent<G: Html>(cx: Scope<'_>) -> View<G> {
    let new_joke = fetch_joke().await.expect("Unable to retrieve new joke");
    let joke = create_signal(cx, new_joke);

    let on_btn_click = |_| {
        // match fetch_joke().await {
        //     Ok(new_joke) => joke.set(new_joke),
        //     Err(_) => {}
        // }
    };

    view! { cx,
        p {button(on:click=on_btn_click) {"Get new joke"}}
        p {"Joke setup: " (joke.get().setup)}
        p {"Joke punchline: " (joke.get().punchline)}
    }
}