use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{
    *, 
    components::{
        stars::Stars,
        transtext::{TransText, TransFlagText},
    },
};
use serde::{Serialize, Deserialize};
use dioxus_router::components::IntoRoutable;


pub fn Home() -> Element {
    rsx! {
        Stars {

        }

        div {
            class: "absolute flex h-screen w-screen justify-center items-center",

            Index {
            
            }
        }
    }
}

pub fn Index() -> Element {
    rsx! {
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
                        TransFlagText {
                            text: "they / she".to_string()
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
    }
}

pub fn ButtonGrid() -> Element {
    rsx! {
        div {
            class: "container mx-auto justify-center flex flex-row",
            div {
                class: "container mx-auto flex flex-col justify-center",
                Entry {
                    entry: EntryObj {
                        to: Routable::FromRoute(Route::About {}),
                        text: "About".to_string(),
                    }
                }
                Entry {
                    entry: EntryObj {
                        to: Routable::FromRoute(Route::Blog {}),
                        text: "Blog".to_string(),
                    }
                
                }
                Entry {
                    entry: EntryObj {
                        to: Routable::FromStr("https://github.com/sylk0s".to_string()),
                        text: "Projects".to_string(),
                    }
                
                }
            }
            div {
                class: "container mx-auto flex flex-col justify-center",
                Entry {
                    entry: EntryObj {
                        to: Routable::FromRoute(Route::Contact {}),
                        text: "Contact".to_string(),
                    }
                }
                Entry {
                    entry: EntryObj {
                        to: Routable::FromStr("https://sylkos.xyz/resume.pdf".to_string()),
                        text: "Resume".to_string(),
                    }
                }
                Entry {
                    entry: EntryObj {
                        to: Routable::FromRoute(Route::Tmp {}),
                        text: "Links".to_string(),
                    }
                
                }
            }
        }
    }
}

// I have to do this thing because for some reason god (rustc) really doesn't
// want me to use ImplIter bc apparently it's not partialeq or something
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
enum Routable {
    FromStr(String),
    FromRoute(Route),
}

fn from_routable(routable: Routable) -> IntoRoutable {
    match routable {
        Routable::FromStr(a) => IntoRoutable::FromStr(a),
        Routable::FromRoute(a) => a.into(),
    }
}

#[derive(Props, Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct EntryObj {
    to: Routable,
    #[serde(default)]
    text: String,
}

#[component]
fn Entry(entry: EntryObj) -> Element {
    rsx! {
        Link {
            to: from_routable(entry.to.clone()),
            div {
                class: "p-2",
                div {
                    class: "flex text-rosewater p-1 text-4md hover:text-lavender cursor-pointer bg-surface0 justify-center borders border-lavender border-2 rounded-lg p-1",
                    {format!("{}", entry.text.clone())}
                }
            }
        }
    }
}