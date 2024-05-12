#![allow(non_snake_case)]

use dioxus::{prelude::*, html::GlobalAttributes};
use dioxus_router::prelude::*;
use sylkos_xyz::Route;

// Urls are relative to your Cargo.toml file
const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    dioxus_web::launch(App);
}

pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "mocha",
            Router::<Route> {}
        }
    })
}
