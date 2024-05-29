use dioxus::prelude::*;

#[component]
pub fn TopBar(children: Vec<Element>) -> Element {
    rsx! {
        div { // header padding
            class: "pb-4",
            div { // header bar
                class: "flex flex-row bg-base rounded-lg p-2",
                
                // header content

                for child in children {
                    ButtonEntry { children: child }
                }
            }
        }
    }
}

#[component]
fn ButtonEntry(children: Element) -> Element {
    rsx! {
        h1 {
            class: "text-md rounded-lg bg-base px-2 text-rosewater text-xl",
            {children}
        }
    }
}