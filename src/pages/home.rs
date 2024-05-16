use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{
    *, 
    components::{
        stars::Stars,
        transtext::TransText,
    },
};
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

static JULIA_PATH: &str = "julia.png";

pub fn Index(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex flex-col justify-center bg-base rounded-lg p-1 max-w-lg borders border-rosewater border-2",

            div {
                class: "flex flex-row p-4 justify-center",

                img {
                    class: "rounded-full h-24 w-24",
                    src: "sylkos.png",
                }

                div {
                    class: "flex flex-col justify-center",
                    h1 {
                        class: "text-4xl text-center text-rosewater px-4 py-2",
                    "Julia Keadey" br{}
                    "[sylkos]"
                    }
                    h2 {
                        class: "text-md justify-center",
                        TransText {
                            text: "they/she".to_string()
                        }
                    }
                }

                img {
                    class: "rounded-full h-24 w-24",
                    src: "julia.png",
                }
            }

            hr { class: "height", }

            p {
                class: "text-center text-lg text-rosewater p-4",
                "Hello there! Welcome to my corner of the internet :3"
            }

            ButtonGrid { }

            // then the things with the stuff here
        }
    })
}

pub fn ButtonGrid(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "container mx-auto justify-center flex flex-row",
            div {
                class: "container mx-auto flex flex-col justify-center",
                Entry {
                    entry: EntryObj {
                        to: Route::About {},
                        text: "About".to_string(),
                    }
                }
                Entry {
                    entry: EntryObj {
                        to: Route::Blog {},
                        text: "Blog".to_string(),
                    }
                
                }
                Entry {
                    entry: EntryObj {
                        to: Route::Tmp {},
                        text: "Projects".to_string(),
                    }
                
                }
            }
            div {
                class: "container mx-auto flex flex-col justify-center",
                Entry {
                    entry: EntryObj {
                        to: Route::Contact {},
                        text: "Contact".to_string(),
                    }
                }
                Entry {
                    entry: EntryObj {
                        to: Route::Tmp {},
                        text: "Resume".to_string(),
                    }
                
                }
                Entry {
                    entry: EntryObj {
                        to: Route::Tmp {},
                        text: "Links".to_string(),
                    }
                
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
        Link {
            to: entry.to.clone(),
            div {
                class: "p-2",
                div {
                    class: "flex text-rosewater p-1 text-4md hover:text-lavender cursor-pointer bg-surface0 justify-center borders border-lavender border-2 rounded-lg p-1",
                    
                    format!("{}", entry.text.clone())
                }
            }
        }
    })
}