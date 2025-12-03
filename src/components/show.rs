use dioxus::prelude::*;

use crate::{backend::server_funktions::get_single_item, Item};
#[component]
pub fn Show(
    list_signal: Signal<Vec<Item>>,
    id: i64,
    list: Signal<Vec<Item>>,
    editmode: Signal<bool>,
) -> Element {
    let navigator = use_navigator();

    let _ = use_resource(move || async move {
        match get_single_item(id).await {
            Ok(item) => list.set(vec![item]),
            Err(_) => (),
        }
    });
    rsx!(
        div { class: "text-3xl",
            li {
                class: "mb-1 flex item-center rounded",
                h1 { "ID :" },
                h1 { "{list.read()[0].id}" }
            }
            li {
                class: "mb-1 flex item-center rounded",
                h1 { "Name :" },
                h1 { "{list.read()[0].name}" }
            }
            li {
                class: "mb-1 flex item-center rounded",
                h1 { "Mass :" },
                h1 { "{list.read()[0].mass}" }
            }
            li {
                class: "mb-1 flex item-center rounded",
                h1 { "unit :" },
                h1 { "{list.read()[0].unit}" }
            }
            li {
                class: "mb-1 flex item-center rounded",
                h1 { "experation :" },
                h1 { "{list.read()[0].experation}" }
            }
        }
        li {
            class: "mb-2 flex justify-between item-center text-purple-800 px-2 py-2 rounded",
            button { class: "bg-sky-50 text-purple-800 rounded p-1 hover:bg-sky-100",
                onclick: move |_| editmode.set(true),
                "edit"
            }
            button { class: "bg-sky-50 text-purple-800 rounded p-1 hover:bg-sky-100",
                onclick: move |_| navigator.go_back(),
                "go back"
            }
        }
    )
}
