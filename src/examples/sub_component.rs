use sycamore::prelude::*;


#[derive(Prop)]
pub struct Props<'a, G: Html> {
    children: Children<'a, G>,
}

#[component]
pub fn DrawerOverlay<'a, G: Html>(
    cx: Scope<'a>,
    props: Props<'a, G>,
) -> View<G> {
    let children = props.children.call(cx);
    view! { cx,
        (children)
    }
}

#[component]
pub fn TitleBar<G: Html>(
    cx: Scope,
) -> View<G> {
    view! { cx,
        "Title bar"
    }
}
