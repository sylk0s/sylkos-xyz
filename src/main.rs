#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use sylkos_xyz::Route;

fn main() {
    dioxus_web::launch(App);
}

pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Router::<Route> {}
    })
}