use sycamore::prelude::*;

#[component]
pub(crate) async fn Svg<'a, G: Html>(cx: Scope<'a>) -> View<G> {
    view! {cx,
        svg(xmlns="http://www.w3.org/2000/svg", aria-label="Flickr", role="img", viewbox="0 0 512 512") {
            rect(width="512", height="512", rx="15%", fill="#fff") {}
            circle(cx="157", cy="256", fill="#0063dc", r="79") {}
            circle(cx="355", cy="256", fill="#ff0084", r="79") {}
        }
    }
}