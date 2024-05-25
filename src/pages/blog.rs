use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::Route;
use dioxus_router::prelude::*;
use crate::components::{
    stars::Stars, 
    document::Document,
    markdown::Markdown,
};
use chrono::prelude::*;
use std::{
    fs,
    path::Path,
};
use lazy_static::lazy_static;

lazy_static! {
    //static ref SERIES_CONFIGS: Vec<BlogConfig> = read_blog_config();
    //static ref ENTRIES: Vec<EntryObj> = initialize_blog_entries(SERIES_CONFIG);
}

#[derive(Serialize, Deserialize)]
struct BlogConfig {
    id: u32,
    title: String,
    content: Vec<BlogPageConfig>,
}

#[derive(Serialize, Deserialize)]
struct BlogPageConfig {
    id: u32,
    subname: String,
    date: DateTime<Local>,
    description: String,
    content: String,
}

// read the blog config from a file
fn read_blog_config() -> Vec<BlogConfig> {
    // for each folder in the blog folder
    let entries = fs::read_dir(Path::new("../../public/blog")).expect("Could not find blog dir");
    
    entries.filter_map(|ent| {
        let entry = ent.expect("unwrapping entry");
        if entry.metadata().expect("metadata").is_dir() {
            let mut sub = fs::read_dir(entry.path()).expect("failed to read subfolder");
            // read the .toml file in the folder
            if let Some(config_file) = sub.find(|e| e.as_ref().expect("unwrap entry").file_name() == "blog.toml") {
                let config_str = fs::read_to_string(config_file.expect("unwrapping config file entry").path()).expect("Failed to read toml");
                let config: BlogConfig = toml::from_str(&config_str).expect("Failed to parse toml");
                return Some(config);
            }
        }
        None
    }).collect()
}

fn initialize_blog_entries(configs: Vec<BlogConfig>) -> Vec<EntryObj> {
    // for page_config
    unimplemented!();
}

#[derive(Clone, PartialEq, Debug, Copy)]
struct EntryObj {
    id: u32,
    title: &'static str,
    date: &'static str,
    description: &'static str,
    content: &'static str,
}

const ENTRIES: [EntryObj; 3] = [
    EntryObj {
        id: 0,
        title: "First Post",
        date: "2021-09-01",
        description: "This is the first post",
        content: "This is the first post",
    },
    EntryObj {
        id: 1,
        title: "Second Post",
        date: "2021-09-02",
        description: "This is the second post",
        content: "This is the second post",
    },
    EntryObj {
        id: 2,
        title: "Third Post",
        date: "2021-09-03",
        description: "This is the third post",
        content: "This is the third post",
    },
];

pub fn Blog() -> Element {
    rsx! {
        Stars {}
        div { // background of everything
            class: "absolute flex h-screen w-screen justify-center",
            div { // center text column
                    class: "container mx-auto max-w-4xl",

                h1 {
                    class: "text-4xl text-left text-rosewater p-4",
                    "Blog"
                }

                hr {}

                div {
                    class: "flex flex-col p-4",
                    for &entry in ENTRIES.iter() {
                        Entry { entry: entry }
                    }
                }

                hr {}
            }
        }
    }
}

#[component]
fn Entry(entry: EntryObj) -> Element {
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

    rsx!{
        Document {

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