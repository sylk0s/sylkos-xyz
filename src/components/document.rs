use dioxus::prelude::*;
use crate::{
    Route,
    components::stars::Stars,
    components::topbar::TopBar,
};

#[component]
pub fn Document(children: ReadOnlySignal<Element>) -> Element {
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
                    ]}
                }

                div {
                    class: "flex flex-col bg-base rounded-lg p-2",

                    // document content

                    { children }
                }
            }
        }
    }
}