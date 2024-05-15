use dioxus::prelude::*;
use std::fmt;
use crate::{
    Route,
    components::{markdown::Markdown, stars::Stars},
};
use dioxus_router::prelude::*;

#[derive(Props, Clone, Debug)]
pub struct DocumentProps<'a> {
    children: Element<'a>,
}

pub fn Document<'a>(cx: Scope<'a, DocumentProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        Stars {

        }
        div { // bg
            class: "absolute flex h-screen w-screen justify-center",

            div { // center text column
                    class: "container mx-auto p-2 max-w-4xl",
                    
                    div { // header padding
                        class: "pb-4",
                        div { // header bar
                            class: "flex bg-base rounded-lg p-2",
                            
                            // header content

                            h1 {
                                class: "text-xl text-left text-rosewater",
                                Link {
                                    to: Route::Home {},
                                    "⇦ Return Home"
                                }
                            }
                        }
                    }

                    div {
                        class: "flex flex-col bg-base rounded-lg p-2",
        
                        // document content

                        cx.props.children.clone()
                    }
            }
        }
    })
}