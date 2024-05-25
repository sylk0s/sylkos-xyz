#![allow(non_snake_case)]

pub mod pages;
pub mod components;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Serialize, Deserialize};

use pages::{home::*, blog::*, *};
#[derive(Routable, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Route {
	#[route("/")]
	Home {},

	#[route("/about")]
	About {},
    //#[route("/projects")]
    // Tmp {},
    #[route("/blog")]
    Blog {},
    #[route("/contact")]
    Contact {},
    // #[route("/links")]
    // Tmp {},
    // #[route("/resume")]
    // Tmp {},

    #[route("/tmp")]
    Tmp {},

    #[route("/blog/:id")]
    BlogPost { 
        id: u32 
    },

	#[route("/:..route")]
    Cat { route: Vec<String> },
}

pub fn Tmp() -> Element {
    rsx! {
        div {
            class: "h-screen w-screen bg-crust",
            h1 {
                class: "text-text p-8 text-xl",
                "This is a temporary page... the actual page is still under construction :)"
            }
        }
    }
}