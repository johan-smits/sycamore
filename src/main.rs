use sycamore::prelude::*;
use sycamore::suspense::Suspense;

mod examples;

#[component]
async fn App<G: Html>(cx: Scope<'_>) -> View<G> {
    let input = create_signal(cx, String::new());

    view! { cx,
        // Example Other component reactive
        h1 {"Reactive component"}
        p {input(bind:value=input, type="text", placeholder="Type here")}
        examples::OtherComponent(input=input)

        hr {}

        // Example fetch new joke and show the joke
        h1 {"Fetch on button action"}
        examples::UpdateFetchInComponent

        hr {}

        // Component with sub item
        h1 {"Sub component"}
        examples::DrawerOverlay {
            div(class="w-full h-full flex flex-col items-center") {
                examples::TitleBar
                div {
                    "Home!"
                }
            }
        }

    }
}

fn main() {
    sycamore::render(|cx| {
        view! { cx,
            Suspense(fallback=view! { cx, "Loading any async event..." }) {
                App {}
            }
        }
    });
}
