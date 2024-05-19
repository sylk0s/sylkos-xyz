use dioxus::prelude::*;
use crate::Route;
use dioxus_router::prelude::*;
use crate::components::{
    stars::Stars, 
    document::Document,
    markdown::Markdown,
};


#[derive(Clone, PartialEq, Debug, Copy)]
struct EntryObj {
    id: u32,
    title: &'static str,
    date: &'static str,
    description: &'static str,
    content: &'static str,
}

#[derive(Props, Clone, PartialEq, Debug, Copy)]
struct BlogProps<'a> {
    entries: &'a Vec<EntryObj>,
}

#[derive(Props, Clone, PartialEq, Debug, Copy)]
struct EntryProps {
    entry: EntryObj,
}

const ENTRIES: [EntryObj; 3] = [
    EntryObj {
        id: 0,
        title: "Entry Title",
        date: "01/01/1970",
        description: "some filler text...",
        content: include_str!("../../blog.md")
    },
    EntryObj {
        id: 1,
        title: "AAA",
        date: "01/01/1971",
        description: "some filler text...",
        content: include_str!("../../blog.md")
    },
    EntryObj {
        id: 2,
        title: "ZZZ",
        date: "01/01/1972",
        description: "some filler text...",
        content: include_str!("../../blog.md")
    }
];

pub fn Blog<'a>(cx: Scope) -> Element {
    cx.render(rsx! {
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
    })
}

fn Entry(cx: Scope<EntryProps>) -> Element {
    cx.render(rsx! {
        Link {
            to: Route::BlogPost { id: cx.props.entry.id },
            div {
                class: "flex p-2",
                div {
                    class: "container bg-base rounded-lg borders border-rosewater border-2",

                    div {
                        class: "flex flex-col p-2",
                        h2 {
                            class: "text-3xl text-left text-rosewater p-1",
                            cx.props.entry.title
                        }

                        p {
                            class: "text-left text-overlay0 ",
                            cx.props.entry.date
                        }
                    }

                    p {
                        class: "text-left text-rosewater p-2",
                        cx.props.entry.description
                    }
                }
            }
        }
    })
}

#[derive(Props, Clone, PartialEq, Debug, Copy)]
pub struct PostProps {
    id: u32,
}

pub fn BlogPost(cx: Scope<PostProps>) -> Element {
    let entry = ENTRIES.iter().find(|e| e.id == cx.props.id).unwrap();

    cx.render(rsx!{
        Document {
            script {
                "hljs.highlightAll();"
            }

            div {
                class: "flex flex-col p-2",
                h2 {
                    class: "text-3xl text-left text-rosewater p-1",
                    entry.title
                }

                p {
                    class: "text-left text-overlay0 ",
                    entry.date
                }

                hr {}
            }

            Markdown {
                content: entry.content
            }
        }
    })
}