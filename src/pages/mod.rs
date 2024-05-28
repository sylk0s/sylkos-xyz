pub mod home;
pub mod blog;
pub mod links;

use dioxus::prelude::*;
use crate::components::{
    stars::Stars,
    document::Document,
    markdown::Markdown,
};

#[component]
pub fn Cat(route: Vec<String>) -> Element {
    let sroute = route.join("/");
    rsx! {
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
    }
}

pub fn About() -> Element {
    rsx! {
        Document {
            Markdown {
                content: include_str!("../../assets/pages/about.md")
            }
        }
    }
}

pub fn Contact() -> Element {
    rsx! {
        Document {
            Markdown {
                content: include_str!("../../assets/pages/contact.md")
            }
        }
    }
}