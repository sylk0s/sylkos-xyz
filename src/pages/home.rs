use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{*, components::stars::Stars};
use serde::{Serialize, Deserialize};

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        Stars {

        }

        div {
            class: "absolute flex h-screen w-screen justify-center items-center",

            Index {
            
            }
        }
    })
}

pub fn Index(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex flex-col justify-center items-center bg-base rounded-lg p-1 max-w-md borders border-rosewater border-2",

            h1 {
                class: "text-4xl text-center text-rosewater p-4",
            "Julia Keadey    [sylkos]"
            }

            Entry {
                entry: EntryObj {
                    to: Route::About {},
                    text: "About me!".to_string(),
                }
            }
            Entry {
                entry: EntryObj {
                    to: Route::Blog {},
                    text: "Blog :3".to_string(),
                }
            
            }
        }
    })
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EntryObj {
    to: Route,
    #[serde(default)]
    text: String,
}

#[component]
fn Entry(cx: Scope, entry: EntryObj) -> Element {
    cx.render(rsx! {
        div {
            class: "flex text-lavender p-2 text-4md hover:text-rosewater cursor-pointer",
            Link {
                to: entry.to.clone(),
                format!("~ {}", entry.text.clone())
            }
        }
    })
}