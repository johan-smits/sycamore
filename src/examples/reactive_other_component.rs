use sycamore::prelude::*;

#[derive(Prop)]
pub(crate) struct Props<'a> {
    input: &'a Signal<String>,
}

#[component]
pub(crate) async fn OtherComponent<'a, G: Html>(cx: Scope<'a>, props: Props<'a>) -> View<G> {
    view! { cx,
        (
            if !props.input.get().is_empty() {
                view! { cx, (props.input.get())}
            } else {
                view! { cx, "Type in the input box"}
            }
        )
    }
}