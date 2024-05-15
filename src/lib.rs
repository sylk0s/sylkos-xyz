#![allow(non_snake_case)]

pub mod pages;
pub mod components;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Serialize, Deserialize};

use pages::{home::*, about::*, blog::*, *};
use components::document::Document;

#[derive(Routable, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Route {
	#[route("/")]
	Home {},

	#[route("/about")]
	About {},
    // #[route("/projects")]
    // Projects {},
    #[route("/blog")]
    Blog {},
    // #[route("/contact")]
    // Contact {},
    // #[route("/links")]
    // Links {},
    // #[route("/resume")]
    // Resume {},

    #[route("/blog/:id")]
    BlogPost { 
        id: u32 
    },

    #[route("/testing")]
    Document {},
	#[route("/:..route")]
    Cat { route: Vec<String> },
}