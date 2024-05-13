use dioxus::prelude::*;

pub fn Blog(cx: Scope) -> Element {
    cx.render(rsx! {
        div { // background of everything
            class: "min-h-screen bg-base",
            div { // center text column
                    class: "sm:container sm:mx-auto",

                h1 {
                    class: "text-4xl text-left text-rosewater bg-base p-4",
                    "Blog"
                }

                hr {}

                div {
                    class: "p-4",
                    Entry {

                    }

                    Entry {

                    }
                }

                hr {}
            }
        }
    })
}

fn Entry(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex p-2",
            div {
                class: "flex flex-col bg-crust rounded-lg",

                div {
                    class: "flex flex-col p-2",
                    h2 {
                        class: "text-3xl text-left text-rosewater p-1",
                        "Entry Title"
                    }

                    p {
                        class: "text-left text-overlay0 ",
                        "01/01/1970"
                    }
                }

                p {
                    class: "text-left text-rosewater p-2",
                    "lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
                    "lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
                    "lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
                    "lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
                    "lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
                }
            }
        }
    })
}