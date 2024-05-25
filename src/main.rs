#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use sylkos_xyz::Route;

// Urls are relative to your Cargo.toml file
// const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    launch(App);
}

pub fn App() -> Element {
    rsx! {
            Router::<Route> {}
    }
}
