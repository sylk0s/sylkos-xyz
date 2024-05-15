pub mod home;
pub mod about;
pub mod blog;

use dioxus::prelude::*;
use crate::components::stars::Stars;

#[component]
pub fn Cat(cx: Scope, route: Vec<String>) -> Element {
    let sroute = route.join("/");
    cx.render(rsx! {
        Stars {

        }
        div {
            class: "absolute flex flex-col h-screen w-screen justify-center items-center",
            h1 {
                class: "text-center text-rosewater text-6xl p-20",
                "404: {sroute}" br {}
                "Enjoy this cat instead! :3"
            }
            div {
                class: "flex flex-col p-10 bg-crust text-text text-4xl rounded-lg justify-center items-center", 
                white_space: "pre-wrap",
                h1 {
                "   |\\__/,|   (`\\" br {}
                "_.|o o  |_   ) )" br {}
                "-(((---(((-------"
                }
            }
        }
    })
}