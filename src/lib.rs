#![allow(non_snake_case)]

pub mod components;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Serialize, Deserialize};

use components::{
    index::*,
    tsparticles::{Stars, *},
};

#[derive(Routable, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Route {
	#[route("/")]
	Home {},
	#[route("/about")]
	About {},
    #[route("/cat")]
    Cat {},
    #[route("/stars")]
    Stars {},
    #[route("/projects")]
    Projects {},
    #[route("/wip")]
    Wip {},
    #[route("/celeste")]
    Celeste {},
	#[route("/:..route")]
    NotFound { route: Vec<String> },
}

fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        link { rel: "stylesheet", href: "../dist/output.css" }
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

fn About(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            div {
                display: "flex",
                justify_content: "center",
                align_items: "center",

                position: "absolute",
                top: "0",
                left: "0",
                height: "100%",
                width: "100%",

                div {
                    display: "flex",
                    flex_direction: "column",

                    position: "absolute",
                    top: "5%",
                    left: "30%",
                    height: "90%",
                    width: "40%",

                    background_color: "#181825",
                    border: "4px solid #585b70",
                    border_radius: "10px",
                    font_family: "IBM Plex Mono",
                    font_size: "14px",

                    div {
                        display: "flex",
                        flex_direction: "column",
                        position: "relative",
                        width: "100%",
                        align_items: "center",

                        h1{ "Julia Keadey" }
                        h2{ "[Sylkos]" }
                    }

                    div {
                        align_items: "left",
                        padding: "10px",

                        h3 {"Links:"}
                        "Github ~> [LINK]" br{}
                        "Discord ~> [LINK]" br{}
                        "Email ~> [LINK]" br{}
                    }

                    div {
                        align_items: "left",
                        padding: "10px",

                        h3 {"About:"}
                        "I'm a 20 year old student currently studying Computer Science and Computer Engineering at Northeastern University." br{}

                        // h3 {"Interests:"}

                        // h3 {"Contact:"}


                    
                    }
                }
            }
        }
    })
}

#[component]
fn NotFound(cx: Scope, route: Vec<String>) -> Element {
    let sroute = route.join("/");
    cx.render(rsx! {
        div {
            "404 {sroute}"
        }
    })
}

fn Cat(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            white_space: "pre-wrap",
            h1 {
            "      |\\__/,|   (`\\" br {}
            "    _.|o o  |_   ) )  " br {}
            "    -(((---(((-------- \n" br {}
            }
        }
    })
}

fn Projects(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Projects"
        }
    })
}

fn Wip(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            padding: "10px",
            h1 { "This page is under construction... come back later!" }
        }
    })
}

fn Celeste(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "add the art thingy here :P"
        }
    })
}

// mod stars {
//     use wasm_bindgen::prelude::*;

//     #[wasm_bindgen(raw_module = "/node_modules/@tsparticles/engine/tsparticles.engine.js")]
//     extern "C" {
//         type Engine;
//         type domItem;
//         type Settings;

//         #[wasm_bindgen(method)]
//         pub fn load() {

//         }
//     }
// }
