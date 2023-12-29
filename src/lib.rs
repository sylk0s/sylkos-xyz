#![allow(non_snake_case)]

pub mod components;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Serialize, Deserialize};

use components::index::*;

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
	//  if the current location doesn't match any of the above routes, render the NotFound component
	#[route("/:..route")]
    NotFound { route: Vec<String> },
}

fn Home(cx: Scope) -> Element {
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
    
                background_color: "#181825",
                Index {
                    
                }
            }
        }
    })
}

fn About(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "About"
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
            "       |\\__/,|   (`\\" br {}
            "    _.|o o  |_   ) )  " br {}
            "    -(((---(((-------- \n" br {}
            }
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

fn Stars(cx: Scope) -> Element {

    // Use eval returns a function that can spawn eval instances
   // let create_eval = use_eval(cx);

    let element = cx.render(rsx! {
        div {
            id: "tsparticles",
            position: "absolute",
            top: "0",
            left: "0",
            height: "100%",
            width: "100%",
        }

        script {
            src: "tsparticles.engine.min.js"
        }
    });

    // stars::particles();

    element
}