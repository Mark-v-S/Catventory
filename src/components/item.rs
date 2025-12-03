use dioxus::prelude::*;

use crate::{
    backend::server_funktions::get_single_item,
    components::{change::Change, show::Show},
    Item,
};

#[component]
pub fn Itemm(id: i64) -> Element {
    let list_signal = use_context::<Signal<Vec<Item>>>();
    let mut list = use_signal(|| vec![]);
    let editmode = use_signal(|| false);
    let navigator = use_navigator();

    let _ = use_resource(move || async move {
        match get_single_item(id).await {
            Ok(item) => list.set(vec![item]),
            Err(_) => (),
        }
    });

    let is_edit = *editmode.read();

    if list.read().len() != 0 {
        rsx!(
            div { class: "my-5 flex justify-center",
                div { class: "border-solid border-sky-100 border-2 rounded p-1",
                    if !is_edit {
                        div { Show { list_signal, id , list, editmode} }
                    }
                    else if is_edit {
                        div { Change { list_signal, id , list, editmode} }
                    }
                }
            }
        )
    } else {
        rsx!(
            div {
                class: "my-5 flex justify-center",
                div { class: "text-5xl", "Item id : {id} Not Found!" }
                button { class: "bg-sky-50 text-purple-800 rounded p-1 hover:bg-sky-100",
                    onclick: move |_| navigator.go_back(),
                    "go back"
                }
            }
        )
    }
}
