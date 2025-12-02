use dioxus::prelude::*;

use crate::{
    backend::server_funktions::{get_single_item, update_item_quantity},
    Item,
};

#[component]
pub fn Itemm(list_signal: Signal<Vec<Item>>, id: i64) -> Element {
    let mut list = use_signal(|| vec![]);
    let mut editmode = use_signal(|| false);
    let mut newvalue = use_signal(|| String::new());
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
                    }
                    else if is_edit {
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
                                //onclick: move |_| update_item_quantity(id, (newvalue.read().clone)),
                                onclick: move |_| async move {
                                    let mass: i64;
                                    mass = (*newvalue.read()).clone().parse().unwrap();
                                    match update_item_quantity(list.read()[0].id, (*newvalue.read()).clone()).await {
                                        Ok(_) => {
                                            list_signal.remove(list.read()[0].id.try_into().unwrap());
                                            let item = Item {
                                                id: list.read()[0].id,
                                                name: list.read()[0].name.clone(),
                                                mass: mass,
                                                unit: list.read()[0].unit.clone(),
                                                experation: list.read()[0].experation.clone(),
                                            };
                                            list_signal.push(item);
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
