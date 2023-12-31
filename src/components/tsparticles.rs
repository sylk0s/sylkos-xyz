use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
struct Settings {
    id: String,
    options: Options,
}

#[derive(Default, Serialize, Deserialize)]
struct Options {
    preset: String,
}

#[wasm_bindgen(raw_module = "/node_modules/@tsparticles/engine/esm/index.js")]
extern "C" {
    type Engine;
    type DomItem;

    static tsParticles: Engine;

    #[wasm_bindgen(method)]
    pub fn load(this: &Engine, settings: &JsValue);

    #[wasm_bindgen(method)]
    pub fn domItem(this: &Engine, index: u32) -> DomItem;

    #[wasm_bindgen(method)]
    pub fn play(this: &DomItem);
}

pub fn Stars(cx: Scope) -> Element {

   let element = cx.render(rsx! {
        div {
            id: "tsparticles",
            position: "absolute",
            top: "0",
            left: "0",
            height: "100%",
            width: "100%",
        }

        script {
            src: "https://unpkg.com/browse/@tsparticles/engine@3.0.3/esm/index.js"
        }
    });

    // let engine = init();
    let settings = Settings {
        id: "tsparticles".to_string(),
        options: Options {
            preset: "stars".to_string(),
        },
    };
    let settings_js = serde_wasm_bindgen::to_value(&settings).unwrap();
    tsParticles.load(&settings_js);

    let particles = tsParticles.domItem(0);
    particles.play();

    // // Use eval returns a function that can spawn eval instances
    // let create_eval = use_eval(cx);

    // // You can create as many eval instances as you want
    // let eval = create_eval(
    //     r#"
    //     const tsParticles = require("@tsparticles/engine")
    //     tsParticles.load({
    //         id: "tsparticles",
    //         options: {
    //             preset: "stars"
    //         },
    //     });
    //     const particles = tsParticles.domItem(0);
    //     particles.play();
    //     "#,
    // )
    // .unwrap();

    // let future = use_future(cx, (), |_| {
    //     to_owned![eval];
    //     async move {
    //         // You can receive any message from JavaScript with the recv method
    //         eval.recv().await.unwrap()
    //     }
    // });

    // let _ = future.value();

    element
}