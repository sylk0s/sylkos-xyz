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
            class: "text-xl rounded-lg bg-base p-2 text-rosewater",
            {children}
        }
    }
}