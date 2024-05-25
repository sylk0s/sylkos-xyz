use dioxus::prelude::*;
use crate::{
    Route,
    components::stars::Stars,
};
use dioxus_router::prelude::*;

#[component]
pub fn Document(children: ReadOnlySignal<Element>) -> Element {
    rsx! {
        Stars {

        }
        div { // bg
            class: "absolute flex h-screen w-screen justify-center",

            div { // center text column
                    class: "container mx-auto p-2 max-w-4xl",
                    
                    div { // header padding
                        class: "pb-4",
                        div { // header bar
                            class: "flex bg-base rounded-lg p-2",
                            
                            // header content

                            h1 {
                                class: "text-xl text-left text-rosewater",
                                Link {
                                    to: Route::Home {},
                                    "⇦ Return Home"
                                }
                            }
                        }
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