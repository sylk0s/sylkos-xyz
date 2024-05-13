#![allow(non_snake_case)]

pub mod pages;
pub mod components;

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Serialize, Deserialize};

use pages::{home::*, about::*, blog::*, testing::*, *};

#[derive(Routable, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Route {
	#[route("/")]
	Home {},
	#[route("/about")]
	About {},
    #[route("/cat")]
    Cat {},
    #[route("/projects")]
    Projects {},
    #[route("/wip")]
    Wip {},
    #[route("/celeste")]
    Celeste {},
    #[route("/blog")]
    Blog {},
    #[route("/testing")]
    Testing {},
	#[route("/:..route")]
    NotFound { route: Vec<String> },
}