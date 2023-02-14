#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use sajoopalja_lib::{ohaeng, jiji};

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch(App);
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let ohaeng = sajoopalja_lib::ohaeng::create_ohaeng();
    let jiji = sajoopalja_lib::jiji::create_jiji(&ohaeng);
    cx.render(rsx! {
        div {
            style {
                include_str!("./style.css")
            }
            header {
            }
            main {
                div {
                    ohaeng.iter().map(|(k, v)| {
                        rsx!(div { 
                            class: "haeng",
                            background_color: "{v.as_ref().borrow().color}",
                            "{v.as_ref().borrow().character}"})
                    })
                }
                div {
                    jiji.iter().map(|(k, v)| {
                        rsx!(div {
                            class: "ja",
                            "{v.as_ref().borrow().character}"})
                    })
                }
            }
        }
    })
}
