use dioxus::prelude::*;

use crate::{backend::server_funktions::add_new_item, Item};

#[component]
pub fn Add() -> Element {
    let mut content_name = use_signal(|| String::new());
    let mut content_mass = use_signal(|| String::new());
    let mut content_unit = use_signal(|| String::new());
    let mut content_date = use_signal(|| String::new());
    let mut list_signal = use_context::<Signal<Vec<Item>>>();

    rsx!(
      div { class: "flex w-full justify-center my-2",
        div { class: "sm:w-1/6 w-5/6",
            ul {
                class: "mb-2 flex justify-center text-purple-200 py-2 rounded",
                li {
                    class: "w-1/1 text-purple-200 py-2 rounded",
                    div{
                        h1 { "Produkts" },
                    },
                    div {
                        select {
                            class: "p-1 rounded border-2 border-slate-100 border-solid",
                            onchange: move |e| content_name.set(e.value()),
                            option { disabled: true, "Choose Produckt" },
                            for item in list_signal.read().iter() {
                                option { "{item.name}" }
                            }
                        }
                    }
                    div {
                        input {
                            class: "w-1/1 p-1 rounded border-2 border-slate-100 border-solid",
                            r#type: "text",
                            value: content_name,
                            oninput: move |e| content_name.set(e.value()),
                        }
                    }
                },
                li {
                    class: "w-1/1 text-purple-200 py-2 rounded",
                    div{
                        h1 { "Name" },
                    }
                    div {
                        input {
                            class: "p-1 rounded border-2 border-slate-100 border-solid",
                            r#type: "text",
                            value: content_name,
                            oninput: move |e| content_name.set(e.value()),
                        },
                    }
                },
                li {
                    class: "w-1/1 text-purple-200 py-2 rounded",
                    //class: "p-1 rounded border-2 border-slate-100 border-solid",
                    div{
                        h1 { "Mass" },
                    }
                    div {
                        input {
                            class: "p-1 rounded border-2 border-slate-100 border-solid",
                            r#type: "text",
                            value: content_mass,
                            oninput: move |e| content_mass.set(e.value()),
                        },
                    }
                },
                li {
                    class: "w-1/1 text-purple-200 py-2 rounded",
                    div{
                        h1 { "Unit" },
                    },
                    div {
                        input {
                            class: "p-1 rounded border-2 border-slate-100 border-solid",
                            r#type: "text",
                            value: content_unit,
                            oninput: move |e| content_unit.set(e.value()),
                        },
                    }
                },
                li {
                    class: "w-1/1 text-purple-200 py-2 rounded",
                    div{
                        h1 { "Date" },
                    }
                    input {
                      class: "p-1 rounded border-2 border-slate-100 border-solid",
                      r#type: "text",
                      value: content_date,
                      oninput: move |e| content_date.set(e.value()),
                    },
                },
                li {
                    class: "w-1/1 text-purple-200 py-2 rounded",
                    div{
                        h1 { "button" },
                    }
                    div {
                        button {
                          class: "w-1/1 text-slate-50 bg-sky-500 p-1 rounded hover:bg-sky-600",
                          class: "p-1 rounded border-2 border-sky-500 border-solid",
                          //class: " p-1 rounded border-2 border-slate-100 border-solid",
                          onclick: move |_| async move {
                              let mass: i64;
                              mass = (*content_mass.read()).clone().parse().unwrap();
                              match add_new_item((*content_name.read()).clone(), mass, (*content_unit.read()).clone(), (*content_date.read()).clone(),).await {
                                  Ok(id) => {
                                      let item = Item {
                                          id,
                                          name: (*content_name.read()).clone(),
                                          mass: mass,
                                          unit: (*content_unit.read()).clone(),
                                          experation: (*content_date.read()).clone(),
                                      };
                                      let mut new_list = list_signal.read().clone();
                                      new_list.push(item);
                                      list_signal.set(new_list);
                                  }
                                  Err(_) => {}
                              }
                              content_name.set(String::new());
                              content_mass.set(String::new());
                              content_unit.set(String::new());
                              content_date.set(String::new());
                          },
                          disabled: if content_name.to_string().trim() == "" { true } else { false },
                          "add"
                        }
                    }
                },
                /*
                div{
                    class: "w-1/5",
                    h1 { "Unit" },
                },
                div{
                    class: "w-1/5",
                    h1 { "Date" },
                },
                div{
                    class: "w-1/4",
                    h1 { "" },
                },
                */
            }
            /*
            li {
                class: "mb-2 flex justify-between item-center text-purple-200 px-2 py-2 rounded",
                /*
                input {
                  class: "w-1/5 p-1 rounded border-2 border-slate-100 border-solid",
                  r#type: "text",
                  value: content_name,
                  oninput: move |e| content_name.set(e.value()),
                },
                */
                //select {
                //class: "w-1/5 p-1 rounded border-2 border-slate-100 border-solid",
                //onchange: move |e| content_name.set(e.value()),
                //option { disabled: true, "Choose Produckt" },
                //option { "Brot" }
                //option { "Mehl" }
                //option { "Zucker" }
                //}
                /*
                li {
                    div { "test1" },
                    div { "test2" }
                }
                select {
                    class: "w-1/5 p-1 rounded border-2 border-slate-100 border-solid",
                    onchange: move |e| content_name.set(e.value()),
                    option { disabled: true, "Choose Produckt" },
                    for item in list_signal.read().iter() {
                        option { "{item.name}" }
                    }
                    option {
                            input {
                              class: "w-1/5 p-1 rounded border-2 border-slate-100 border-solid",
                              r#type: "text",
                              value: content_name,
                              oninput: move |e| content_name.set(e.value()),
                        }
                    }
                }
                input {
                  class: "w-1/5 p-1 rounded border-2 border-slate-100 border-solid",
                  r#type: "text",
                  value: content_mass,
                  oninput: move |e| content_mass.set(e.value()),
                },
                input {
                  class: "w-1/5 p-1 rounded border-2 border-slate-100 border-solid",
                  r#type: "text",
                  value: content_unit,
                  oninput: move |e| content_unit.set(e.value()),
                },
                input {
                  class: "w-1/5 p-1 rounded border-2 border-slate-100 border-solid",
                  r#type: "text",
                  value: content_date,
                  oninput: move |e| content_date.set(e.value()),
                }
                */
                button {
                  class: "text-slate-50 bg-sky-500 p-1 rounded w-1/4 hover:bg-sky-600",
                  onclick: move |_| async move {
                      let mass: i64;
                      mass = (*content_mass.read()).clone().parse().unwrap();
                      match add_new_item((*content_name.read()).clone(), mass, (*content_unit.read()).clone(), (*content_date.read()).clone(),).await {
                          Ok(id) => {
                              let item = Item {
                                  id,
                                  name: (*content_name.read()).clone(),
                                  mass: mass,
                                  unit: (*content_unit.read()).clone(),
                                  experation: (*content_date.read()).clone(),
                              };
                              let mut new_list = list_signal.read().clone();
                              new_list.push(item);
                              list_signal.set(new_list);
                          }
                          Err(_) => {}
                      }
                      content_name.set(String::new());
                      content_mass.set(String::new());
                      content_unit.set(String::new());
                      content_date.set(String::new());
                  },
                  disabled: if content_name.to_string().trim() == "" { true } else { false },
                  "add"
                }
            }
            */
        }
      }
    )
}
