use sycamore::prelude::*;

#[derive(Prop)]
struct Props<'a> {
        input: &'a Signal<String>,
    }

#[component]
fn OtherComponent<'a, G: Html>(cx: Scope<'a>, props: Props<'a>) -> View<G> {

    if props.input.get().is_empty() {
        view! { cx, "Empty" }
    } else {
        view! { cx,
            p {
                (props.input.get())
            }
        }
    }
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let input = create_signal(cx, String::new());

    view! { cx,
        p {
            input(bind:value=input, type="email")
        }
        OtherComponent(input=input)
    }
}

fn main() {
    sycamore::render(|cx| {
        view! { cx, App {} }
    });
}
