pub mod home;
pub mod about;
pub mod blog;
pub mod testing;

use dioxus::prelude::*;
use crate::components::stars::Stars;

#[component]
pub fn NotFound(cx: Scope, route: Vec<String>) -> Element {
    let sroute = route.join("/");
    cx.render(rsx! {
        div {
            "404 {sroute}"
        }
    })
}

pub fn Cat(cx: Scope) -> Element {
    cx.render(rsx! {
        Stars {

        }
        div {
            class: "absolute flex h-screen w-screen justify-center items-center",
            div {
                class: "flex flex-col p-10 bg-crust text-text text-4xl rounded-lg justify-center items-center", 
                white_space: "pre-wrap",
                h1 {
                "       |\\__/,|   (`\\" br {}
                "    _.|o o  |_   ) )  " br {}
                "    -(((---(((-------- \n" br {}
                }
            }
        }
    })
}

pub fn Projects(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Projects"
        }
    })
}

pub fn Wip(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            padding: "10px",
            h1 { "This page is under construction... come back later!" }
        }
    })
}

pub fn Celeste(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "add the art thingy here :P"
        }
    })
}