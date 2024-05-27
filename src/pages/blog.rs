use dioxus::prelude::*;
use crate::Route;
use crate::components::{
    stars::Stars,
    markdown::Markdown,
    topbar::TopBar,
};

#[derive(Clone, PartialEq, Debug)]
struct Entry {
    id: u32,
    title: &'static str,
    date: &'static str,
    description: &'static str,
    content: &'static str,
}

// this... is a more boring to solve this, but i can't quite get the blog entries to read in properly on compile time. the wasm seems like it *can't* do this.
const ENTRIES: [Entry; 2] = [
    Entry {
        id: 0,
        title: "I made a website!",
        date: "2024-06-01",
        description: "All about this website you're on Right Now!!!",
        content: include_str!("../../assets/blog/website-meta.md"),
    },
    Entry {
        id: 1,
        title: "Why I use NixOS",
        date: "2024-06-01",
        description: "TLDR: I's... pretty good?",
        content: include_str!("../../assets/blog/nixos-why.md"),
    },
];

// all of the stuff below is for the auto blog configs... i gave up on that bc it wasn't working well in wasm

// use std::{
//     fs,
//     path::Path,
// };
// use lazy_static::lazy_static;
// use serde::{Deserialize, Serialize};
// use toml::{value::Date, Value};

// static mut ENTRIES: Vec<EntryObj> = Vec::new();
// static mut BLOG_CONFIG: Vec<BlogConfig> = Vec::new();

// #[derive(Serialize, Deserialize, Clone)]
// pub struct BlogConfig {
//     id: u32,
//     title: String,
//     content: Vec<BlogPageConfig>,
// }

// #[derive(Serialize, Deserialize, Clone)]
// struct BlogPageConfig {
//     id: u32,
//     subname: String,
//     date: DateTime<Local>,
//     description: String,
//     content: String,
// }

// read the blog config from a file
// pub fn read_blog_config() -> Vec<BlogConfig> {
//     // for each folder in the blog folder
//     let entries = fs::read_dir(Path::new("public/blog")).expect("Could not find blog dir");
    
//     entries.filter_map(|ent| {
//         let entry = ent.expect("unwrapping entry");
//         if entry.metadata().expect("metadata").is_dir() {
//             let mut sub = fs::read_dir(entry.path()).expect("failed to read subfolder");
//             // read the .toml file in the folder
//             if let Some(config_file) = sub.find(|e| e.as_ref().expect("unwrap entry").file_name() == "blog.toml") {
//                 let config_str = fs::read_to_string(config_file.expect("unwrapping config file entry").path()).expect("Failed to read toml");
//                 //let config: BlogConfig = toml::from_str(&config_str).expect("Failed to parse toml");
//                 let config: Value = toml::from_str(&config_str).expect("Failed to parse toml");
//                 return Some(BlogConfig {
//                     id: config["id"].as_integer().expect("Failed to parse id") as u32,
//                     title: config["title"].as_str().expect("Failed to parse title").to_string(),
//                     content: config["content"].as_array().expect("Failed to parse content").iter().map(|page| {
//                         BlogPageConfig {
//                             id: page["id"].as_integer().expect("Failed to parse id") as u32,
//                             subname: page["subname"].as_str().expect("Failed to parse subname").to_string(),
//                             date: chrono::Local::now(), //DateTime::parse_from_str(page["date"].as_str().expect("failed finding date"), "%Y-%m-%d").expect("Failed to parse date").into(),
//                             description: page["description"].as_str().expect("Failed to parse description").to_string(),
//                             content: page["content"].as_str().expect("Failed to parse content").to_string(),
//                         }
//                     }).collect(),
//                 });
//                 // return Some(config);
//             }
//         }
//         None
//     }).collect()
// }

// pub fn initialize_blog_entries(configs: Vec<BlogConfig>) -> Vec<EntryObj> {
//     // for page_config
//     configs.iter().flat_map(|config| {
//         config.content.iter().map(move |page_config| {

//             // read the content in
//             let content = fs::read_to_string(format!("public/blog/{}/{}", config.id, page_config.content)).expect("Failed to read content");

//             EntryObj {
//                 id: page_config.id,
//                 title: format!("{}: {}", config.title, page_config.subname),
//                 date: page_config.date.to_string(),
//                 description: page_config.description.clone(),
//                 content,
//             }
//         })
//     }).collect()
// }

pub fn Blog() -> Element {
    rsx! {
        Stars {}
        div { // background of everything
            class: "absolute flex h-screen w-screen justify-center",
            div { // center text column
                    class: "container mx-auto p-2 max-w-4xl",
                
                TopBar {
                    children: { vec![ 
                        rsx! { Link {
                            to: Route::Home {},
                            "🏠 Home"
                        }},
                    ]}
                }

                h1 {
                    class: "text-4xl text-left text-rosewater p-4",
                    "Blog"
                }

                hr {}

                div {
                    class: "flex flex-col p-4",
                    for entry in ENTRIES.iter().rev() {
                        EntryDisp { entry: entry.clone() }
                    }
                }

                hr {}
            }
        }
    }
}

#[component]
fn EntryDisp(entry: Entry) -> Element {
    rsx! {
        Link {
            to: Route::BlogPost { id: entry.id },
            div {
                class: "flex p-2",
                div {
                    class: "container bg-base rounded-lg borders border-rosewater border-2",

                    div {
                        class: "flex flex-col p-2",
                        h2 {
                            class: "text-3xl text-left text-rosewater p-1",
                            {entry.title},
                        }

                        p {
                            class: "text-left text-overlay0 ",
                            {entry.date},
                        }
                    }

                    p {
                        class: "text-left text-rosewater p-2",
                        {entry.description},
                    }
                }
            }
        }
    }
}

#[component]
pub fn BlogPost(id: u32) -> Element {
    let entry = ENTRIES.iter().find(|e| id == e.id).unwrap();

    rsx! {
        Stars {

        }
        div { // bg
            class: "absolute flex h-screen w-screen justify-center",

            div { // center text column
                class: "container mx-auto p-2 max-w-4xl",
                
                TopBar {
                    children: { vec![ 
                        rsx! { Link {
                            to: Route::Home {},
                            "🏠 Home"
                        }},
                        rsx! { Link {
                            to: Route::Blog {},
                            "🖊️ Blog"
                        }},
                    ]}
                }

                div {
                    class: "flex flex-col bg-base rounded-lg p-2",

                    // document content
                    div {
                        class: "flex flex-col p-2",
                        h2 {
                            class: "text-3xl text-left text-rosewater p-1",
                            {entry.title}
                        }
        
                        p {
                            class: "text-left text-overlay0 ",
                            {entry.date}
                        }
        
                        hr {}
                    }
        
                    Markdown {
                        content: entry.content
                    }
                }
            }
        }
    }
}