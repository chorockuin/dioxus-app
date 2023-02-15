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
                        let haeng = v.as_ref().borrow();
                        rsx!(div {
                            class: "haeng_box",
                            div {
                                class: "haeng",
                                background_color: "{haeng.color}",
                                "{haeng.character}"
                            }
                        })
                    })
                }
                div {
                    clear: "left"
                }
                div {
                    jiji.iter().map(|(k, v)| {
                        let jijija = v.as_ref().borrow();
                        let haeng = sajoopalja_lib::ohaeng::get_haeng(&ohaeng, jijija.haeng_name);
                        rsx!(div {
                            class: "ja_box",
                            div {
                                class: "wol",
                                "{jijija.wol}",
                            }
                            div {
                                class: "ja",
                                background_color: "{haeng.color}",
                                "{jijija.character}"
                            }
                        })
                    })
                }
            }
        }
    })
}
