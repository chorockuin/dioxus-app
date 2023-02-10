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
            ohaeng.iter().map(|(k, v)| rsx!(li {"{v.as_ref().borrow().character}"}))
            jiji.iter().map(|(k, v)| rsx!(li {"{v.as_ref().borrow().character}"}))
        }
    })
}
