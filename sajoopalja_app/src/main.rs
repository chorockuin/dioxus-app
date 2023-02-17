#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use sajoopalja_lib::{umyang, ohaeng, cheongan, jiji};

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch(App);
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    let ohaeng = sajoopalja_lib::ohaeng::create_ohaeng();
    let cheongan = sajoopalja_lib::cheongan::create_cheongan();
    let jiji = sajoopalja_lib::jiji::create_jiji();
    cx.render(rsx! {
        div {
            style {
                include_str!("./style.css")
            }
            header {
            }
            main {
                div {
                    ohaeng.iter().map(|haeng| {
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
                    cheongan.iter().map(|cheonganja| {
                        let haeng = sajoopalja_lib::ohaeng::get_haeng(&ohaeng, cheonganja.haeng_name).unwrap();
                        let color = if cheonganja.umyang_name == sajoopalja_lib::umyang::Name::Um {"blue"} else {"red"};
                        rsx!(div {
                            class: "cheonganja_box",
                            div {
                                class: "cheonganja",
                                background_color: "{haeng.color}",
                                color: "{color}",
                                "{cheonganja.character}"
                            }
                        })
                    })
                }
                div {
                    clear: "left"
                }
                div {
                    jiji.iter().map(|jijija| {
                        let haeng = sajoopalja_lib::ohaeng::get_haeng(&ohaeng, jijija.haeng_name).unwrap();
                        let color = if jijija.umyang_name == sajoopalja_lib::umyang::Name::Um {"blue"} else {"red"};
                        rsx!(div {
                            class: "jijija_box",
                            div {
                                class: "jijiwol",
                                "{jijija.wol}",
                            }
                            div {
                                class: "jijija",
                                background_color: "{haeng.color}",
                                color: "{color}",
                                "{jijija.character}"
                            }
                        })
                    })
                }
            }
        }
    })
}
