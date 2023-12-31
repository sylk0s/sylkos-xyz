#![allow(non_snake_case)]

use dioxus::{prelude::*, html::GlobalAttributes};
use dioxus_router::prelude::*;
use sylkos_xyz::Route;

fn main() {
    dioxus_web::launch(App);
}

pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            display: "flex",
            position: "absolute",
            top: "0",
            left: "0",
            height: "100%",
            width: "100%",

            background: "#181825",
            color: "#f5c2e7",
            font_family: "monospace",

            Router::<Route> {}
        }
    })
}