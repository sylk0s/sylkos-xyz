use dioxus::prelude::*;
use std::fmt;
use crate::components::markdown::Markdown;

pub fn Testing(cx: Scope) -> Element {

    cx.render(rsx! {
        div { // background of everything
            class: "min-h-screen bg-base",
            div { // center text column
                    class: "container mx-auto p-4 max-w-4xl",
                    script {
                        "hljs.highlightAll();"
                    }

                    div {
                        class: "flex flex-col bg-crust rounded-lg",
        
                        div {
                            class: "flex flex-col p-2",
                            h2 {
                                class: "text-3xl text-left text-rosewater p-1",
                                "Entry Title"
                            }
        
                            p {
                                class: "text-left text-overlay0 ",
                                "01/01/1970"
                            }
                        }
        
                        Markdown {
                            content: include_str!("../../blog.md")
                        }
                    }
            }
        }
    })
}