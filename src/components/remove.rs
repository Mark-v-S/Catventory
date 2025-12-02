use dioxus::prelude::*;

use crate::{backend::server_funktions::remove_item, Item};

#[component]
pub fn Remove(list_signal: Signal<Vec<Item>>, id: i64) -> Element {
    rsx!(
      button {
        class: "text-slate-50 bg-rose-500 rounded p-1 ml-1 hover:bg-rose-600",
        onclick: move |_| async move {
            match remove_item(id).await {
                Ok(_) => {
                    let new_list = list_signal
                        .read()
                        .iter()
                        .filter(|item| item.id != id)
                        .map(|item| item.clone())
                        .collect::<Vec<Item>>();
                    list_signal.set(new_list);
                }
                Err(_) => {}
            }
        },
        "remove"
      }
    )
}
