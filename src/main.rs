use sycamore::prelude::*;

#[derive(Prop)]
struct Props<'a> {
    input: &'a Signal<String>,
}

#[component]
fn OtherComponent<'a, G: Html>(cx: Scope<'a>, props: Props<'a>) -> View<G> {
    view! { cx,
        (
            if !props.input.get().is_empty() {
                view! { cx, (props.input.get())}
            } else {
                view! { cx, "Empty"}
            }
        )
    }
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let input = create_signal(cx, String::new());

    view! { cx,
        p {
            input(bind:value=input, type="text", placeholder="Type here")
        }
        OtherComponent(input=input)
    }
}

fn main() {
    sycamore::render(|cx| {
        view! { cx, App {} }
    });
}
