use dioxus::prelude::*;

use crate::components::remove::Remove;
use crate::{Item, Route};

#[component]
pub fn EList() -> Element {
    let list = use_context::<Signal<Vec<Item>>>();
    if list.read().len() == 0 {
        rsx!(
            div { class: "text-center text-xl", "No Items" }
        )
    } else {
        rsx!(
            //Add {},
            div { class: "h-screen w-full flex justify-center",
                ul { class: "sm:w-1/2 w-5/6",
                    {
                        list.read().iter().map(|sitem| {
                            let id = sitem.id;
                            //let name = &sitem.name;
                            //let mass = sitem.mass;
                            //let unit = &sitem.unit;
                            //let experation = &sitem.experation;
                            rsx!(
                                li {
                                    class: "mb-2 flex justify-between item-center bg-green-400 text-purple-800 px-2 py-2 rounded hover:bg-sky-200",
                                    key : "{id}",
                                    class: if sitem.mass <= 0 { "bg-red-200" },
                                    div { class: "flex" ,
                                        Link {to: Route::Itemm { id }, "{sitem.name.clone()}"," | ", "{sitem.mass.clone()}", "{sitem.unit.clone()}"," | ", "{sitem.experation.clone()}" }
                                    }
                                    div {
                                        Remove { list_signal: list, id }
                                    }
                                }
                            )
                        })
                    }
                }
            }
        )
    }
}
