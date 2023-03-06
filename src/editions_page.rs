use dioxus::prelude::*;

pub fn page(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            h1 { "Reading blog post:" }
            button { class: "primary", "my button" }
            button { class: "accent",  "secondary" }
            button { class: "warn", "Delete" }
            p { "example blog post..." }
        }
    })
}
