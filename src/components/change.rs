use dioxus::prelude::*;

use crate::{
    backend::server_funktions::{get_single_item, update_item_quantity},
    components::get_index::get_index,
    Item,
};
#[component]
pub fn Change(
    list_signal: Signal<Vec<Item>>,
    id: i64,
    list: Signal<Vec<Item>>,
    editmode: Signal<bool>,
) -> Element {
    let mut newvalue = use_signal(|| String::new());

    let _ = use_resource(move || async move {
        match get_single_item(id).await {
            Ok(item) => list.set(vec![item]),
            Err(_) => (),
        }
    });
    rsx!(
        div { class: "text-3xl text-purple-800",
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
                input {
                    class: "",
                    type: "number",
                    value: "{list.read()[0].mass}",
                    oninput: move |e| newvalue.set(e.value()),
                }
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
                onclick: move |_| async move {
                    let mass: i64;
                    mass = (*newvalue.read()).clone().parse().unwrap();
                    let current_item = list.read()[0].clone();
                    match update_item_quantity(current_item.id, (*newvalue.read()).clone()).await {
                        Ok(_) => {
                            let index = get_index(list_signal, list.read()[0].id);
                            if index < list_signal.len() as i64{
                                let mut new_list = list_signal.read().clone();
                                let item = Item {
                                    id: list.read()[0].id,
                                    name: list.read()[0].name.clone(),
                                    mass: mass,
                                    unit: list.read()[0].unit.clone(),
                                    experation: list.read()[0].experation.clone(),
                                };
                                list.set(vec![item.clone()]);
                                new_list[index as usize] = item.clone();
                                list_signal.set(new_list);
                            }
                        }
                        Err(_) => {}
                    }
                    editmode.set(false)
                },
                "Safe"
            }
            button { class: "bg-sky-50 text-purple-800 rounded p-1 hover:bg-sky-100",
                onclick: move |_| editmode.set(false),
                "Cancel"
            }
        }
    )
}
