use dioxus::prelude::*;

const SCRIPT_CONTENT: &str = r#"tsParticles.load({
    id: "tsparticles",
    options: {
        background: {
            color: 'rgb(30, 30, 46)',
        },
        particles: {
            number: {
                value: 350,
            },
            move: {
                direction: "random",
                enable: true,
                outModes: {
                default: 0,
                },
                random: true,
                speed: .1,
                straight: true,
            },
            opacity: {
                animation: {
                enable: true,
                speed: 1,
                sync: false,
                },
                value: { min: 0, max: 1 },
            },
            size: {
                value: { min: 1, max: 3 },
            },
        },
        preset: "stars",
    },
});"#;

pub fn Stars(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            id: "tsparticles",
            script {
                SCRIPT_CONTENT
            }
        }
    })
}