#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::*;
use serde::{Serialize, Deserialize};

pub fn Index(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            vertical_align: "middle",

            background_color: "#f5c2e7",
            border: "4px solid #585b70",    
            border_radius: "10px",
            padding: "10px",

            height: "25%",
            width: "25%",
            color: "#181825",
            hr {}
            h1 {
            "Julia Keadey    [sylkos]"
            }
            hr {}
            Entry {
                entry: EntryObj {
                    to: Route::About {},
                    text: "About me!".to_string(),
                }
            }
            Entry {
                entry: EntryObj {
                    to: Route::Cat {},
                    text: "Cats.".to_string(),
                }
            }
            Entry {
                entry: EntryObj {
                    to: Route::Stars {},
                    text: "Stars.".to_string(),
                }
            }
            Entry {
                entry: EntryObj {
                    to: Route::Wip {},
                    text: "Projects!".to_string(),
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
            Link {
                to: entry.to.clone(),
                entry.text.clone()
            }
        }
    })
}