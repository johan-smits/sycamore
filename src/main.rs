use std::rc::Rc;
use sycamore::prelude::*;

#[derive(Prop)]
struct Props {
    input: RcSignal<String>,
}

#[component]
fn OtherComponent<G: Html>(cx: Scope, props: Props) -> View<G> {


    view! { cx,
        p {
            (props.input)
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
        OtherComponent(input=input.get())
    }
}

fn main() {
    sycamore::render(|cx| {
        view! { cx, App {} }
    });
}
