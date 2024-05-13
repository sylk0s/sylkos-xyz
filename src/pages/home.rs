use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::*;
use serde::{Serialize, Deserialize};

const SCRIPT_CONTENT: &str = r#"tsParticles.load({
    id: "tsparticles",
    options: {
        background: {
            color: 'rgb(26,27,28)',
        },
        particles: {
            number: {
                value: 350,
            },
            move: {
                direction: 0,
                enable: true,
                outModes: {
                default: 0,
                },
                random: true,
                speed: 0.1,
                straight: true,
            },
            opacity: {
                animation: {
                enable: true,
                speed: 1,
                sync: false,
                },
                value: { min: 0, max: 1 },
            },
            size: {
                value: { min: 1, max: 3 },
            },
        },
        preset: "stars",
    },
});"#;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            id: "tsparticles",
            script {
                SCRIPT_CONTENT
            }
        }

        div {
            display: "flex",
            justify_content: "center",
            align_items: "center",

            position: "absolute",
            top: "0",
            left: "0",
            height: "100%",
            width: "100%",

            Index {
            
            }
        }
    })
}

pub fn Index(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex flex-col justify-center items-center bg-rosewater bg-opacity-20 rounded-lg p-1",

            hr {}
            h1 {
                class: "text-4xl text-center text-rosewater p-4",
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
            // Entry {
            //     entry: EntryObj {
            //         to: Route::Stars {},
            //         text: "Stars.".to_string(),
            //     }
            // }
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
            class: "flex text-lavender p-2 text-4md hover:text-rosewater cursor-pointer",
            Link {
                to: entry.to.clone(),
                format!("~ {}", entry.text.clone())
            }
        }
    })
}